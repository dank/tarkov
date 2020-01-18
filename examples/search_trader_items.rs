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

    // Fetch the translation table because trader names are in Russian.
    let locale = t.get_i18n("en").await?;

    // Find trader by name.
    let traders = t.get_traders().await?;
    let trader = traders
        .into_iter()
        .find(|t| locale.traders.get(&t.id).unwrap().nickname == "Therapist")
        .unwrap();

    // Fetch all items to map item ID to item name/descriptor.
    let items = t.get_all_items().await?;

    // Find trader's item by name
    let trader_items = t.get_trader_items(&trader.id).await?;
    let painkiller = trader_items
        .into_iter()
        .find(|i| {
            // Therapist sells painkillers for Roubles or 2 matches. Finding the barter deal.
            let item = items.get(&i.schema_id).unwrap();
            item.name == "painkiller" && i.price.get(0).unwrap().count == 2.0
        })
        .unwrap();

    println!("{:#?}", painkiller);

    Ok(())
}
