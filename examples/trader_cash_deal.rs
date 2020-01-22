use tarkov::inventory::BarterItem;
use tarkov::profile::Side;
use tarkov::{Error, Tarkov};

#[actix_rt::main]
async fn main() -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "tarkov=debug");
    env_logger::init();

    let t = Tarkov::from_session("3e463058dd4884ab0c4a6035dc56066b");

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

    // Therapist wants 3990₽ for 1 painkiller.
    let painkiller = t
        .get_trader_items(&trader.id)
        .await?
        .into_iter()
        .find(|i| i.id == "5e064f5deb009468d90baef7")
        .unwrap();

    println!(">> {:?}", painkiller);

    // Get Rouble item ID from my inventory
    let rouble = &profile
        .inventory
        .items
        .into_iter()
        .find(|i| i.schema_id == "5449016a4bdc2d6f028b456f")
        .unwrap();

    // Trade item
    println!(
        "{:?}",
        t.trade_item(
            &trader.id,
            "5e064f5deb009468d90baef1",
            1,
            &[BarterItem {
                id: rouble.id.to_owned(),
                count: painkiller.price.get(0).unwrap().count, // Assume price is 3990₽.
            }],
        )
        .await
    );

    Ok(())
}
