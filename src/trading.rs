use crate::profile::{MoveItemRequest, UpdLight, UpdMedkit, UpdRepairable};
use crate::{ErrorResponse, Result, Tarkov, PROD_ENDPOINT, TRADING_ENDPOINT};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, err_derive::Error)]
pub enum TradingError {
    /// Transaction error
    #[error(display = "transaction error")]
    TransactionError,
}

/// Trader info.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Trader {
    #[serde(rename = "_id")]
    pub id: String,
    pub working: bool,
    pub customization_seller: bool,
    pub name: String,
    pub surname: String,
    pub nickname: String,
    pub location: String,
    pub avatar: String,
    pub balance_rub: u64,
    pub balance_dol: u64,
    pub balance_eur: u64,
    pub display: bool,
    pub discount: i64,
    pub discount_end: i64,
    pub buyer_up: bool,
    pub currency: Currency,
    pub supply_next_time: u64,
    pub repair: Repair,
    pub insurance: Insurance,
    #[serde(rename = "gridHeight")]
    pub grid_height: u64,
    pub loyalty: Loyalty,
    /// Unknown type
    pub sell_category: Vec<serde_json::Value>,
}

/// Trader's repair stats.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Repair {
    pub availability: bool,
    pub quality: String,
    pub excluded_id_list: Vec<String>,
    pub excluded_category: Vec<String>,
    pub currency: Option<String>,
    pub currency_coefficient: Option<u64>,
    pub price_rate: u64,
}

/// Trader currency.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum Currency {
    #[serde(rename = "RUB")]
    Rouble,
    #[serde(rename = "USD")]
    Dollar,
    #[serde(rename = "EUR")]
    Euro,
}

/// Trader's insurance offer.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Insurance {
    pub availability: bool,
    pub min_payment: u64,
    pub min_return_hour: u64,
    pub max_return_hour: u64,
    pub max_storage_time: u64,
    pub excluded_category: Vec<String>,
}

/// Trader loyalty.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Loyalty {
    pub current_level: u64,
    pub current_standing: f64,
    pub current_sales_sum: u64,
    pub loyalty_levels: HashMap<String, LoyaltyLevel>,
}

/// Trader loyalty level.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LoyaltyLevel {
    pub min_level: u64,
    pub min_sales_sum: u64,
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

/// In-game item.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_tpl")]
    pub schema_id: String,
    pub parent_id: Option<String>,
    pub slot_id: Option<String>,
    pub upd: Option<Upd>,
    // XXX: This type can be both Integer and `Location`...
    // location: Option<Location>
}

/// Item options.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Upd {
    pub stack_objects_count: Option<u64>,
    pub spawned_in_session: Option<bool>,
    pub med_kit: Option<UpdMedkit>,
    pub repairable: Option<UpdRepairable>,
    pub light: Option<UpdLight>,
    pub unlimited_count: Option<bool>,
    pub buy_restriction_max: Option<u64>,
    pub buy_restriction_current: Option<u64>,
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

/// Trader item price.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Price {
    #[serde(rename = "_tpl")]
    pub schema_id: String,
    pub count: f64,
}

/// Item for trade.
#[derive(Debug, Clone, PartialEq)]
pub struct TraderItem {
    pub id: String,
    pub schema_id: String,
    pub upd: Option<Upd>,
    pub price: Vec<Price>,
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
    scheme_items: Vec<BarterItem>,
}

/// Inventory item for trading.
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct BarterItem {
    /// Item ID from your inventory.
    pub id: String,
    /// Amount of items.
    pub count: f64,
}

#[derive(Debug, Deserialize)]
struct TradeResponse {
    #[serde(flatten)]
    error: ErrorResponse,
}

#[derive(Debug, Serialize)]
struct SellItemRequest<'a> {
    #[serde(rename = "Action")]
    action: &'a str,
    #[serde(rename = "type")]
    trade_type: &'a str,
    #[serde(rename = "tid")]
    trader_id: &'a str,
    items: Vec<SellItem>,
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

        self.handle_error(res.error, res.data)
    }

    /// Get a trader by ID.
    pub async fn get_trader(&self, trader_id: &str) -> Result<Trader> {
        let url = format!(
            "{}/client/trading/api/getTrader/{}",
            TRADING_ENDPOINT, trader_id
        );
        let res: TraderResponse = self.post_json(&url, &{}).await?;

        self.handle_error(res.error, res.data)
    }

    async fn get_trader_items_raw(&self, trader_id: &str) -> Result<TraderItems> {
        let url = format!(
            "{}/client/trading/api/getTraderAssort/{}",
            TRADING_ENDPOINT, trader_id
        );
        let res: TraderItemsResponse = self.post_json(&url, &{}).await?;

        self.handle_error(res.error, res.data)
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

        self.handle_error(res.error, res.data)
    }

    /// Get a list of items for sale by trader ID.
    pub async fn get_trader_items(&self, trader_id: &str) -> Result<Vec<TraderItem>> {
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

    /// Trade items with the specified trader.
    ///
    /// All trades, including cash trades, are considered bartering. `barter_items` expects a
    /// list of items from your inventory that matches the item price.
    pub async fn trade_item(
        &self,
        trader_id: &str,
        item_id: &str,
        quantity: u64,
        barter_items: Vec<BarterItem>,
    ) -> Result<()> {
        let url = format!("{}/client/game/profile/items/moving", PROD_ENDPOINT);
        let body = MoveItemRequest {
            data: vec![TradeItemRequest {
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
        self.handle_error(res.error, Some(()))
    }

    pub async fn sell_item(&self, trader_id: &str, item_id: &str, quantity: u64) -> Result<()> {
        let url = format!("{}/client/game/profile/items/moving", PROD_ENDPOINT);
        let body = MoveItemRequest {
            data: vec![SellItemRequest {
                action: "TradingConfirm",
                trade_type: "sell_to_trader",
                trader_id,
                items: vec![SellItem {
                    id: item_id.to_string(),
                    count: quantity,
                    scheme_id: 0,
                }],
            }],
            tm: 0,
        };

        let res: TradeResponse = self.post_json(&url, &body).await?;
        self.handle_error(res.error, Some(()))
    }
}
