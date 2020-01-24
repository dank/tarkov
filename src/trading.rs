use crate::inventory::{
    BarterItem, InventoryUpdate, Item, MoveItemRequest, RagfairResponseData, Upd,
};
use crate::{
    handle_error, handle_error2, Error, ErrorResponse, Result, Tarkov, PROD_ENDPOINT,
    TRADING_ENDPOINT,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trading error
#[derive(Debug, err_derive::Error)]
pub enum TradingError {
    /// Transaction error
    #[error(display = "transaction error")]
    TransactionError,
    /// Loyalty level is not high enough to purchase this item.
    #[error(display = "bad loyalty level")]
    BadLoyaltyLevel,
}

/// Trader info
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Trader {
    /// Trader ID
    #[serde(rename = "_id")]
    pub id: String,
    /// Trader is working
    pub working: bool,
    /// ?
    pub customization_seller: bool,
    /// Trader name
    pub name: String,
    /// Trader surname
    pub surname: String,
    /// Trader nickname
    pub nickname: String,
    /// Trader location
    pub location: String,
    /// Trader avatar
    pub avatar: String,
    /// Trader rouble balance
    pub balance_rub: u64,
    /// Trader dollar balance
    pub balance_dol: u64,
    /// Trader euro balance
    pub balance_eur: u64,
    /// ?
    pub display: bool,
    /// Trader discount
    pub discount: i64,
    /// Trader discount expiry
    pub discount_end: i64,
    /// ?
    pub buyer_up: bool,
    /// Trader currency
    pub currency: Currency,
    /// Resupply time
    pub supply_next_time: u64,
    /// Trader repair offer
    pub repair: Repair,
    /// Trader insurance offer
    pub insurance: Insurance,
    /// Trader grid height
    #[serde(rename = "gridHeight")]
    pub grid_height: u64,
    /// Trader loyalty
    pub loyalty: Loyalty,
    /// Unknown type
    pub sell_category: Vec<serde_json::Value>,
}

/// Trader's repair stats
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Repair {
    /// Repair is available
    pub availability: bool,
    /// Repair quality
    pub quality: String,
    /// Item IDs excluded from repair.
    pub excluded_id_list: Vec<String>,
    /// Category IDs excluded from repair.
    pub excluded_category: Vec<String>,
    /// Currency
    pub currency: Option<String>,
    /// ?
    pub currency_coefficient: Option<u64>,
    /// Repair price rate
    pub price_rate: u64,
}

/// Trader currency
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum Currency {
    /// Rouble
    #[serde(rename = "RUB")]
    Rouble,
    /// US Dollar
    #[serde(rename = "USD")]
    Dollar,
    /// Euro
    #[serde(rename = "EUR")]
    Euro,
}

/// Trader's insurance offer
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Insurance {
    /// Insurance is available
    pub availability: bool,
    /// Minimum cost to insure
    pub min_payment: u64,
    /// Minimum return time in hours.
    pub min_return_hour: u64,
    /// Maximum return time in hours.
    pub max_return_hour: u64,
    /// Maximum storage time in hours.
    pub max_storage_time: u64,
    /// Categories IDs excluded from insurance.
    pub excluded_category: Vec<String>,
}

/// Trader loyalty
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Loyalty {
    /// Current loyalty level
    pub current_level: u64,
    /// Current loyalty standing
    pub current_standing: f64,
    /// Amount spent on trader
    pub current_sales_sum: u64,
    /// All loyalty levels
    pub loyalty_levels: HashMap<String, LoyaltyLevel>,
}

/// Trader loyalty level
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LoyaltyLevel {
    /// Minimum level
    pub min_level: u64,
    /// Minimum sales amount
    pub min_sales_sum: u64,
    /// Minimum standing
    pub min_standing: f64,
}

#[derive(Debug, Deserialize)]
struct TradersResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Option<Vec<Trader>>,
}

#[derive(Debug, Deserialize)]
struct TraderResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Option<Trader>,
}

#[derive(Debug, Deserialize)]
struct TraderItemsResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Option<TraderItems>,
}

#[derive(Debug, Deserialize)]
struct TraderItems {
    items: Vec<Item>,
    barter_scheme: HashMap<String, Vec<Vec<Price>>>,
    loyal_level_items: HashMap<String, u8>,
}

#[derive(Debug, Deserialize)]
struct TraderPricesResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Option<HashMap<String, Vec<Vec<Price>>>>,
}

/// Trader item price
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Price {
    /// Item localization schema ID
    #[serde(rename = "_tpl")]
    pub schema_id: String,
    /// Item count
    pub count: f64,
}

/// Item for trade
#[derive(Debug, Clone, PartialEq)]
pub struct TraderItem {
    /// Item ID
    pub id: String,
    /// Item localization schema ID
    pub schema_id: String,
    /// Item attachments/options
    pub upd: Option<Upd>,
    /// Item price
    pub price: Vec<Price>,
    /// Loyalty level
    pub loyalty_level: u8,
}

#[derive(Debug, Serialize)]
struct TradeItemRequest<'a> {
    #[serde(rename = "Action")]
    action: &'a str,
    #[serde(rename = "type")]
    trade_type: &'a str,
    #[serde(rename = "tid")]
    trader_id: &'a str,
    item_id: &'a str,
    count: u64,
    scheme_id: u64,
    scheme_items: &'a [BarterItem],
}

