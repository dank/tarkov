use crate::{Error, ErrorResponse, Result, Tarkov, GAME_VERSION, PROD_ENDPOINT, UNITY_VERSION};
use actix_web::http::StatusCode;
use core::fmt;
use flate2::read::ZlibDecoder;
use serde::de::Unexpected;
use serde::{de, Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct ProfileResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Option<Vec<Profile>>,
}

#[derive(Debug, Deserialize)]
pub struct Profile {
    #[serde(rename = "_id")]
    pub id: String,
    pub aid: u64,
    pub savage: Option<String>,
    #[serde(rename = "Info")]
    pub info: Info,
    #[serde(rename = "Customization")]
    pub customization: Customization,
    #[serde(rename = "Health")]
    pub health: Health,
    #[serde(rename = "Inventory")]
    pub inventory: Inventory,
    #[serde(rename = "Skills")]
    pub skills: Skills,
    #[serde(rename = "Stats")]
    pub stats: Stats,
    #[serde(rename = "Encyclopedia")]
    pub encyclopedia: HashMap<String, bool>,
    #[serde(rename = "ConditionCounters")]
    pub condition_counters: ConditionCounters,
    #[serde(rename = "BackendCounters")]
    pub backend_counters: HashMap<String, BackendCounter>,
    // insured_items: [],
    // hideout: {},
    // notes: {},
    #[serde(rename = "Bonuses")]
    pub bonuses: Vec<Bonus>,
    #[serde(rename = "Quests")]
    pub quests: Vec<Quest>,
    #[serde(rename = "RagfairInfo")]
    pub ragfair: Ragfair,
    // trader_standings: [],
    // wish_list: [],
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Info {
    pub nickname: String,
    pub lower_nickname: String,
    // XXX: This could be enum
    pub side: String,
    pub voice: String,
    pub level: u64,
    pub experience: u64,
    pub registration_date: u64,
    pub game_version: String,
    pub account_type: u64,
    // XXX: Bad devs! This field can be both String and integer, ignoring for now.
    // member_category: String,
    #[serde(rename = "lockedMoveCommands")]
    pub locked_move_commands: bool,
    pub savage_lock_time: u64,
    pub last_time_played_as_savage: u64,
    pub settings: InfoSettings,
    pub need_wipe: bool,
    pub global_wipe: bool,
    pub nickname_change_date: u64,
    // bans: []
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InfoSettings {
    pub role: Option<String>,
    pub bot_difficulty: Option<String>,
    pub experience: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Customization {
    pub head: String,
    pub body: String,
    pub feet: String,
    pub hands: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HealthLevel {
    pub current: u64,
    pub maximum: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BodyParts {
    pub head: Head,
    pub chest: Chest,
    pub stomach: Stomach,
    pub left_arm: LeftArm,
    pub right_arm: RightArm,
    pub left_leg: LeftLeg,
    pub right_leg: RightLeg,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Head {
    pub health: HealthLevel,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Chest {
    pub health: HealthLevel,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Stomach {
    pub health: HealthLevel,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LeftArm {
    pub health: HealthLevel,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RightArm {
    pub health: HealthLevel,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LeftLeg {
    pub health: HealthLevel,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RightLeg {
    pub health: HealthLevel,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Health {
    pub hydration: HealthLevel,
    pub energy: HealthLevel,
    pub body_parts: BodyParts,
    pub update_time: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Upd {
    pub stack_objects_count: Option<u64>,
    pub spawned_in_session: Option<bool>,
    pub med_kit: Option<UpdMedkit>,
    pub repairable: Option<UpdRepairable>,
    pub light: Option<UpdLight>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdMedkit {
    pub hp_resource: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdRepairable {
    pub max_durability: f64,
    pub durability: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdLight {
    pub is_active: bool,
    pub selected_mode: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub x: u64,
    pub y: u64,
    pub r: u64,
    pub is_searched: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_tpl")]
    pub tpl: String,
    pub parent_id: Option<String>,
    pub slot_id: Option<String>,
    pub upd: Option<Upd>,
    // XXX: This type can be both Integer and `Location`...
    // location: Option<Location>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inventory {
    pub items: Vec<Item>,
    pub equipment: String,
    pub stash: Option<String>,
    pub quest_raid_items: String,
    pub quest_stash_items: String,
    // first_panel: {}
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Skills {
    pub common: Vec<CommonSkill>,
    pub mastering: Vec<MasteringSkill>,
    pub points: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommonSkill {
    pub id: String,
    pub progress: f64,
    pub points_earned_during_session: f64,
    pub last_access: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MasteringSkill {
    pub id: String,
    pub progress: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Stats {
    pub session_counters: SessionCounters,
    pub overall_counters: OverallCounters,
    pub session_experience_mult: u64,
    pub experience_bonus_mult: u64,
    pub total_session_experience: u64,
    pub last_session_date: u64,
    pub aggressor: StatsAggressor,
    pub total_in_game_time: u64,
    pub survivor_class: String,
    // dropped_items: [],
    // found_in_raid_items: [],
    // victims: [],
    // carried_quest_items: []
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SessionCounters {
    pub items: Vec<SessionItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OverallCounters {
    pub items: Vec<SessionItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SessionItem {
    pub key: Vec<String>,
    pub value: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StatsAggressor {
    pub name: String,
    pub side: String,
    pub body_part: String,
    pub head_segment: Option<String>,
    pub weapon_name: String,
    pub category: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConditionCounters {
    pub counters: Vec<ConditionCounter>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionCounter {
    pub id: String,
    pub value: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackendCounter {
    pub id: String,
    pub qid: String,
    pub value: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bonus {
    #[serde(rename = "type")]
    pub bonus_type: String,
    pub template_id: Option<String>,
    pub value: Option<i64>,
    pub passive: Option<bool>,
    pub visible: Option<bool>,
    pub production: Option<bool>,
    pub filter: Option<Vec<String>>,
    pub id: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quest {
    pub qid: String,
    pub start_time: u64,
    pub status: u64,
    pub status_timers: HashMap<String, u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ragfair {
    pub rating: f64,
    pub is_rating_growing: bool,
    // offers: []
}

#[derive(Debug, err_derive::Error)]
pub enum ProfileError {
    #[error(display = "invalid user id selected")]
    InvalidUserID,
}

#[derive(Debug, Serialize)]
struct SelectRequest<'a> {
    uid: &'a str,
}

#[derive(Deserialize)]
struct SelectResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    status: Option<String>,
}

impl Tarkov {
    /// Get a list of your profiles.
    pub async fn get_profiles(&self) -> Result<Vec<Profile>> {
        let url = format!("{}/client/game/profile/list", PROD_ENDPOINT);
        let res: ProfileResponse = self.post_json(&url, &{}).await?;

        self.handle_error(res.error, res.data.expect("API returned no errors but `data` is unavailable."))
    }

    /// Select a profile by user ID.
    pub async fn select_profile(&self, user_id: &str) -> Result<()> {
        let url = format!("{}/client/game/profile/select", PROD_ENDPOINT);
        let res: SelectResponse = self
            .post_json(&url, &SelectRequest { uid: user_id })
            .await?;

        self.handle_error(res.error, ())
    }
}

// XXX: I shouldn't have to do this if tarkov devs know what types are.
//struct LocationVisitor;
//
//impl<'de> de::Visitor<'de> for LocationVisitor {
//    type Value = Option<Location>;
//
//    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//        formatter.write_str("Location struct")
//    }
//
//    fn visit_none<E>(self) -> std::result::Result<Self::Value, E>
//        where
//            E: de::Error,
//    {
//        Ok(None)
//    }
//
//    fn visit_some<D>(self, d: D) -> std::result::Result<Self::Value, D::Error>
//        where
//            D: de::Deserializer<'de>,
//    {
//        Ok(None)
//    }
//}
//
//pub fn deserialize_location_or_none<'de, D>(d: D) -> std::result::Result<Option<Location>, D::Error>
//    where
//        D: de::Deserializer<'de>,
//{
//    d.deserialize_option(LocationVisitor)
//}
