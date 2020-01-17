use tarkov::profile::Side;
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

    // Get Therapist
    let traders = t.get_traders().await?;
    let trader = traders
        .into_iter()
        .find(|t| t.nickname == "Терапевт")
        .unwrap();

    // Get painkiller item ID from my inventory
    let painkiller = &profile
        .inventory
        .items
        .into_iter()
        .find(|i| i.schema_id == "544fb37f4bdc2dee738b4567")
        .unwrap();

    // Trade item
    t.sell_item(
        &trader.id,
        &painkiller.id,
        1,
    )
    .await?;

    Ok(())
}
