use tarkov::profile::Side;
use tarkov::{Error, Tarkov};

#[tokio::main]
async fn main() -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "tarkov=info");
    env_logger::init();

    let t = Tarkov::from_session("e1bc65a216325f0ad0db8518fa299db2");

    // Find and select PMC profile to complete login.
    let profiles = t.get_profiles().await?;
    let profile = profiles
        .into_iter()
        .find(|p| p.info.side != Side::Savage)
        .unwrap();
    t.select_profile(&profile.id).await?;

    let _ = t.get_items().await?;
    let _ = t.get_item_prices().await?;
    let _ = t.get_locations().await?;
    let _ = t.get_weather().await?;
    let _ = t.get_i18n("en").await?;
    let _ = t.get_friends().await?;
    let _ = t.get_traders().await?;
    let _ = t.get_trader_items("54cb50c76803fa8b248b4571").await?;

    Ok(())
}
