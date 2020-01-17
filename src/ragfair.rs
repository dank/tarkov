use crate::{ErrorResponse, Result, Tarkov, RAGFAIR_ENDPOINT};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use crate::trading::Item;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchRequest<'a> {
    page: u64,
    limit: u64,
    sort_type: u64,
    sort_direction: i64,
    currency: u64,
    price_from: u64,
    price_to: u64,
    quantity_from: u64,
    quantity_to: u64,
    condition_from: u64,
    condition_to: u64,
    one_hour_expiration: bool,
    remove_bartering: bool,
    offer_owner_type: u64,
    only_functional: bool,
    update_offer_count: bool,
    handbook_id: &'a str,
    linked_search_id: &'a str,
    needed_search_id: &'a str,
    // build_items: {}
    build_count: u64,
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

impl Tarkov {
    /// Search the flea market.
    pub async fn search_market(&self) -> Result<SearchResult> {
        let body = SearchRequest {
            page: 0,
            limit: 15,
            sort_type: 5,
            sort_direction: 0,
            currency: 0,
            price_from: 0,
            price_to: 0,
            quantity_from: 0,
            quantity_to: 0,
            condition_from: 0,
            condition_to: 100,
            one_hour_expiration: false,
            remove_bartering: false,
            offer_owner_type: 0,
            only_functional: true,
            update_offer_count: true,
            handbook_id: "5b5f78dc86f77409407a7f8e",
            linked_search_id: "",
            needed_search_id: "",
            build_count: 0,
            tm: 1,
        };
        let url = format!("{}/client/ragfair/find", RAGFAIR_ENDPOINT);
        let res: SearchResponse = self.post_json(&url, &body).await?;

        self.handle_error(
            res.error,
            res.data
        )
    }
}
