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

    // Get Therapist
    let traders = t.get_traders().await?;
    let trader = traders
        .into_iter()
        .find(|t| t.id == "54cb57776803fa99248b456e")
        .unwrap();

    // Therapist wants 3990₽ for 1 painkiller.
    let painkiller = t
        .get_trader_items(&trader.id)
        .await?
        .into_iter()
        .find(|i| i.id == "5e064f5deb009468d90baef7")
        .unwrap();

    // Get Rouble item ID from my inventory
    let rouble = &profile
        .inventory
        .items
        .into_iter()
        .find(|i| i.schema_id == "5449016a4bdc2d6f028b456f")
        .unwrap();

    // Trade item
    t.trade_item(
        &trader.id,
        &painkiller.id,
        1,
        vec![BarterItem {
            id: rouble.id.clone(),
            count: 3990.0,
        }],
    )
    .await?;

    Ok(())
}
