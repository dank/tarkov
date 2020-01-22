use crate::{handle_error, ErrorResponse, Result, Tarkov, PROD_ENDPOINT};

use crate::bad_json::deserialize_integer_to_string;
use crate::ragfair::Offer;
use crate::trading::Item;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub(crate) struct MoveItemRequest<'a, T> {
    pub(crate) data: &'a [T],
    pub(crate) tm: u64,
}

#[derive(Debug, Deserialize)]
struct ProfileResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Option<Vec<Profile>>,
}

/// Profile
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Profile {
    /// Profile ID
    #[serde(rename = "_id")]
    pub id: String,
    /// ?
    #[serde(rename = "aid")]
    pub aid: u64,
    /// SCAV profile ID
    #[serde(rename = "savage")]
    pub savage: Option<String>,
    /// Profile info
    pub info: Info,
    /// Character customization
    pub customization: Customization,
    /// Character health
    pub health: Health,
    /// Inventory
    pub inventory: Inventory,
    /// Skills
    pub skills: Skills,
    /// Stats
    pub stats: Stats,
    /// Encyclopedia
    pub encyclopedia: HashMap<String, bool>,
    /// ?
    pub condition_counters: ConditionCounters,
    /// ?
    pub backend_counters: HashMap<String, BackendCounter>,
    /// Insured items
    pub insured_items: Vec<InsuredItem>,
    /// Unimplemented type
    pub hideout: serde_json::Value,
    /// Unknown type
    pub notes: serde_json::Value,
    /// Bonuses?
    pub bonuses: Vec<Bonus>,
    /// Active quests
    pub quests: Vec<Quest>,
    /// Flea market stats
    #[serde(rename = "RagfairInfo")]
    pub ragfair: Ragfair,
    /// Unknown type
    pub trader_standings: serde_json::Value,
    /// Item wishlist
    #[serde(rename = "WishList")]
    pub wishlist: Vec<String>,
}

/// Profile info
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Info {
    /// Profile nickname
    pub nickname: String,
    /// Profile lowercase nickname
    pub lower_nickname: String,
    /// Profile side
    pub side: Side,
    /// Profile voice
    pub voice: String,
    /// Profile level
    pub level: u64,
    /// Profile experience points
    pub experience: u64,
    /// Profile registration timestamp
    pub registration_date: u64,
    /// Game version
    pub game_version: String,
    /// Profile type
    pub account_type: u64,
    /// ?
    #[serde(deserialize_with = "deserialize_integer_to_string")]
    pub member_category: String,
    /// ?
    #[serde(rename = "lockedMoveCommands")]
    pub locked_move_commands: bool,
    /// SCAV cooldown timestamp
    pub savage_lock_time: u64,
    /// Last time played as SCAV
    pub last_time_played_as_savage: u64,
    /// Profile settings
    pub settings: Settings,
    /// ?
    pub need_wipe: bool,
    /// ?
    pub global_wipe: bool,
    /// Profile nickname change timestamp
    pub nickname_change_date: u64,
    /// Unknown type
    pub bans: serde_json::Value,
}

/// Faction, team or side.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum Side {
    /// BEAR faction
    Bear,
    /// USEC faction
    Usec,
    /// SCAV
    Savage,
}

/// Profile settings
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Settings {
    /// ?
    pub role: Option<String>,
    /// ?
    pub bot_difficulty: Option<String>,
    /// ?
    pub experience: Option<i64>,
}

/// Player customization
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Customization {
    /// Head customization
    pub head: String,
    /// Body customization
    pub body: String,
    /// Feet customization
    pub feet: String,
    /// Hands customization
    pub hands: String,
}

/// Player health
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Health {
    /// Hydration level
    pub hydration: HealthLevel,
    /// Energy level
    pub energy: HealthLevel,
    /// Body parts health
    pub body_parts: BodyParts,
    /// ?
    pub update_time: u64,
}

/// Health level
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct HealthLevel {
    /// Current health
    pub current: f64,
    /// Maximum health
    pub maximum: f64,
}

/// Body health
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BodyParts {
    /// Head health
    pub head: Head,
    /// Chest health
    pub chest: Chest,
    /// Stomach health
    pub stomach: Stomach,
    /// Left arm health
    pub left_arm: LeftArm,
    /// Right arm health
    pub right_arm: RightArm,
    /// Left leg health
    pub left_leg: LeftLeg,
    /// Right arm health
    pub right_leg: RightLeg,
}

/// Head health
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Head {
    /// Health
    pub health: HealthLevel,
}

/// Chest health
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Chest {
    /// Health
    pub health: HealthLevel,
}

/// Stomach health
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Stomach {
    /// Health
    pub health: HealthLevel,
}

/// Left arm health
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LeftArm {
    /// Health
    pub health: HealthLevel,
}

/// Right arm health
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RightArm {
    /// Health
    pub health: HealthLevel,
}

/// Left leg health
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LeftLeg {
    /// Health
    pub health: HealthLevel,
}

/// Right leg health
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RightLeg {
    /// Health
    pub health: HealthLevel,
}

/// Profile inventory
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Inventory {
    /// Items
    pub items: Vec<Item>,
    /// Equipment
    pub equipment: String,
    /// Stash
    pub stash: Option<String>,
    /// ?
    pub quest_raid_items: String,
    /// ?
    pub quest_stash_items: String,
    /// Unknown type
    pub fast_panel: serde_json::Value,
}

/// Player skills
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Skills {
    /// Common skills
    pub common: Vec<CommonSkill>,
    /// Master skills
    pub mastering: Vec<MasteringSkill>,
    /// Skill points
    pub points: f64,
}

