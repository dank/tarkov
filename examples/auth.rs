use tarkov::Tarkov;

// TODO: Change my password soon!

#[actix_rt::main]
async fn main() {
    let t = Tarkov::from_email_and_password("***REMOVED***", "***REMOVED***", "***REMOVED***").await.unwrap();
    println!("{:?}", t.session);
    println!("{:?}", t.get_profiles().await);
//    println!("{:?}", t.select_profile("5e11471915aa567697686170").await);
//    println!("{:?}", t.get_friends().await);
//    println!("{:?}", t.keep_alive().await);
//    println!("{:?}", t.get_traders().await);
//    println!("{:?}", t.get_trader("54cb50c76803fa8b248b4571").await);
    println!("{:?}", t.search_market().await);
}
