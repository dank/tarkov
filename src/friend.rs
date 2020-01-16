use crate::{ErrorResponse, Result, Tarkov, PROD_ENDPOINT};

use crate::profile::Side;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FriendResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Option<FriendResult>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FriendResult {
    pub friends: Vec<Friend>,
    // ignore: []
    // in_ignore_list: []
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Friend {
    #[serde(rename = "_id")]
    pub id: String,
    pub info: Info,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
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

        self.handle_error(
            res.error,
            res.data
                .expect("API returned no errors but `data` is unavailable."),
        )
    }
}
