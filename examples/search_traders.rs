use tarkov::{Error, Tarkov};
use tarkov::profile::Side;

#[actix_rt::main]
async fn main() -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "tarkov=info");
    env_logger::init();

    let t = Tarkov::from_session("e1bc65a216325f0ad0db8518fa299db2").await?;

    // Find and select PMC profile to complete login.
    let profiles = t.get_profiles().await?;
    let profile = profiles.into_iter().find(|p| p.info.side != Side::Savage).unwrap();
    t.select_profile(&profile.id).await?;

    // Fetch the translation table because trader names are in Russian.
    let locale = t.get_i18n("en").await?;

    // Find trader by name.
    let trader_name = "Mechanic";
    let traders = t.get_traders().await?;
    let trader = traders.into_iter().find(|t| locale.traders.get(&t.id).unwrap().nickname == trader_name).unwrap();

    println!("{:#?}", trader);

    Ok(())
}