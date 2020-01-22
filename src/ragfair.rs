use crate::{
    handle_error, handle_error2, Error, ErrorResponse, Result, Tarkov, PROD_ENDPOINT,
    RAGFAIR_ENDPOINT,
};

use crate::inventory::{BarterItem, InventoryUpdate, Item, MoveItemRequest, RagfairResponseData};
use crate::market_filter::{Currency, MarketFilter, Owner, SortBy, SortDirection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Ragfair error
#[derive(Debug, err_derive::Error)]
pub enum RagfairError {
    /// Offer is not available yet.
    #[error(display = "offer not available yet")]
    OfferNotAvailableYet,
    /// Offer was not found.
    #[error(display = "offer not found")]
    OfferNotFound,
    /// Provided `BarterItem` is invalid, not enough quantities available or not found.
    #[error(display = "barter items provided cannot be found")]
    InvalidBarterItems,
    /// Maximum outstanding offer count of 3 was reached.
    #[error(display = "maximum offer count of 3 was reached")]
    MaxOfferCount,
    /// Insufficient funds to pay the flea market fee.
    #[error(display = "insufficient funds to pay market fee")]
    InsufficientTaxFunds,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchRequest<'a> {
    page: u64,
    limit: u64,
    sort_type: SortBy,
    sort_direction: SortDirection,
    currency: Currency,
    price_from: u64,
    price_to: u64,
    quantity_from: u64,
    quantity_to: u64,
    condition_from: u64,
    condition_to: u64,
    one_hour_expiration: bool,
    remove_bartering: bool,
    offer_owner_type: Owner,
    only_functional: bool,
    update_offer_count: bool,
    handbook_id: &'a str,
    linked_search_id: &'a str,
    needed_search_id: &'a str,
    //build_items: {}
    //build_count: u64,
    tm: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Option<SearchResult>,
}

/// Market search result
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    /// Market categories
    pub categories: HashMap<String, u64>,
    /// Available offers
    pub offers: Vec<Offer>,
    /// Number of items for sale
    pub offers_count: u64,
    /// Selected category ID
    pub selected_category: String,
}

/// Market offer
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Offer {
    /// Offer ID
    #[serde(rename = "_id")]
    pub id: String,
    /// ?
    pub int_id: String,
    /// Merchant profile
    pub user: User,
    /// ?
    pub root: String,
    /// Items for sale
    pub items: Vec<Item>,
    /// Items cost
    pub items_cost: u64,
    /// Items wanted in return
    pub requirements: Vec<Requirement>,
    /// Requirement items cost
    pub requirements_cost: u64,
    /// Summary cost
    pub summary_cost: u64,
    /// ?
    pub sell_in_one_piece: bool,
    /// Time when item was listed on the market.
    ///
    /// Add 60 seconds for the true start time, when the item will be available for purchase.
    pub start_time: u64,
    /// Offer expiry time
    pub end_time: u64,
    /// Merchant loyalty level
    pub loyalty_level: u64,
}

/// Merchant profile
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// Merchant ID
    pub id: String,
    /// Merchant member type
    pub member_type: u64,
    /// Merchant nickname
    pub nickname: Option<String>,
    /// Merchant rating
    pub rating: Option<f64>,
    /// Merchant rating is growing
    pub is_rating_growing: Option<bool>,
    /// Merchant avatar
    pub avatar: Option<String>,
}

/// Offer requirement
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Requirement {
    /// Item localization schema ID
    #[serde(rename = "_tpl")]
    pub schema_id: String,
    /// Item count
    pub count: f64,
}

#[derive(Debug, Serialize)]
struct BuyItemRequest<'a> {
    #[serde(rename = "Action")]
    action: &'a str,
    offers: &'a [BuyOffer<'a>],
}

#[derive(Debug, Serialize)]
struct BuyOffer<'a> {
    id: &'a str,
    count: u64,
    items: &'a [BarterItem],
}

#[derive(Debug, Serialize)]
struct BuyItem {
    item: String,
    count: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BuyItemResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: serde_json::Value,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct GetPriceRequest<'a> {
    template_id: &'a str,
}

#[derive(Debug, Deserialize)]
struct GetPriceResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Option<Price>,
}

