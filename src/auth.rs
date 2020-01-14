use crate::{Tarkov, LAUNCHER_ENDPOINT, LAUNCHER_VERSION, Result, Error};
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};
use std::str::from_utf8;
use std::io::Read;
use flate2::read::ZlibDecoder;

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
    aid: String,
    lang: String,
    region: String,
    #[serde(rename = "gameVersion")]
    game_version: String,
    #[serde(rename = "dataCenters")]
    data_centers: Vec<String>,
    #[serde(rename = "ipRegion")]
    // XXX: Not a mistake. Tarkov developers are inconsistent.
    ip_region: String,
    token_type: String,
    expires_in: u64,
    access_token: String,
    refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    #[serde(rename = "err")]
    error_code: u8,
    #[serde(rename = "errmsg")]
    error_message: Option<String>,
    data: LoginData
}

impl Tarkov {
    pub async fn login(&self, email: &str, password: &str, hw_code: &str) -> Result<LoginResponse> {
        let url = format!("{}/launcher/login?launcherVersion={}&branch=live", LAUNCHER_ENDPOINT, LAUNCHER_VERSION);
        let password = format!("{:x}", md5::compute(&password));
        let req = LoginRequest {
            email,
            pass: &password,
            hw_code,
            captcha: None,
        };

        let mut res = self.client.post(url)
            .header("User-Agent", "UnityPlayer/5.6.5p3")
            .send_json(&req)
            .await?;
        let body = res.body().await?;
        let mut decode = ZlibDecoder::new(&body[..]);
        let mut body = String::new();
        decode.read_to_string(&mut body)?;

        match res.status() {
            StatusCode::OK => return Ok(serde_json::from_slice::<LoginResponse>(body.as_bytes())?),
            _ => {}
        };

        Err(Error::UnknownError)
    }

    // TODO: Implement refresh_tokens.
}
