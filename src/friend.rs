use crate::{Tarkov, Result, PROD_ENDPOINT, UNITY_VERSION, GAME_VERSION, ErrorResponse, Error};
use flate2::read::ZlibDecoder;
use std::io::Read;
use serde::Deserialize;
use actix_web::http::StatusCode;

#[derive(Debug, Deserialize)]
struct FriendResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Friends,
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
    info: FriendInfo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FriendInfo {
    nickname: String,
    side: String,
    level: u64,
    member_category: String,
}

impl Tarkov {
    pub async fn get_friends(&self) -> Result<Vec<Friend>> {
        let url = format!("{}/client/friend/list", PROD_ENDPOINT);
        let res: FriendResponse = self.post_json(&url, &{}).await?;

        match res.error.code {
            0 => Ok(res.data.friends),
            _ => Err(Error::UnknownAPIError(res.error.code)),
        }
    }
}
