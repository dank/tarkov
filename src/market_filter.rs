use serde_repr::Serialize_repr;

/// Search filter for the flea market.
#[derive(Debug)]
pub struct MarketFilter {
    /// Sort type.
    pub sort_type: SortBy,
    /// Sort direction.
    pub sort_direction: SortDirection,
    /// Offer currency type.
    pub currency: Currency,
    /// Minimum item price.
    pub price_from: Option<u64>,
    /// Maximum item price.
    pub price_to: Option<u64>,
    /// Minimum item quantity.
    pub quantity_from: Option<u64>,
    /// Maximum item quantity.
    pub quantity_to: Option<u64>,
    /// Minimum item condition percentage.
    pub condition_from: Option<u64>,
    /// Maximum item condition percentage.
    pub condition_to: Option<u64>,
    /// Show offers expiring soon.
    pub expiring_within_hour: bool,
    /// Hide offers asking for items for trade.
    pub hide_bartering_offers: bool,
    /// Offer owner type.
    pub offer_owner: OfferOwner,
    /// Hide inoperable weapons.
    pub hide_inoperable_weapons: bool,
    /// Search by market category ID.
    pub handbook_id: Option<String>,
    /// Search item related to item ID.
    pub linked_search_id: Option<String>,
    /// Search items that can be traded for item ID.
    pub required_search_id: Option<String>,
}

/// Sort by categories.
#[derive(Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum SortBy {
    ID = 0,
    BarteringOffers = 2,
    MerchantRating = 3,
    Price = 5,
    Expiry = 6,
}

/// Sort by direction.
#[derive(Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum SortDirection {
    Ascending = 0,
    Descending = 1,
}

/// Currency types.
#[derive(Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum Currency {
    Any = 0,
    Rouble = 1,
    Dollar = 2,
    Euro = 3,
}

/// Item listed by.
#[derive(Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum OfferOwner {
    Any = 0,
    Traders = 1,
    Player = 2,
}

impl Default for MarketFilter {
    fn default() -> Self {
        Self {
            sort_type: SortBy::Price,
            sort_direction: SortDirection::Ascending,
            currency: Currency::Any,
            price_from: None,
            price_to: None,
            quantity_from: None,
            quantity_to: None,
            condition_from: None,
            condition_to: Some(100),
            expiring_within_hour: false,
            hide_bartering_offers: false,
            offer_owner: OfferOwner::Any,
            hide_inoperable_weapons: true,
            handbook_id: None,
            linked_search_id: None,
            required_search_id: None,
        }
    }
}
