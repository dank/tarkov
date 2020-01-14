use crate::{
    Error, Result, GAME_VERSION, LAUNCHER_ENDPOINT, LAUNCHER_VERSION, PROD_ENDPOINT,
};
use actix_web::client::Client;
use actix_web::http::StatusCode;
use flate2::read::ZlibDecoder;
use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct LoginRequest<'a> {
    email: &'a str,
    pass: &'a str,
    hw_code: &'a str,
    captcha: Option<()>, // Always null
}

#[derive(Debug, Deserialize)]
pub struct LoginData {
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
    data: Option<LoginData>,
}

#[derive(Debug, err_derive::Error)]
pub enum LoginError {
    #[error(display = "unidentified login error with error code: {}", _0)]
    UnknownError(u8),
    #[error(display = "invalid or missing login parameters")]
    MissingParameters,
    #[error(display = "2fa is not supported yet")]
    TwoFactorRequired,
    #[error(display = "captcha is not supported yet")]
    Captcha,
}

// TODO: Implement refresh_tokens.

pub async fn login(client: &Client, email: &str, password: &str, hwid: &str) -> Result<LoginData> {
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

    let mut res = client
        .post(url)
        .header("User-Agent", format!("BSG Launcher {}", LAUNCHER_VERSION))
        .send_json(&req)
        .await?;
    let body = res.body().await?;
    let mut decode = ZlibDecoder::new(&body[..]);
    let mut body = String::new();
    decode.read_to_string(&mut body)?;

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
                _ => Err(LoginError::UnknownError(res.error_code))?,
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
    data: Option<ExchangeData>,
}

#[derive(Debug, Deserialize)]
pub struct ExchangeData {
    pub queued: bool,
    pub session: String,
}

pub async fn exchange_access_token(client: &Client, access_token: &str, hwid: &str) -> Result<ExchangeData> {
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

    match res.status() {
        StatusCode::OK => {
            let res = serde_json::from_slice::<ExchangeResponse>(body.as_bytes())?;
            match res.error_code {
                0 => Ok(res
                    .data
                    .expect("API returned no errors but `data` is unavailable.")),
                _ => Err(LoginError::UnknownError(res.error_code))?,
            }
        }
        _ => Err(Error::Status(res.status())),
    }
}
