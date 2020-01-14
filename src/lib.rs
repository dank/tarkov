use actix_web::client::Client;
use err_derive::Error;

const LAUNCHER_VERSION: &str = "0.9.1.935";

const LAUNCHER_ENDPOINT: &str = "https://launcher.escapefromtarkov.com";
const PROD_ENDPOINT: &str = "https://prod.escapefromtarkov.com";

mod auth;

#[derive(Debug, Error)]
pub enum Error {
    #[error(display = "io error: {}", _0)]
    Io(#[error(source)] std::io::Error),
    #[error(display = "send request error: {}", _0)]
    SendRequestError(#[error(from)] actix_web::client::SendRequestError),
    #[error(display = "payload error: {}", _0)]
    PayloadError(#[error(from)] actix_web::client::PayloadError),
    #[error(display = "json error: {}", _0)]
    Json(#[error(source)] serde_json::error::Error),
    #[error(display = "???")]
    UnknownError, // TODO
}

type Result<T> = std::result::Result<T, Error>;

pub struct Tarkov {
    client: Client,
}

impl Tarkov {
    pub fn new() -> Self {
        Tarkov {
            client: Client::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
