//! An unofficial client library for the [Escape from Tarkov](https://escapefromtarkov.com) (EFT) API.
//!
//! To get started, login to EFT with `Tarkov::login`, `from_access_token`, or `from_session`.
//! Additionally, on a new session, a profile must be selected with `select_profile` before continuing.
//!
//! Once authenticated, the resulting value can be used to make further API requests.
//!
//! See [Tarkov](struct.Tarkov.html) for a list of available methods.
//!
//! For examples, see the `examples` directory in the source tree.

#![warn(missing_docs)]

use crate::auth::LoginError;
use crate::hwid::generate_hwid;
use crate::profile::ProfileError;
use crate::ragfair::RagfairError;
use crate::trading::TradingError;
use err_derive::Error;
use flate2::read::ZlibDecoder;
use hyper::body::Buf;
use hyper::client::connect::dns::GaiResolver;
use hyper::client::{Client, HttpConnector};
use hyper::Body;
use hyper::Request;
use hyper::{Method, StatusCode};
use hyper_tls::HttpsConnector;
use log::debug;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::io::Read;

const GAME_VERSION: &str = "0.12.5.7295";
const LAUNCHER_VERSION: &str = "10.1.0.1116";
const UNITY_VERSION: &str = "2018.4.13f1";

const LAUNCHER_ENDPOINT: &str = "https://launcher.escapefromtarkov.com";
const PROD_ENDPOINT: &str = "https://prod.escapefromtarkov.com";
const TRADING_ENDPOINT: &str = "https://trading.escapefromtarkov.com";
const RAGFAIR_ENDPOINT: &str = "https://ragfair.escapefromtarkov.com";

mod bad_json;

/// Structs for authentication.
pub mod auth;
/// Structs for game constants API.
pub mod constant;
/// Structs for the Friend API.
pub mod friend;
/// Helper functions for hardware ID.
pub mod hwid;
/// Structs for inventory and items.
pub mod inventory;
/// Flea market search helpers.
pub mod market_filter;
/// Structs for the Profile API.
pub mod profile;
/// Structs for the Flea Market (Ragfair) API.
pub mod ragfair;
/// Structs for the Trading API.
pub mod trading;

