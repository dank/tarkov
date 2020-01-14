use crate::{Error, ErrorResponse, Result, Tarkov, TRADING_ENDPOINT};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Trader {
    #[serde(rename = "_id")]
    id: String,
    working: bool,
    customization_seller: bool,
    name: String,
    surname: String,
    nickname: String,
    location: String,
    avatar: String,
    balance_rub: u64,
    balance_dol: u64,
    balance_eur: u64,
    display: bool,
    discount: i64,
    discount_end: i64,
    buyer_up: bool,
    // enum?
    currency: String,
    supply_next_time: u64,
    repair: Repair,
    insurance: Insurance,
    #[serde(rename = "gridHeight")]
    grid_height: u64,
    loyalty: Loyalty,
    // sell_category: []
}

#[derive(Debug, Deserialize)]
pub struct Repair {
    availability: bool,
    quality: String,
    excluded_id_list: Vec<String>,
    excluded_category: Vec<String>,
    currency: Option<String>,
    currency_coefficient: Option<u64>,
    price_rate: u64,
}

#[derive(Debug, Deserialize)]
pub struct Insurance {
    availability: bool,
    min_payment: u64,
    min_return_hour: u64,
    max_return_hour: u64,
    max_storage_time: u64,
    excluded_category: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Loyalty {
    current_level: u64,
    current_standing: f64,
    current_sales_sum: u64,
    loyalty_levels: HashMap<String, LoyaltyLevel>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoyaltyLevel {
    min_level: u64,
    min_sales_sum: u64,
    min_standing: f64,
}

#[derive(Debug, Deserialize)]
struct TraderResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Option<Vec<Trader>>,
}

impl Tarkov {
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
}
