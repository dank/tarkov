use crate::{Error, ErrorResponse, Result, Tarkov, GAME_VERSION, PROD_ENDPOINT, UNITY_VERSION};
use actix_web::http::StatusCode;
use flate2::read::ZlibDecoder;
use serde::Deserialize;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct FriendResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Option<Friends>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Friends {
    pub friends: Vec<Friend>,
    // TODO: Types unknown
    // ignore: []
    // in_ignore_list: []
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Friend {
    #[serde(rename = "_id")]
    pub id: String,
    pub info: Info,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Info {
    pub nickname: String,
    pub side: String,
    pub level: u64,
    pub member_category: String,
}

impl Tarkov {
    /// Get a list of your friends.
    pub async fn get_friends(&self) -> Result<Vec<Friend>> {
        let url = format!("{}/client/friend/list", PROD_ENDPOINT);
        let res: FriendResponse = self.post_json(&url, &{}).await?;

        match res.error.code {
            0 => Ok(res
                .data
                .expect("API returned no errors but `data` is unavailable.")
                .friends),
            _ => Err(Error::UnknownAPIError(res.error.code)),
        }
    }
}
