use crate::{ErrorResponse, Result, Tarkov, RAGFAIR_ENDPOINT};

use crate::profile::MoveItemRequest;
use crate::trading::{BarterItem, Item};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::market_filter::{Currency, MarketFilter, SortBy, SortDirection, OfferOwner};

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
    offer_owner_type: OfferOwner,
    only_functional: bool,
    update_offer_count: bool,
    handbook_id: &'a str,
    linked_search_id: &'a str,
    needed_search_id: &'a str,
    //build_items: {}
    //build_count: u64,
    tm: u64,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
struct SearchResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Option<SearchResult>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub categories: HashMap<String, u64>,
    pub offers: Vec<Offer>,
    pub offers_count: u64,
    pub selected_category: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Offer {
    #[serde(rename = "_id")]
    pub id: String,
    pub int_id: String,
    pub user: User,
    pub root: String,
    pub items: Vec<Item>,
    pub items_cost: u64,
    pub requirements: Vec<Requirement>,
    pub requirements_cost: u64,
    pub summary_cost: u64,
    pub sell_in_one_piece: bool,
    pub start_time: u64,
    pub end_time: u64,
    pub loyalty_level: u64,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub member_type: u64,
    pub nickname: String,
    pub rating: f64,
    pub is_rating_growing: bool,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Requirement {
    #[serde(rename = "_tpl")]
    pub schema_id: String,
    pub count: u64,
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
    items: Vec<BuyItem>,
}

#[derive(Debug, Serialize)]
struct BuyItem {
    item: String,
    count: f64,
}

impl Tarkov {
    /// Search the flea market.
    pub async fn search_market(&self, page: u64, limit: u64, filter: MarketFilter) -> Result<()> {
        let body = SearchRequest {
            page,
            limit,
            sort_type: filter.sort_type,
            sort_direction: filter.sort_direction,
            currency: filter.currency,
            price_from: filter.price_from.unwrap_or(0),
            price_to: filter.price_to.unwrap_or(0),
            quantity_from: filter.quantity_from.unwrap_or(0),
            quantity_to: filter.quantity_to.unwrap_or(0),
            condition_from: filter.condition_from.unwrap_or(0),
            condition_to: filter.condition_to.unwrap_or(0),
            one_hour_expiration: filter.expiring_within_hour,
            remove_bartering: filter.hide_bartering_offers,
            offer_owner_type: filter.offer_owner, // 0
            only_functional: filter.hide_inoperable_weapons,
            update_offer_count: true,
            handbook_id: &filter.handbook_id.unwrap_or("".to_string()),
            linked_search_id: &filter.linked_search_id.unwrap_or("".to_string()),
            needed_search_id: &filter.required_search_id.unwrap_or("".to_string()),
            tm: 1,
        };

        println!("{:?}", serde_json::to_string(&body));
//        let url = format!("{}/client/ragfair/find", RAGFAIR_ENDPOINT);
//        let res: SearchResponse = self.post_json(&url, &body).await?;
//
//        self.handle_error(res.error, res.data)

        Ok(())
    }

    /// Buy items from the flea market.
    pub async fn buy_item(
        &self,
        item_schema_id: &str,
        barter_items: &[BarterItem],
    ) -> Result<()> {
        let url = format!("{}/client/ragfair/find", RAGFAIR_ENDPOINT);
        let body = &MoveItemRequest {
            // TODO: make data &[]
            data: &[BuyItemRequest {
                action: "RagFairBuyOffer",
                offers: &[BuyOffer {
                    id: item_schema_id,
                    count: 0,
                    items: barter_items
                        .into_iter()
                        .map(|i| BuyItem {
                            item: i.id.to_string(),
                            count: i.count,
                        })
                        .collect::<Vec<BuyItem>>()
                }],
            }],
            tm: 0,
        };
        let res: SearchResponse = self.post_json(&url, body).await?;

        Ok(())
    }

    /// List an item for sale on the flea market.
    pub async fn offer_item(&self) -> Result<()> {
        Ok(())
    }
}
