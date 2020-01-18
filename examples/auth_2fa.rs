use tarkov::auth::LoginError;
use tarkov::hwid::generate_hwid;
use tarkov::{Error, Tarkov};

#[actix_rt::main]
async fn main() -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "tarkov=info");
    env_logger::init();

    let email = "me@example.com";
    let password = "password";
    let hwid = generate_hwid();

    let t = match Tarkov::login(email, password, &hwid).await {
        Ok(t) => Ok(t),
        Err(Error::LoginError(e)) => match e {
            // 2FA required!
            LoginError::TwoFactorRequired => {
                // Get 2FA from email (or generate TOTP) then continue...
                let code = "XYZ";
                Tarkov::login_with_2fa(email, password, code, &hwid).await
            }
            _ => Err(e)?,
        },
        Err(e) => Err(e),
    }?;

    println!("{}", t.session);

    Ok(())
}
