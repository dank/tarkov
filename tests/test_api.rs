use tarkov::Tarkov;

const SESSION: &str = env!("TARKOV_SESSION");

#[tokio::test]
async fn test_profile() {
    let t = Tarkov::from_session(SESSION);
    t.get_profiles().await.unwrap();
    t.select_profile()
}
