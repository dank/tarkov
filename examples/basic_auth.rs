use tarkov::{Error, Tarkov};
use tarkov::hwid::generate_hwid;

#[actix_rt::main]
async fn main() -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "tarkov=info");
    env_logger::init();

    let t = Tarkov::from_email_and_password("me@example.com", "password", generate_hwid().as_str()).await?;
    println!("{}", t.session);

    let t = Tarkov::from_access_token("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...", generate_hwid().as_str()).await?;
    println!("{}", t.session);

    let t = Tarkov::from_session("e1bc65a216325f0ad0db8518fa299db1").await?;
    println!("{}", t.session);

    Ok(())
}