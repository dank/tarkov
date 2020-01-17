use tarkov::profile::Side;
use tarkov::trading::{BarterItem, Item};
use tarkov::{Error, Tarkov};

#[actix_rt::main]
async fn main() -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "tarkov=debug");
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
        .find(|t| t.id == "54cb57776803fa99248b456e")
        .unwrap();

    // Therapist wants 2 matches for 1 painkiller.
    let painkiller = t
        .get_trader_items(&trader.id)
        .await?
        .into_iter()
        .find(|i| i.id == "5e064f5deb009468d90baf01")
        .unwrap();

    // Get the IDs of 2 painkillers from my inventory
    let barter_items = &profile
        .inventory
        .items
        .clone()
        .into_iter()
        .filter(|i| i.schema_id == painkiller.price.get(0).unwrap().schema_id)
        .map(|i| BarterItem {
            id: i.id,
            count: 1.0,
        })
        .collect::<Vec<BarterItem>>()[..1];

    // Trade item
    t.trade_item(&trader.id, &painkiller.id, 1, barter_items.to_vec())
        .await?;

    Ok(())
}
