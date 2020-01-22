use tarkov::profile::Side;
use tarkov::{Error, Tarkov};

#[actix_rt::main]
async fn main() -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "tarkov=debug");
    env_logger::init();

    let t = Tarkov::from_session("da3902b29e442da4972f8ce499834ee7");

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
    println!("{:?}", t.sell_item(&trader.id, &painkiller.id, 1).await);

    Ok(())
}
