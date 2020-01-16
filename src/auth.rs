use crate::{
    Error, ErrorResponse, Result, Tarkov, GAME_VERSION, LAUNCHER_ENDPOINT, LAUNCHER_VERSION,
    PROD_ENDPOINT,
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

/// Authenticated user.
#[derive(Debug, Deserialize)]
pub(crate) struct Auth {
    // TODO: Why did I use default here?
    #[serde(default)]
    pub aid: String,
    #[serde(default)]
    pub lang: String,
    #[serde(default)]
    pub region: String,
    #[serde(default)]
    #[serde(rename = "gameVersion")]
    pub game_version: String,
    #[serde(default)]
    #[serde(rename = "dataCenters")]
    pub data_centers: Vec<String>,
    #[serde(default)]
    #[serde(rename = "ipRegion")]
    pub ip_region: String,
    // XXX: Not a mistake. Tarkov developers are inconsistent.
    #[serde(default)]
    pub token_type: String,
    #[serde(default)]
    pub expires_in: u64,
    #[serde(default)]
    pub access_token: String,
    #[serde(default)]
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
struct LoginResponse {
    #[serde(rename = "err")]
    error_code: u8,
    #[serde(rename = "errmsg")]
    error_message: Option<String>,
    data: Option<Auth>,
}

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
    Captcha,
}

// TODO: Implement refresh_tokens.

pub(crate) async fn login(
    client: &Client,
    email: &str,
    password: &str,
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
        captcha: None,
    };

    debug!("Sending request to {}...", url);
    let mut res = client
        .post(url)
        .header("User-Agent", format!("BSG Launcher {}", LAUNCHER_VERSION))
        .send_json(&req)
        .await?;
    let body = res.body().await?;
    let mut decode = ZlibDecoder::new(&body[..]);
    let mut body = String::new();
    decode.read_to_string(&mut body)?;
    debug!("Response: {}", body);

    match res.status() {
        StatusCode::OK => {
            let res = serde_json::from_slice::<LoginResponse>(body.as_bytes())?;
            match res.error_code {
                0 => Ok(res
                    .data
                    .expect("API returned no errors but `data` is unavailable.")),
                207 => Err(LoginError::MissingParameters)?,
                209 => Err(LoginError::TwoFactorRequired)?,
                214 => Err(LoginError::Captcha)?,
                _ => Err(Error::UnknownAPIError(res.error_code)),
            }
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
    #[serde(rename = "err")]
    error_code: u8,
    #[serde(rename = "errmsg")]
    error_message: Option<String>,
    data: Option<Session>,
}

/// Authenticated user session.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Session {
    pub queued: bool,
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
    let body = res.body().await?;
    let mut decode = ZlibDecoder::new(&body[..]);
    let mut body = String::new();
    decode.read_to_string(&mut body)?;
    debug!("Response: {}", body);

    match res.status() {
        StatusCode::OK => {
            let res = serde_json::from_slice::<ExchangeResponse>(body.as_bytes())?;
            match res.error_code {
                0 => Ok(res
                    .data
                    .expect("API returned no errors but `data` is unavailable.")),
                207 => Err(LoginError::MissingParameters)?,
                209 => Err(LoginError::TwoFactorRequired)?,
                214 => Err(LoginError::Captcha)?,
                _ => Err(Error::UnknownAPIError(res.error_code)),
            }
        }
        _ => Err(Error::Status(res.status())),
    }
}

impl Tarkov {
    /// Keep the current session alive
    pub async fn keep_alive(&self) -> Result<()> {
        let url = format!("{}/client/game/keepalive", PROD_ENDPOINT);
        let res: ErrorResponse = self.post_json(&url, &{}).await?;

        match res.code {
            0 => Ok(()),
            _ => Err(Error::UnknownAPIError(res.code)),
        }
    }
}
