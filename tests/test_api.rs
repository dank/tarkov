use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use tarkov::inventory::BarterItem;
use tarkov::market_filter::{Currency, MarketFilter, Owner};
use tarkov::profile::Side;
use tarkov::ragfair::Requirement;
use tarkov::{Result, Tarkov};

#[tokio::test]
async fn test_profile() -> Result<()> {
    let session = env::var("TARKOV_SESSION").unwrap();
    let t = Tarkov::from_session(&session);

    let profiles = t.get_profiles().await?;
    let profile = profiles
        .into_iter()
        .find(|p| p.info.side != Side::Savage)
        .unwrap();

    t.select_profile(&profile.id).await?;

    Ok(())
}

#[tokio::test]
async fn test_keep_alive() -> Result<()> {
    let session = env::var("TARKOV_SESSION").unwrap();
    let t = Tarkov::from_session(&session);

    t.keep_alive().await?;

    Ok(())
}

#[tokio::test]
async fn test_constants() -> Result<()> {
    let session = env::var("TARKOV_SESSION").unwrap();
    let t = Tarkov::from_session(&session);

    let _ = t.get_items().await?;
    let _ = t.get_item_prices().await?;
    let _ = t.get_locations().await?;
    let _ = t.get_weather().await?;
    let _ = t.get_i18n("en").await?;

    Ok(())
}

#[tokio::test]
async fn test_flea_market_getters() -> Result<()> {
    let session = env::var("TARKOV_SESSION").unwrap();
    let t = Tarkov::from_session(&session);

    let _ = t.search_market(0, 15, MarketFilter::default()).await?;

    // USD price
    let _ = t.get_item_price("5696686a4bdc2da3298b456a").await?;

    Ok(())
}

#[tokio::test]
async fn test_flea_market_buying() -> Result<()> {
    let session = env::var("TARKOV_SESSION").unwrap();
    let t = Tarkov::from_session(&session);

    let profiles = t.get_profiles().await?;
    let profile = profiles
        .into_iter()
        .find(|p| p.info.side != Side::Savage)
        .unwrap();

    let offers = t
        .search_market(
            0,
            15,
            MarketFilter {
                max_price: Some(2000),
                handbook_id: Some("5b47574386f77428ca22b33e"),
                owner_type: Owner::Player,
                hide_bartering_offers: true,
                currency: Currency::Rouble,
                ..MarketFilter::default()
            },
        )
        .await?;

    let current_time = SystemTime::now();
    let epoch_time = current_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let offer = offers
        .offers
        .into_iter()
        .find(|o| o.start_time + 60 <= epoch_time && o.end_time >= epoch_time)
        .unwrap();

    let rouble = &profile
        .inventory
        .items
        .into_iter()
        .find(|i| i.schema_id == "5449016a4bdc2d6f028b456f")
        .unwrap();

    let _ = t
        .buy_item(
            &offer.id,
            1,
            &[BarterItem {
                id: rouble.id.to_owned(),
                count: offer.requirements_cost as f64,
            }],
        )
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_flea_market_selling() -> Result<()> {
    let session = env::var("TARKOV_SESSION").unwrap();
    let t = Tarkov::from_session(&session);

    let profiles = t.get_profiles().await?;
    let profile = profiles
        .into_iter()
        .find(|p| p.info.side != Side::Savage)
        .unwrap();

    let painkiller = &profile
        .inventory
        .items
        .into_iter()
        .find(|i| i.schema_id == "544fb37f4bdc2dee738b4567")
        .unwrap();

    let _ = t
        .offer_item(
            &[&painkiller.id],
            &[Requirement {
                schema_id: "5449016a4bdc2d6f028b456f".to_string(),
                count: 2000.0,
            }],
            false,
        )
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_trader_getters() -> Result<()> {
    let session = env::var("TARKOV_SESSION").unwrap();
    let t = Tarkov::from_session(&session);

    let traders = t.get_traders().await?;
    let trader = traders.get(0).unwrap();
    let trader = t.get_trader(&trader.id).await?;

    let _ = t.get_trader_items(&trader.id).await?;

    Ok(())
}

#[tokio::test]
async fn test_trader_buying() -> Result<()> {
    let session = env::var("TARKOV_SESSION").unwrap();
    let t = Tarkov::from_session(&session);

    let profiles = t.get_profiles().await?;
    let profile = profiles
        .into_iter()
        .find(|p| p.info.side != Side::Savage)
        .unwrap();

    let traders = t.get_traders().await?;
    let trader = traders
        .into_iter()
        .find(|t| t.nickname == "Терапевт")
        .unwrap();

    let rouble = &profile
        .inventory
        .items
        .into_iter()
        .find(|i| {
            i.schema_id == "5449016a4bdc2d6f028b456f"
                && i.upd.as_ref().unwrap().stack_objects_count >= Some(3990)
        })
        .unwrap();

    let _ = t
        .trade_item(
            &trader.id,
            "5e064f5deb009468d90baef7", // ID might change.
            1,
            &[BarterItem {
                id: rouble.id.to_owned(),
                count: 3990.0, // Assume price is 3990₽.
            }],
        )
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_trader_selling() -> Result<()> {
    let session = env::var("TARKOV_SESSION").unwrap();
    let t = Tarkov::from_session(&session);

    let profiles = t.get_profiles().await?;
    let profile = profiles
        .into_iter()
        .find(|p| p.info.side != Side::Savage)
        .unwrap();

    let traders = t.get_traders().await?;
    let trader = traders
        .into_iter()
        .find(|t| t.nickname == "Терапевт")
        .unwrap();

    let painkiller = &profile
        .inventory
        .items
        .into_iter()
        .find(|i| i.schema_id == "544fb37f4bdc2dee738b4567")
        .unwrap();

    let _ = t.sell_item(&trader.id, &painkiller.id, 1).await?;

    Ok(())
}
