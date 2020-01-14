use crate::{Tarkov, LAUNCHER_ENDPOINT, LAUNCHER_VERSION, Result};
use actix_web::http::StatusCode;
use serde::Serialize;
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

impl Tarkov {
    pub async fn login(&self, email: &str, password: &str, hw_code: &str) -> Result<()> {
        let url = format!("{}/launcher/login?launcherVersion={}&branch=live", LAUNCHER_ENDPOINT, LAUNCHER_VERSION);
        let password = format!("{:x}", md5::compute(&password));
        let req = LoginRequest {
            email,
            pass: &password,
            hw_code,
            captcha: None
        };

        let mut res = self.client.post(url)
            .header("User-Agent", "UnityPlayer/5.6.5p3")
            .send_json(&req)
            .await?;
        let body = res.body().await?;
        let mut decode = ZlibDecoder::new(&body[..]);
        let mut body = String::new();
        decode.read_to_string(&mut body)?;

        println!("{}", body);

        Ok(())

//        match res.status() {
//            StatusCode::OK => Ok(serde_json::from_slice::<T>(&body)?),
//            _ => {}
//        };
    }
}
