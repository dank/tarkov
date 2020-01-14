use tarkov::Tarkov;

// TODO: Change my password soon!

#[actix_rt::main]
async fn main() {
    let t = Tarkov::new("***REMOVED***", "***REMOVED***", "#1-17620e25e76ab3031bd59ee937a463b289cb5698:a0c1ec9e624f0e6732e19221d0cf7cd75f6fb570:9558bf53513b1dc06f7f2fd78883cd0326f00e42-55e722c489976f9349796e35f87594d3569872d9-5f929b6d87f35ef70f3563d4e02e7ff438f67c3a-8c60765c988e992f0c858626407fd97fe2392799-75fd29e7ea40550d9bab6bf0fea1fd9cc8cf53e1-e1bc65a216325f0ad0db8518fa299db2").await.unwrap();
    println!("{:?}", t.session);
    println!("{:?}", t.get_profiles().await);
    println!("{:?}", t.select_profile("5e11471915aa567697686170").await);
    println!("{:?}", t.get_friends().await);
    println!("{:?}", t.keep_alive().await);
    println!("{:?}", t.get_traders().await);
    println!("{:?}", t.get_trader("54cb50c76803fa8b248b4571").await);
}
