use crate::{Error, ErrorResponse, Result, Tarkov, TRADING_ENDPOINT};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
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
    // XXX: This could be an enum?
    pub currency: String,
    pub supply_next_time: u64,
    pub repair: Repair,
    pub insurance: Insurance,
    #[serde(rename = "gridHeight")]
    pub grid_height: u64,
    pub loyalty: Loyalty,
    // sell_category: []
}

#[derive(Debug, Deserialize)]
pub struct Repair {
    pub availability: bool,
    pub quality: String,
    pub excluded_id_list: Vec<String>,
    pub excluded_category: Vec<String>,
    pub currency: Option<String>,
    pub currency_coefficient: Option<u64>,
    pub price_rate: u64,
}

#[derive(Debug, Deserialize)]
pub struct Insurance {
    pub availability: bool,
    pub min_payment: u64,
    pub min_return_hour: u64,
    pub max_return_hour: u64,
    pub max_storage_time: u64,
    pub excluded_category: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Loyalty {
    pub current_level: u64,
    pub current_standing: f64,
    pub current_sales_sum: u64,
    pub loyalty_levels: HashMap<String, LoyaltyLevel>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoyaltyLevel {
    pub min_level: u64,
    pub min_sales_sum: u64,
    pub min_standing: f64,
}

#[derive(Debug, Deserialize)]
struct TraderResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Option<Vec<Trader>>,
}

#[derive(Debug, Deserialize)]
struct GetTraderResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Option<Trader>,
}

impl Tarkov {
    /// Get a list of all traders.
    pub async fn get_traders(&self) -> Result<Vec<Trader>> {
        let url = format!("{}/client/trading/api/getTradersList", TRADING_ENDPOINT);
        let res: TraderResponse = self.post_json(&url, &{}).await?;

        match res.error.code {
            0 => Ok(res
                .data
                .expect("API returned no errors but `data` is unavailable.")),
            _ => Err(Error::UnknownAPIError(res.error.code)),
        }
    }

    /// Get a trader by ID.
    pub async fn get_trader(&self, trader_id: &str) -> Result<Trader> {
        let url = format!(
            "{}/client/trading/api/getTrader/{}",
            TRADING_ENDPOINT, trader_id
        );
        let res: GetTraderResponse = self.post_json(&url, &{}).await?;

        match res.error.code {
            0 => Ok(res
                .data
                .expect("API returned no errors but `data` is unavailable.")),
            _ => Err(Error::UnknownAPIError(res.error.code)),
        }
    }
}
