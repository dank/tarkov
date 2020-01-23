use tarkov::auth::LoginError;
use tarkov::hwid::generate_hwid;
use tarkov::{Error, Tarkov};

#[tokio::main]
async fn main() -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "tarkov=info");
    env_logger::init();

    let email = "me@example.com";
    let password = "password";
    let hwid = generate_hwid();

    let t = match Tarkov::login(email, password, &hwid).await {
        Ok(t) => Ok(t),
        Err(Error::LoginError(e)) => match e {
            // Captcha required!
            LoginError::CaptchaRequired => {
                // Solve captcha here and try again
                let captcha = "03AOLTBLQ952pO-qQYPeLr53N5nK9Co14iXyCp...";
                Tarkov::login_with_captcha(email, password, captcha, &hwid).await
            }
            _ => Err(e)?,
        },
        Err(e) => Err(e),
    }?;

    println!("{}", t.session);

    Ok(())
}
