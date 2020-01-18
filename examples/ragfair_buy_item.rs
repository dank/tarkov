use std::time::{SystemTime, UNIX_EPOCH};
use tarkov::market_filter::{Currency, MarketFilter, Owner};
use tarkov::profile::Side;
use tarkov::trading::BarterItem;
use tarkov::{Error, Tarkov};

#[actix_rt::main]
async fn main() -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "tarkov=info");
    env_logger::init();

    let t = Tarkov::from_session("e1bc65a216325f0ad0db8518fa299db2").await?;

    // Find and select PMC profile to complete login.
    let profiles = t.get_profiles().await?;
    let profile = profiles
        .into_iter()
        .find(|p| p.info.side != Side::Savage)
        .unwrap();
    t.select_profile(&profile.id).await?;

    // Fetch the translation table for market categories.
    let locale = t.get_i18n("en").await?;

    // Get the "Barter items" category ID.
    let barter_item_category_id: Option<String> = {
        let mut id = None;
        for (k, v) in locale.handbook {
            if v == "Barter items" {
                id = Some(k);
                break;
            }
        }

        id
    };

    // Search flea market.
    let offers = t
        .search_market(
            0,
            15,
            MarketFilter {
                max_price: Some(2000),
                handbook_id: barter_item_category_id.as_deref(),
                owner_type: Owner::Player,
                hide_bartering_offers: true,
                currency: Currency::Rouble,
                ..MarketFilter::default()
            },
        )
        .await?;

    // Find the first item available for purchase immediately.
    let current_time = SystemTime::now();
    let epoch_time = current_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let offer = offers
        .offers
        .into_iter()
        .find(|o| o.start_time + 60 <= epoch_time && o.end_time >= epoch_time)
        .unwrap();

    // Get Rouble item ID from my inventory
    let rouble = &profile
        .inventory
        .items
        .into_iter()
        .find(|i| i.schema_id == "5449016a4bdc2d6f028b456f")
        .unwrap();

    // Buy the item.
    t.buy_item(
        &offer.id,
        1,
        &[BarterItem {
            id: rouble.id.clone(),
            count: offer.requirements_cost as f64,
        }],
    )
    .await?;

    Ok(())
}
