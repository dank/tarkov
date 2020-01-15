use crate::{Error, ErrorResponse, Result, Tarkov, PROD_ENDPOINT};

use serde::Deserialize;
use crate::profile::Side;

#[derive(Debug, Deserialize)]
struct FriendResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Option<FriendResult>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FriendResult {
    pub friends: Vec<Friend>,
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
    pub side: Side,
    pub level: u64,
    pub member_category: String,
}

impl Tarkov {
    /// Get a list of your friends.
    pub async fn get_friends(&self) -> Result<FriendResult> {
        let url = format!("{}/client/friend/list", PROD_ENDPOINT);
        let res: FriendResponse = self.post_json(&url, &{}).await?;

        match res.error.code {
            0 => Ok(res
                .data
                .expect("API returned no errors but `data` is unavailable.")),
            _ => Err(Error::UnknownAPIError(res.error.code)),
        }
    }
}
