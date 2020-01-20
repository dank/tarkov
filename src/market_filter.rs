use serde_repr::Serialize_repr;

/// Search filter for the flea market.
#[derive(Debug)]
pub struct MarketFilter<'a> {
    /// Sort type.
    pub sort_type: SortBy,
    /// Sort direction.
    pub sort_direction: SortDirection,
    /// Offer currency type.
    pub currency: Currency,
    /// Minimum item price.
    pub min_price: Option<u64>,
    /// Maximum item price.
    pub max_price: Option<u64>,
    /// Minimum item quantity.
    pub min_quantity: Option<u64>,
    /// Maximum item quantity.
    pub max_quantity: Option<u64>,
    /// Minimum item condition percentage.
    pub min_condition: Option<u64>,
    /// Maximum item condition percentage.
    pub max_condition: Option<u64>,
    /// Show offers expiring soon.
    pub expiring_within_hour: bool,
    /// Hide offers asking for items for trade.
    pub hide_bartering_offers: bool,
    /// Offer owner type.
    pub owner_type: Owner,
    /// Hide inoperable weapons.
    pub hide_inoperable_weapons: bool,
    /// Search by market category or item ID.
    pub handbook_id: Option<&'a str>,
    /// Search item related to item ID.
    pub linked_search_id: Option<&'a str>,
    /// Search items that can be traded for item ID.
    pub required_search_id: Option<&'a str>,
}

/// Sort by categories.
#[derive(Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum SortBy {
    /// Sort by ID
    ID = 0,
    /// Sort by bartering offers
    BarteringOffers = 2,
    /// Sort by merchant rating
    MerchantRating = 3,
    /// Sort by price (default)
    Price = 5,
    /// Sort by expiry
    Expiry = 6,
}

/// Sort by direction.
#[derive(Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum SortDirection {
    /// Sort ascending (default)
    Ascending = 0,
    /// Sort descending
    Descending = 1,
}

/// Currency types.
#[derive(Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum Currency {
    /// Any currency (default)
    Any = 0,
    /// Rouble
    Rouble = 1,
    /// US dollar
    Dollar = 2,
    /// Euro
    Euro = 3,
}

/// Item listed by.
#[derive(Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum Owner {
    /// Any owner (default)
    Any = 0,
    /// Item listed by traders
    Traders = 1,
    /// Item listed by players
    Player = 2,
}

impl<'a> Default for MarketFilter<'a> {
    fn default() -> Self {
        Self {
            sort_type: SortBy::Price,
            sort_direction: SortDirection::Ascending,
            currency: Currency::Any,
            min_price: None,
            max_price: None,
            min_quantity: None,
            max_quantity: None,
            min_condition: None,
            max_condition: Some(100),
            expiring_within_hour: false,
            hide_bartering_offers: false,
            owner_type: Owner::Any,
            hide_inoperable_weapons: true,
            handbook_id: None,
            linked_search_id: None,
            required_search_id: None,
        }
    }
}
