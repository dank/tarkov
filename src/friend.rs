use crate::{handle_error, ErrorResponse, Result, Tarkov, PROD_ENDPOINT};

use crate::profile::Side;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FriendResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Option<Friends>,
}

/// Friend list
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Friends {
    /// Friends
    pub friends: Vec<Friend>,
    /// Muted friend IDs
    pub ignore: Vec<String>,
    /// ?
    pub in_ignore_list: Vec<String>,
}

/// Friend
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Friend {
    /// Friend ID
    #[serde(rename = "_id")]
    pub id: String,
    /// Friend info
    pub info: Info,
}

/// Friend info
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Info {
    /// Friend nickname
    pub nickname: String,
    /// Friend side
    pub side: Side,
    /// Friend level
    pub level: u64,
    /// ?
    pub member_category: String,
}

impl Tarkov {
    /// Get a list of account's friends.
    pub async fn get_friends(&self) -> Result<Friends> {
        let url = format!("{}/client/friend/list", PROD_ENDPOINT);
        let res: FriendResponse = self.post_json(&url, &{}).await?;

        handle_error(res.error, res.data)
    }
}
