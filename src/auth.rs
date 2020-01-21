use crate::{
    handle_error, Error, ErrorResponse, Result, Tarkov, GAME_VERSION, LAUNCHER_ENDPOINT,
    LAUNCHER_VERSION, PROD_ENDPOINT,
};
use actix_web::client::Client;
use actix_web::http::StatusCode;
use flate2::read::ZlibDecoder;
use log::debug;
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
    data: Option<Auth>,
}

/// Login error
#[derive(Debug, err_derive::Error)]
pub enum LoginError {
    /// Invalid or missing parameters.
    #[error(display = "invalid or missing login parameters")]
    MissingParameters,
    /// 2FA code is required to continue authentication.
    #[error(display = "2fa is required")]
    TwoFactorRequired,
    /// Captcha response is required to continue authentication.
    #[error(display = "captcha is required")]
    CaptchaRequired,
    /// Incorrect 2FA code.
    #[error(display = "incorrect 2FA code")]
    BadTwoFactorCode,
}

pub(crate) async fn login(
    client: &Client,
    email: &str,
    password: &str,
    captcha: Option<&str>,
    hwid: &str,
) -> Result<Auth> {
    if email.is_empty() || password.is_empty() || hwid.is_empty() {
        return Err(LoginError::MissingParameters)?;
    }

    let url = format!(
        "{}/launcher/login?launcherVersion={}&branch=live",
        LAUNCHER_ENDPOINT, LAUNCHER_VERSION
    );
    let password = format!("{:x}", md5::compute(&password));
    let req = LoginRequest {
        email,
        pass: &password,
        hw_code: hwid,
        captcha,
    };

    debug!("Sending request to {}...", url);
    let mut res = client
        .post(url)
        .header("User-Agent", format!("BSG Launcher {}", LAUNCHER_VERSION))
        .send_json(&req)
        .await?;

    match res.status() {
        StatusCode::OK => {
            let body = res.body().await?;
            let mut decode = ZlibDecoder::new(&body[..]);
            let mut body = String::new();
            decode.read_to_string(&mut body)?;
            debug!("Response: {}", body);

            let res = serde_json::from_slice::<LoginResponse>(body.as_bytes())?;
            handle_error(res.error, res.data)
        }
        _ => Err(Error::Status(res.status())),
    }
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
    client: &Client,
    email: &str,
    code: &str,
    hwid: &str,
) -> Result<()> {
    if email.is_empty() || code.is_empty() || hwid.is_empty() {
        return Err(LoginError::MissingParameters)?;
    }

    let url = format!(
        "{}/launcher/hardwareCode/activate?launcherVersion={}",
        LAUNCHER_ENDPOINT, LAUNCHER_VERSION
    );
    let req = SecurityLoginRequest {
        email,
        hw_code: hwid,
        activate_code: code,
    };

    debug!("Sending request to {}...", url);
    let mut res = client
        .post(url)
        .header("User-Agent", format!("BSG Launcher {}", LAUNCHER_VERSION))
        .send_json(&req)
        .await?;

    match res.status() {
        StatusCode::OK => {
            let body = res.body().await?;
            let mut decode = ZlibDecoder::new(&body[..]);
            let mut body = String::new();
            decode.read_to_string(&mut body)?;
            debug!("Response: {}", body);

            let res = serde_json::from_slice::<SecurityLoginResponse>(body.as_bytes())?;
            handle_error(res.error, Some(()))
        }
        _ => Err(Error::Status(res.status())),
    }
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
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Session {
    queued: bool,
    /// Session cookie.
    pub session: String,
}

pub(crate) async fn exchange_access_token(
    client: &Client,
    access_token: &str,
    hwid: &str,
) -> Result<Session> {
    let url = format!(
        "{}/launcher/game/start?launcherVersion={}&branch=live",
        PROD_ENDPOINT, LAUNCHER_VERSION
    );
    let req = ExchangeRequest {
        version: ExchangeVersion {
            major: GAME_VERSION,
            game: "live",
            backend: "6",
        },
        hw_code: hwid,
    };

    debug!("Sending request to {}...", url);
    let mut res = client
        .post(url)
        .header("User-Agent", format!("BSG Launcher {}", LAUNCHER_VERSION))
        .bearer_auth(access_token)
        .send_json(&req)
        .await?;

    match res.status() {
        StatusCode::OK => {
            let body = res.body().await?;
            let mut decode = ZlibDecoder::new(&body[..]);
            let mut body = String::new();
            decode.read_to_string(&mut body)?;
            debug!("Response: {}", body);

            let res = serde_json::from_slice::<ExchangeResponse>(body.as_bytes())?;
            handle_error(res.error, res.data)
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