/// Common skill
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CommonSkill {
    /// Skill ID
    pub id: String,
    /// Skill progress
    pub progress: f64,
    /// Points earned during session
    pub points_earned_during_session: f64,
    /// ?
    pub last_access: i64,
}

/// Master skill
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MasteringSkill {
    /// Skill ID
    pub id: String,
    /// Skill progress
    pub progress: u64,
}

/// Player statistics
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Stats {
    /// Session stats counters
    pub session_counters: SessionCounters,
    /// Overall stats counters
    pub overall_counters: OverallCounters,
    /// Session experience multiplier?
    pub session_experience_mult: f64,
    /// Experience bonus multiplier?
    pub experience_bonus_mult: u64,
    /// Total session experience
    pub total_session_experience: u64,
    /// Last session timestamp
    pub last_session_date: u64,
    /// ?
    pub aggressor: Option<Aggressor>,
    /// Total game time
    pub total_in_game_time: u64,
    /// Survivor class
    pub survivor_class: String,
    /// Unknown type
    pub dropped_items: serde_json::Value,
    /// Unknown type
    pub found_in_raid_items: serde_json::Value,
    /// Unknown type
    pub victims: Vec<Victim>,
    /// Unknown type
    pub carried_quest_items: serde_json::Value,
}

/// Session stats counter
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SessionCounters {
    /// Statistics
    pub items: Vec<SessionItem>,
}

/// Overall stats counter
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct OverallCounters {
    /// Statistics
    pub items: Vec<SessionItem>,
}

/// Statistics
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SessionItem {
    /// Statistic key
    pub key: Vec<String>,
    /// Statistic value
    pub value: u64,
}

/// Aggressor
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Aggressor {
    /// Aggressor name
    pub name: String,
    /// Aggressor side
    pub side: Side,
    /// Aggressor body part
    pub body_part: String,
    /// Aggressor head segment
    pub head_segment: Option<String>,
    /// Aggressor weapon name
    pub weapon_name: String,
    /// Aggressor category
    pub category: String,
}

/// Victim
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Victim {
    /// Victim name
    pub name: String,
    /// Victim side
    pub side: Side,
    /// Victim death time
    pub time: String,
    /// Victim level
    pub level: u64,
    /// Body part shot
    pub body_part: String,
    /// Weapon used to kill
    pub weapon: String,
}

/// ?
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ConditionCounters {
    /// Counters
    pub counters: Vec<ConditionCounter>,
}

/// ?
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConditionCounter {
    /// Counter ID
    pub id: String,
    /// Counter value
    pub value: u64,
}

/// ?
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BackendCounter {
    /// Counter ID
    pub id: String,
    /// ?
    pub qid: String,
    /// Counter value
    pub value: u64,
}

/// Insured item
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InsuredItem {
    /// Insurance ID
    #[serde(rename = "tid")]
    id: String,
    /// Insured item ID
    item_id: String,
}

/// Bonus?
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Bonus {
    /// ?
    #[serde(rename = "type")]
    pub bonus_type: String,
    /// ?
    pub template_id: Option<String>,
    /// ?
    pub value: Option<i64>,
    /// ?
    pub passive: Option<bool>,
    /// ?
    pub visible: Option<bool>,
    /// ?
    pub production: Option<bool>,
    /// ?
    pub filter: Option<Vec<String>>,
    /// ?
    pub id: Option<String>,
    /// ?
    pub icon: Option<String>,
}

/// Quest
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Quest {
    /// Quest ID
    #[serde(rename = "qid")]
    pub id: String,
    /// Quest start time
    pub start_time: u64,
    /// Quest status
    pub status: u64,
    /// Quest status timers
    pub status_timers: HashMap<String, u64>,
}

/// Flea market profile
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Ragfair {
    /// Market rating
    pub rating: f64,
    /// Market rating is growing
    pub is_rating_growing: bool,
    /// Active offers (items for sale)
    pub offers: Vec<Offer>,
}

/// Profile error.
#[derive(Debug, err_derive::Error)]
pub enum ProfileError {
    /// Invalid user ID selected.
    #[error(display = "invalid user id selected")]
    InvalidUserID,
    /// Failed selecting profile.
    #[error(display = "select profile failed")]
    SelectProfileFail,
}

#[derive(Debug, Serialize)]
struct SelectRequest<'a> {
    uid: &'a str,
}

#[derive(Debug, Deserialize)]
struct SelectResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Option<ProfileSelected>,
}

/// Profile select result
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
struct ProfileSelected {
    /// Profile status
    pub status: String,
    /// Profile notifier
    pub notifier: Notifier,
    /// Profile notifier server
    pub notifier_server: String,
}

/// Profile notifier
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Notifier {
    /// Notifier server
    pub server: String,
    /// Notifier channel ID
    pub channel_id: String,
    /// Notifier URL
    pub url: String,
}

impl Tarkov {
    /// Get a list of account's profiles.
    pub async fn get_profiles(&self) -> Result<Vec<Profile>> {
        let url = format!("{}/client/game/profile/list", PROD_ENDPOINT);
        let res: ProfileResponse = self.post_json(&url, &{}).await?;

        handle_error(res.error, res.data)
    }

    /// Select a profile by user ID.
    pub async fn select_profile(&self, user_id: &str) -> Result<()> {
        let url = format!("{}/client/game/profile/select", PROD_ENDPOINT);
        let res: SelectResponse = self
            .post_json(&url, &SelectRequest { uid: user_id })
            .await?;

        let res = handle_error(res.error, res.data)?;
        if res.status != "ok" {
            return Err(ProfileError::SelectProfileFail)?;
        }

        Ok(())
    }
}
