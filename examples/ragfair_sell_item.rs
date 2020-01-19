use tarkov::profile::Side;
use tarkov::ragfair::Requirement;
use tarkov::{Error, Tarkov};

#[actix_rt::main]
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

    // Get the painkiller item ID from my inventory
    let painkiller = &profile
        .inventory
        .items
        .into_iter()
        .find(|i| i.schema_id == "544fb37f4bdc2dee738b4567")
        .unwrap();

    // Get market price of painkiller
    let price = t.get_item_price(&painkiller.schema_id).await?;

    // Put the painkiller on market for average price
    t.offer_item(
        &[&painkiller.id],
        &[Requirement {
            schema_id: "5449016a4bdc2d6f028b456f".to_string(),
            count: price.avg.round(),
        }],
        false,
    )
    .await?;

    Ok(())
}