/// Common error enum returned by most functions.
#[derive(Debug, Error)]
pub enum Error {
    /// A `std::io` error
    #[error(display = "io error: {}", _0)]
    Io(#[error(source)] std::io::Error),
    /// HTTP request error.
    #[error(display = "http error: {}", _0)]
    Http(#[error(source)] http::Error),
    /// A `hyper` crate error.
    #[error(display = "hyper error: {}", _0)]
    Hyper(#[error(source)] hyper::Error),
    /// A `serde_json` error.
    #[error(display = "json error: {}", _0)]
    Json(#[error(source)] serde_json::error::Error),
    /// Generic non-success response from the API.
    #[error(display = "non-success response from api: {}", _0)]
    Status(StatusCode),
    /// Invalid or missing parameters.
    #[error(display = "invalid or missing login parameters")]
    InvalidParameters,

    /// Unidentified error within the EFT API.
    #[error(display = "unidentified login error with error code: {}", _0)]
    UnknownAPIError(u64),
    /// Not authorized to API or profile is not selected.
    #[error(display = "not authorized or game profile not selected")]
    NotAuthorized,
    /// EFT API is down for maintenance.
    #[error(display = "api is down for maintenance")]
    Maintenance,
    /// Backend error. No other information is given.
    #[error(display = "backend error")]
    BackendError,
    /// Authentication API error.
    #[error(display = "login api error: {}", _0)]
    LoginError(#[error(source)] LoginError),
    /// Profile API error.
    #[error(display = "profile api error: {}", _0)]
    ProfileError(#[error(source)] ProfileError),
    /// Trading API error.
    #[error(display = "trading api error: {}", _0)]
    TradingError(#[error(source)] TradingError),
    /// Ragfair API error.
    #[error(display = "trading api error: {}", _0)]
    RagfairError(#[error(source)] RagfairError),
}

/// `Result` alias type.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
struct ErrorResponse {
    #[serde(rename = "err")]
    code: u64,
    #[serde(rename = "errmsg")]
    message: Option<String>,
}

/// Client for the EFT API.
pub struct Tarkov {
    client: Client<HttpsConnector<HttpConnector<GaiResolver>>, Body>,
    /// Hardware ID
    pub hwid: String,
    /// Session cookie
    pub session: String,
}

impl Tarkov {
    /// Login with email and password.
    pub async fn login(email: &str, password: &str, hwid: &str) -> Result<Self> {
        if email.is_empty() || password.is_empty() || hwid.is_empty() {
            return Err(Error::InvalidParameters);
        }

        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, Body>(https);

        let user = auth::login(&client, email, password, None, &hwid).await?;
        let session = auth::exchange_access_token(&client, &user.access_token, &hwid).await?;

        Ok(Tarkov {
            client,
            hwid: hwid.to_string(),
            session: session.session,
        })
    }

    /// Login with email, password and captcha.
    pub async fn login_with_captcha(
        email: &str,
        password: &str,
        captcha: &str,
        hwid: &str,
    ) -> Result<Self> {
        if email.is_empty() || password.is_empty() || captcha.is_empty() || hwid.is_empty() {
            return Err(Error::InvalidParameters);
        }

        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, Body>(https);

        let user = auth::login(&client, email, password, Some(captcha), &hwid).await?;
        let session = auth::exchange_access_token(&client, &user.access_token, &hwid).await?;

        Ok(Tarkov {
            client,
            hwid: hwid.to_string(),
            session: session.session,
        })
    }

    /// Login with email, password and 2FA code.
    pub async fn login_with_2fa(
        email: &str,
        password: &str,
        code: &str,
        hwid: &str,
    ) -> Result<Self> {
        if email.is_empty() || password.is_empty() || code.is_empty() || hwid.is_empty() {
            return Err(Error::InvalidParameters);
        }

        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, Body>(https);

        let _ = auth::activate_hardware(&client, email, code, &hwid).await?;
        let user = auth::login(&client, email, password, None, &hwid).await?;
        let session = auth::exchange_access_token(&client, &user.access_token, &hwid).await?;

        Ok(Tarkov {
            client,
            hwid: hwid.to_string(),
            session: session.session,
        })
    }

    /// Login with a Bearer token.
    pub async fn from_access_token(access_token: &str, hwid: &str) -> Result<Self> {
        if access_token.is_empty() || hwid.is_empty() {
            return Err(Error::InvalidParameters);
        }

        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, Body>(https);
        let session = auth::exchange_access_token(&client, &access_token, &hwid).await?;

        Ok(Tarkov {
            client,
            hwid: hwid.to_string(),
            session: session.session,
        })
    }

    /// Login with a cookie session (AKA `PHPSESSID`).
    pub fn from_session(session: &str) -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, Body>(https);

        Tarkov {
            client,
            hwid: generate_hwid(),
            session: session.to_string(),
        }
    }

    async fn post_json<S, T>(&self, url: &str, body: &S) -> Result<T>
    where
        S: serde::Serialize + ?Sized + std::fmt::Debug,
        T: DeserializeOwned,
    {
        debug!("Sending request to {} ({:?})", url, body);
        let body = match serde_json::to_string(&body) {
            Ok(body) => Ok(Body::from(if body == "null" {
                "{}".to_string()
            } else {
                body
            })),
            Err(e) => Err(e),
        }?;

        let req = Request::builder()
            .uri(url)
            .method(Method::POST)
            .header("Content-Type", "application/json")
            .header(
                "User-Agent",
                format!(
                    "UnityPlayer/{} (UnityWebRequest/1.0, libcurl/7.52.0-DEV)",
                    UNITY_VERSION
                ),
            )
            .header("App-Version", format!("EFT Client {}", GAME_VERSION))
            .header("X-Unity-Version", UNITY_VERSION)
            .header("Cookie", format!("PHPSESSID={}", self.session))
            .body(body)?;
        let res = self.client.request(req).await?;

        match res.status() {
            StatusCode::OK => {
                let body = hyper::body::to_bytes(res.into_body()).await?;
                let mut decode = ZlibDecoder::new(body.bytes());
                let mut body = String::new();
                decode.read_to_string(&mut body)?;
                debug!("Response: {}", body);

                Ok(serde_json::from_slice::<T>(body.as_bytes())?)
            }
            _ => Err(Error::Status(res.status())),
        }
    }
}

pub(crate) fn handle_error<T: DeserializeOwned>(error: ErrorResponse, ret: Option<T>) -> Result<T> {
    handle_error2(error)?;
    Ok(ret.expect("API returned no errors but `data` is unavailable."))
}

pub(crate) fn handle_error2(error: ErrorResponse) -> Result<()> {
    match error.code {
        0 => Ok(()),
        201 => Err(Error::NotAuthorized)?,
        205 => Err(ProfileError::InvalidUserID)?,
        206 => Err(LoginError::BadLogin)?,
        207 => Err(Error::InvalidParameters)?,
        209 => Err(LoginError::TwoFactorRequired)?,
        211 => Err(LoginError::BadTwoFactorCode)?,
        214 => Err(LoginError::CaptchaRequired)?,
        228 => Err(RagfairError::InvalidBarterItems)?,
        230 => Err(LoginError::RateLimited)?,
        232 => Err(LoginError::WrongMajorVersion)?,
        263 => Err(Error::Maintenance)?,
        1000 => Err(Error::BackendError)?,
        1501 => Err(RagfairError::MaxOfferCount)?,
        1502 => Err(RagfairError::InsufficientTaxFunds)?,
        1507 => Err(RagfairError::OfferNotFound)?,
        1510 => Err(TradingError::BadLoyaltyLevel)?,
        1512 => Err(RagfairError::OfferNotAvailableYet)?,
        1514 => Err(TradingError::TransactionError)?,
        _ => Err(Error::UnknownAPIError(error.code)),
    }
}
