use crate::bad_json::deserialize_bad_location_as_none;
use crate::ErrorResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub(crate) struct MoveItemRequest<'a, T> {
    pub(crate) data: &'a [T],
    pub(crate) tm: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RagfairResponseData {
    pub(crate) items: serde_json::Value,
    #[serde(rename = "badRequest")]
    pub(crate) errors: Vec<ErrorResponse>,
}

/// Changes to the player's inventory after interacting with traders or the flea market.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryUpdate {
    /// New items in inventory.
    pub new: Option<Vec<Item>>,
    /// Changed items in inventory.
    pub change: Option<Vec<Item>>,
    /// Deleted items in inventory.
    pub del: Option<Vec<DeletedItem>>,
}

/// Item deleted from inventory.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeletedItem {
    /// Item ID
    #[serde(rename = "_id")]
    pub id: String,
}

/// In-game item
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    /// Item ID
    #[serde(rename = "_id")]
    pub id: String,
    /// Item localization schema ID
    #[serde(rename = "_tpl")]
    pub schema_id: String,
    /// Item parent ID
    pub parent_id: Option<String>,
    /// Item slot ID
    pub slot_id: Option<String>,
    /// Item attachments/options
    pub upd: Option<Upd>,
    /// Item location
    #[serde(default, deserialize_with = "deserialize_bad_location_as_none")]
    pub location: Option<Location>,
}

/// Item location
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    /// Inventory slot x
    pub x: u64,
    /// Inventory slot y
    pub y: u64,
    /// Inventory slot rotation
    pub r: u64,
    /// Item is searched (if searchable)
    pub is_searched: Option<bool>,
}

/// Item options
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Upd {
    /// Item stack count
    pub stack_objects_count: Option<u64>,
    /// Item spawned in session
    pub spawned_in_session: Option<bool>,
    /// Item is medkit
    pub med_kit: Option<UpdMedkit>,
    /// Item is repairable
    pub repairable: Option<UpdRepairable>,
    /// Item has a light attachment
    pub light: Option<UpdLight>,
    /// Unlimited stack
    pub unlimited_count: Option<bool>,
    /// ?
    pub buy_restriction_max: Option<u64>,
    /// ?
    pub buy_restriction_current: Option<u64>,
    /// Key info
    pub key: Option<UpdKey>,
}

/// Medkit item info
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct UpdMedkit {
    /// Health
    pub hp_resource: f64,
}

/// Repairable item info
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct UpdRepairable {
    /// Maximum durability
    pub max_durability: Option<f64>,
    /// Current durability
    pub durability: f64,
}

/// Light attachment info
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct UpdLight {
    /// Light is active
    pub is_active: bool,
    /// Light mode
    pub selected_mode: u64,
}

/// Key info
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct UpdKey {
    /// Number of usage
    pub number_of_usages: u64,
}

/// Inventory item for trading.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct BarterItem {
    /// Item ID from player's inventory.
    pub id: String,
    /// Amount of items.
    pub count: f64,
}
