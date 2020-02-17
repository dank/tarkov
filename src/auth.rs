use crate::{
    handle_error, handle_error2, Error, ErrorResponse, Result, Tarkov, GAME_VERSION,
    LAUNCHER_ENDPOINT, LAUNCHER_VERSION, PROD_ENDPOINT,
};
use flate2::read::ZlibDecoder;
use hyper::body::Buf;
use hyper::client::connect::dns::GaiResolver;
use hyper::client::{Client, HttpConnector};
use hyper::Request;
use hyper::StatusCode;
use hyper::{Body, Method};
use hyper_tls::HttpsConnector;
use log::debug;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct LoginRequest<'a> {
    email: &'a str,
    pass: &'a str,
    hw_code: &'a str,
    captcha: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Auth {
    pub aid: String,
    pub lang: String,
    pub region: Option<String>,
    #[serde(rename = "gameVersion")]
    pub game_version: Option<String>,
    #[serde(rename = "dataCenters")]
    pub data_centers: Vec<String>,
    #[serde(rename = "ipRegion")]
    pub ip_region: String,
    pub token_type: String,
    pub expires_in: u64,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
struct LoginResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    #[serde(default)]
    data: serde_json::Value,
}

/// Login error
#[derive(Debug, err_derive::Error)]
pub enum LoginError {
    /// Bad login, invalid email or password.
    #[error(display = "bad login, wrong email or password.")]
    BadLogin,
    /// 2FA code is required to continue authentication.
    #[error(display = "2fa is required")]
    TwoFactorRequired,
    /// Captcha response is required to continue authentication.
    #[error(display = "captcha is required")]
    CaptchaRequired,
    /// Incorrect 2FA code.
    #[error(display = "incorrect 2FA code")]
    BadTwoFactorCode,
    /// Rate limited after too many bad login attempts.
    #[error(display = "too many login attempts")]
    RateLimited,
}

pub(crate) async fn login(
    client: &Client<HttpsConnector<HttpConnector<GaiResolver>>, Body>,
    email: &str,
    password: &str,
    captcha: Option<&str>,
    hwid: &str,
) -> Result<Auth> {
    let url = format!(
        "{}/launcher/login?launcherVersion={}&branch=live",
        LAUNCHER_ENDPOINT, LAUNCHER_VERSION
    );
    let password = format!("{:x}", md5::compute(&password));

    let body = LoginRequest {
        email,
        pass: &password,
        hw_code: hwid,
        captcha,
    };

    let res: LoginResponse = post_json(client, &url, &body).await?;
    handle_error2(res.error)?;

    Ok(Deserialize::deserialize(res.data)?)
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SecurityLoginRequest<'a> {
    email: &'a str,
    hw_code: &'a str,
    activate_code: &'a str,
}

#[derive(Debug, Deserialize)]
struct SecurityLoginResponse {
    #[serde(flatten)]
    error: ErrorResponse,
}

pub(crate) async fn activate_hardware(
    client: &Client<HttpsConnector<HttpConnector<GaiResolver>>, Body>,
    email: &str,
    code: &str,
    hwid: &str,
) -> Result<()> {
    let url = format!(
        "{}/launcher/hardwareCode/activate?launcherVersion={}",
        LAUNCHER_ENDPOINT, LAUNCHER_VERSION
    );

    let body = SecurityLoginRequest {
        email,
        hw_code: hwid,
        activate_code: code,
    };

    let res: SecurityLoginResponse = post_json(client, &url, &body).await?;
    handle_error2(res.error)
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ExchangeRequest<'a> {
    version: ExchangeVersion<'a>,
    hw_code: &'a str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ExchangeVersion<'a> {
    major: &'a str,
    game: &'a str,
    backend: &'a str,
}

#[derive(Debug, Deserialize)]
struct ExchangeResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Option<Session>,
}

/// Authenticated user session.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Session {
    queued: bool,
    /// Session cookie.
    pub session: String,
}

pub(crate) async fn exchange_access_token(
    client: &Client<HttpsConnector<HttpConnector<GaiResolver>>, Body>,
    access_token: &str,
    hwid: &str,
) -> Result<Session> {
    let url = format!(
        "{}/launcher/game/start?launcherVersion={}&branch=live",
        PROD_ENDPOINT, LAUNCHER_VERSION
    );

    let body = ExchangeRequest {
        version: ExchangeVersion {
            major: GAME_VERSION,
            game: "live",
            backend: "6",
        },
        hw_code: hwid,
    };

    debug!("Sending request to {} ({:?})", url, body);
    let req = Request::builder()
        .uri(url)
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .header("User-Agent", format!("BSG Launcher {}", LAUNCHER_VERSION))
        .header("Authorization", access_token)
        .body(Body::from(serde_json::to_string(&body)?))?;
    let res = client.request(req).await?;

    match res.status() {
        StatusCode::OK => {
            let body = hyper::body::to_bytes(res.into_body()).await?;
            let mut decode = ZlibDecoder::new(body.bytes());
            let mut body = String::new();
            decode.read_to_string(&mut body)?;
            debug!("Response: {}", body);

            let res = serde_json::from_slice::<ExchangeResponse>(body.as_bytes())?;
            handle_error(res.error, res.data)
        }
        _ => Err(Error::Status(res.status())),
    }
}

async fn post_json<S: serde::Serialize + ?Sized + std::fmt::Debug, T: DeserializeOwned>(
    client: &Client<HttpsConnector<HttpConnector<GaiResolver>>, Body>,
    url: &str,
    body: &S,
) -> Result<T> {
    debug!("Sending request to {} ({:?})", url, body);
    let req = Request::builder()
        .uri(url)
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .header("User-Agent", format!("BSG Launcher {}", LAUNCHER_VERSION))
        .body(Body::from(serde_json::to_string(&body)?))?;
    let res = client.request(req).await?;

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

impl Tarkov {
    /// Keep the current session alive. Session expires after 30 seconds of idling.
    pub async fn keep_alive(&self) -> Result<()> {
        let url = format!("{}/client/game/keepalive", PROD_ENDPOINT);
        let res: ErrorResponse = self.post_json(&url, &{}).await?;

        match res.code {
            0 => Ok(()),
            _ => Err(Error::UnknownAPIError(res.code)),
        }
    }
}