/// Ragfair item price
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    /// Item localization schema ID
    #[serde(rename = "templateId")]
    pub schema_id: String,
    /// Minimum item price
    pub min: f64,
    /// Maximum item price
    pub max: f64,
    /// Average item price
    pub avg: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SellItemRequest<'a> {
    #[serde(rename = "Action")]
    action: &'a str,
    sell_in_one_piece: bool,
    items: &'a [&'a str],
    requirements: &'a [SellRequirement],
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SellRequirement {
    #[serde(flatten)]
    requirement: Requirement,
    level: u64,
    side: u8,
    only_functional: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SellItemResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: serde_json::Value,
}

impl Tarkov {
    /// Search the flea market.
    pub async fn search_market<'a>(
        &self,
        page: u64,
        limit: u64,
        filter: MarketFilter<'_>,
    ) -> Result<SearchResult> {
        let body = SearchRequest {
            page,
            limit,
            sort_type: filter.sort_type,
            sort_direction: filter.sort_direction,
            currency: filter.currency,
            price_from: filter.min_price.unwrap_or(0),
            price_to: filter.max_price.unwrap_or(0),
            quantity_from: filter.min_quantity.unwrap_or(0),
            quantity_to: filter.max_quantity.unwrap_or(0),
            condition_from: filter.min_condition.unwrap_or(0),
            condition_to: filter.max_condition.unwrap_or(0),
            one_hour_expiration: filter.expiring_within_hour,
            remove_bartering: filter.hide_bartering_offers,
            offer_owner_type: filter.owner_type,
            only_functional: filter.hide_inoperable_weapons,
            update_offer_count: true,
            handbook_id: &filter.handbook_id.unwrap_or(""),
            linked_search_id: &filter.linked_search_id.unwrap_or(""),
            needed_search_id: &filter.required_search_id.unwrap_or(""),
            tm: 1,
        };

        let url = format!("{}/client/ragfair/find", RAGFAIR_ENDPOINT);
        let res: SearchResponse = self.post_json(&url, &body).await?;

        handle_error(res.error, res.data)
    }

    /// Get the item price data from the flea market.
    pub async fn get_item_price(&self, schema_id: &str) -> Result<Price> {
        let url = format!("{}/client/ragfair/itemMarketPrice", RAGFAIR_ENDPOINT);
        let body = GetPriceRequest {
            template_id: schema_id,
        };

        let res: GetPriceResponse = self.post_json(&url, &body).await?;
        handle_error(res.error, res.data)
    }

    /// Buy items from the flea market.
    pub async fn buy_item(
        &self,
        offer_id: &str,
        quantity: u64,
        barter_items: &[BarterItem],
    ) -> Result<InventoryUpdate> {
        let url = format!("{}/client/game/profile/items/moving", PROD_ENDPOINT);
        let body = &MoveItemRequest {
            data: &[BuyItemRequest {
                action: "RagFairBuyOffer",
                offers: &[BuyOffer {
                    id: offer_id,
                    count: quantity,
                    items: barter_items,
                }],
            }],
            tm: 2,
        };

        let res: BuyItemResponse = self.post_json(&url, body).await?;
        handle_error2(res.error)?;

        let res: RagfairResponseData = Deserialize::deserialize(res.data)?;
        if !res.errors.is_empty() {
            let error = &res.errors[0];
            match error.code {
                1503 | 1506 => return Err(RagfairError::OfferNotFound)?,
                _ => return Err(Error::UnknownAPIError(error.code)),
            }
        }

        let items: InventoryUpdate = Deserialize::deserialize(res.items)?;
        Ok(items)
    }

    /// List an item for sale on the flea market.
    pub async fn offer_item(
        &self,
        items: &[&str],
        requirements: &[Requirement],
        sell_all: bool,
    ) -> Result<InventoryUpdate> {
        let url = format!("{}/client/game/profile/items/moving", PROD_ENDPOINT);
        let body = &MoveItemRequest {
            data: &[SellItemRequest {
                action: "RagFairAddOffer",
                sell_in_one_piece: sell_all,
                items,
                requirements: &requirements
                    .into_iter()
                    .map(|r| SellRequirement {
                        requirement: r.to_owned(),
                        level: 0,
                        side: 0,
                        only_functional: false,
                    })
                    .collect::<Vec<SellRequirement>>(),
            }],
            tm: 2,
        };

        let res: SellItemResponse = self.post_json(&url, body).await?;
        handle_error2(res.error)?;

        let res: RagfairResponseData = Deserialize::deserialize(res.data)?;
        if !res.errors.is_empty() {
            let error = &res.errors[0];
            return Err(Error::UnknownAPIError(error.code));
        }

        let items: InventoryUpdate = Deserialize::deserialize(res.items)?;
        Ok(items)
    }
}
