use crate::{Tarkov, Result, PROD_ENDPOINT, UNITY_VERSION, GAME_VERSION, ErrorResponse, Error};
use flate2::read::ZlibDecoder;
use std::io::Read;
use serde::Deserialize;
use actix_web::http::StatusCode;

#[derive(Debug, Deserialize)]
struct FriendResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Friends
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Friends {
    friends: Vec<Friend>
    // TODO: Types unknown
    // ignore: []
    // in_ignore_list: []
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Friend {
    #[serde(rename = "_id")]
    id: String,
    info: FriendInfo
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FriendInfo {
    nickname: String,
    side: String,
    level: u64,
    member_category: String
}

impl Tarkov {
    pub async fn get_friends(&self) -> Result<Vec<Friend>> {
        let url = format!("{}/client/friend/list", PROD_ENDPOINT);
        let mut res = self.client
            .post(url)
            .header("User-Agent", format!("UnityPlayer/{} (UnityWebRequest/1.0, libcurl/7.52.0-DEV)", UNITY_VERSION))
            .header("App-Version", format!("EFT Client {}", GAME_VERSION))
            .header("X-Unity-Version", UNITY_VERSION)
            .header("Cookie", format!("PHPSESSID={}", self.session))
            .send_json(&{})
            .await?;

        let body = res.body().await?;
        let mut decode = ZlibDecoder::new(&body[..]);
        let mut body = String::new();
        decode.read_to_string(&mut body)?;

        match res.status() {
            StatusCode::OK => {
                let res = serde_json::from_slice::<FriendResponse>(body.as_bytes())?;
                match res.error.error_code {
                    0 => Ok(res.data.friends),
                    _ => Err(Error::UnknownAPIError(res.error.error_code)),
                }
            }
            _ => Err(Error::Status(res.status())),
        }
    }
}