#[derive(Debug, Deserialize)]
struct TradeResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct SellItemRequest<'a> {
    #[serde(rename = "Action")]
    action: &'a str,
    #[serde(rename = "type")]
    trade_type: &'a str,
    #[serde(rename = "tid")]
    trader_id: &'a str,
    items: &'a [SellItem],
}

#[derive(Debug, Serialize)]
struct SellItem {
    id: String,
    count: u64,
    scheme_id: u64,
}

#[derive(Debug, Deserialize)]
struct SellResponse {
    #[serde(flatten)]
    error: ErrorResponse,
}

impl Tarkov {
    /// Get a list of all traders.
    pub async fn get_traders(&self) -> Result<Vec<Trader>> {
        let url = format!("{}/client/trading/api/getTradersList", TRADING_ENDPOINT);
        let res: TradersResponse = self.post_json(&url, &{}).await?;

        handle_error(res.error, res.data)
    }

    /// Get a trader by ID.
    pub async fn get_trader(&self, trader_id: &str) -> Result<Trader> {
        if trader_id.is_empty() {
            return Err(Error::InvalidParameters);
        }

        let url = format!(
            "{}/client/trading/api/getTrader/{}",
            TRADING_ENDPOINT, trader_id
        );
        let res: TraderResponse = self.post_json(&url, &{}).await?;

        handle_error(res.error, res.data)
    }

    async fn get_trader_items_raw(&self, trader_id: &str) -> Result<TraderItems> {
        let url = format!(
            "{}/client/trading/api/getTraderAssort/{}",
            TRADING_ENDPOINT, trader_id
        );
        let res: TraderItemsResponse = self.post_json(&url, &{}).await?;

        handle_error(res.error, res.data)
    }

    async fn get_trader_prices_raw(
        &self,
        trader_id: &str,
    ) -> Result<HashMap<String, Vec<Vec<Price>>>> {
        let url = format!(
            "{}/client/trading/api/getUserAssortPrice/trader/{}",
            TRADING_ENDPOINT, trader_id
        );
        let res: TraderPricesResponse = self.post_json(&url, &{}).await?;

        handle_error(res.error, res.data)
    }

    /// Get a list of items for sale by trader ID.
    pub async fn get_trader_items(&self, trader_id: &str) -> Result<Vec<TraderItem>> {
        if trader_id.is_empty() {
            return Err(Error::InvalidParameters);
        }

        let mut result: Vec<TraderItem> = Vec::new();

        let items = self.get_trader_items_raw(trader_id).await?;
        let prices = self.get_trader_prices_raw(trader_id).await?;

        for item in items.items {
            // TODO: Properly deal with parent/children items
            if item.parent_id != Some("hideout".to_string()) {
                continue;
            }

            let loyalty_level = items
                .loyal_level_items
                .get(&item.id)
                .expect("Loyalty level could not be mapped.");
            let price = {
                let barter_or_price = match items.barter_scheme.get(&item.id) {
                    None => prices
                        .get(&item.id)
                        .expect("Item price could not be mapped."),
                    Some(barter) => barter,
                };

                barter_or_price.get(0)
            };

            let trader_item = TraderItem {
                id: item.id,
                schema_id: item.schema_id,
                upd: item.upd,
                price: price.expect("Item price could not be mapped.").clone(),
                loyalty_level: *loyalty_level,
            };

            result.push(trader_item);
        }

        Ok(result)
    }

    /// Trade items with traders.
    ///
    /// All trades, including cash trades, are considered bartering. `barter_items` expects a
    /// list of items from your inventory that matches the item price.
    pub async fn trade_item(
        &self,
        trader_id: &str,
        item_id: &str,
        quantity: u64,
        barter_items: &[BarterItem],
    ) -> Result<InventoryUpdate> {
        if trader_id.is_empty() || item_id.is_empty() || quantity == 0 || barter_items.is_empty() {
            return Err(Error::InvalidParameters);
        }

        let url = format!("{}/client/game/profile/items/moving", PROD_ENDPOINT);
        let body = MoveItemRequest {
            data: &[TradeItemRequest {
                action: "TradingConfirm",
                trade_type: "buy_from_trader",
                trader_id,
                item_id,
                count: quantity,
                scheme_id: 0,
                scheme_items: barter_items,
            }],
            tm: 0,
        };

        let res: TradeResponse = self.post_json(&url, &body).await?;
        handle_error2(res.error)?;

        let res: RagfairResponseData = Deserialize::deserialize(res.data)?;
        if !res.errors.is_empty() {
            let error = &res.errors[0];
            return Err(Error::UnknownAPIError(error.code));
        }

        let items: InventoryUpdate = Deserialize::deserialize(res.items)?;
        Ok(items)
    }

    /// Sell items to trader.
    pub async fn sell_item(
        &self,
        trader_id: &str,
        item_id: &str,
        quantity: u64,
    ) -> Result<InventoryUpdate> {
        if trader_id.is_empty() || item_id.is_empty() || quantity == 0 {
            return Err(Error::InvalidParameters);
        }

        let url = format!("{}/client/game/profile/items/moving", PROD_ENDPOINT);
        let body = MoveItemRequest {
            data: &[SellItemRequest {
                action: "TradingConfirm",
                trade_type: "sell_to_trader",
                trader_id,
                items: &[SellItem {
                    id: item_id.to_string(),
                    count: quantity,
                    scheme_id: 0,
                }],
            }],
            tm: 0,
        };

        let res: TradeResponse = self.post_json(&url, &body).await?;
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
