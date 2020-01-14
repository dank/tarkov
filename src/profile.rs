use crate::{Tarkov, Result, GAME_VERSION, UNITY_VERSION, PROD_ENDPOINT, Error, ErrorResponse};
use serde::{Deserialize, Serialize, de};
use flate2::read::ZlibDecoder;
use std::io::Read;
use actix_web::http::StatusCode;
use std::fmt::Write;
use core::fmt;
use serde::de::Unexpected;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct ProfileResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Option<Vec<ProfileData>>,
}

#[derive(Debug, Deserialize)]
pub struct ProfileData {
    #[serde(rename = "_id")]
    id: String,
    aid: u64,
    savage: Option<String>,
    #[serde(rename = "Info")]
    info: Info,
    #[serde(rename = "Customization")]
    customization: Customization,
    #[serde(rename = "Health")]
    health: Health,
    #[serde(rename = "Inventory")]
    inventory: Inventory,
    #[serde(rename = "Skills")]
    skills: Skills,
    #[serde(rename = "Stats")]
    stats: Stats,
    #[serde(rename = "Encyclopedia")]
    encyclopedia: HashMap<String, bool>,
    #[serde(rename = "ConditionCounters")]
    condition_counters: ConditionCounters,
    #[serde(rename = "BackendCounters")]
    backend_counters: HashMap<String, BackendCounter>,
    // insured_items: [],
    // hideout: {},
    // notes: {},
    #[serde(rename = "Bonuses")]
    bonuses: Vec<Bonus>,
    #[serde(rename = "Quests")]
    quests: Vec<Quest>,
    #[serde(rename = "RagfairInfo")]
    ragfair: Ragfair
    // trader_standings
    // wish_list
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Info {
    nickname: String,
    // XXX: BAD DEVS!
    lower_nickname: Option<String>,
    lowercase_nickname: Option<String>,
    // TODO: This can be enum
    side: String,
    voice: String,
    level: u64,
    experience: u64,
    registration_date: u64,
    game_version: String,
    account_type: u64,
    // XXX: Bad devs! This field can be both String and integer, ignoring for now.
    // member_category: String,
    #[serde(rename = "lockedMoveCommands")]
    locked_move_commands: bool,
    savage_lock_time: u64,
    last_time_played_as_savage: u64,
    settings: InfoSettings,
    need_wipe: bool,
    global_wipe: bool,
    nickname_change_date: u64,
    // bans: [] TODO: Type unknown
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InfoSettings {
    role: Option<String>,
    bot_difficulty: Option<String>,
    experience: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Customization {
    head: String,
    body: String,
    feet: String,
    hands: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HealthData {
    current: u64,
    maximum: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BodyParts {
    head: Head,
    chest: Chest,
    stomach: Stomach,
    left_arm: LeftArm,
    right_arm: RightArm,
    left_leg: LeftLeg,
    right_leg: RightLeg,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Head {
    health: HealthData
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Chest {
    health: HealthData
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Stomach {
    health: HealthData
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LeftArm {
    health: HealthData
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RightArm {
    health: HealthData
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LeftLeg {
    health: HealthData
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RightLeg {
    health: HealthData
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Health {
    hydration: HealthData,
    energy: HealthData,
    body_parts: BodyParts,
    update_time: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Upd {
    stack_objects_count: Option<u64>,
    spawned_in_session: Option<bool>,
    med_kit: Option<UpdMedkit>,
    repairable: Option<UpdRepairable>,
    light: Option<UpdLight>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdMedkit {
    hp_resource: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdRepairable {
    max_durability: u64,
    durability: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdLight {
    is_active: bool,
    selected_mode: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    x: u64,
    y: u64,
    r: u64,
    is_searched: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "_tpl")]
    tpl: String,
    parent_id: Option<String>,
    slot_id: Option<String>,
//    location: Option<Location>, TODO: Bad type...
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inventory {
    items: Vec<Item>,
    equipment: String,
    stash: Option<String>,
    quest_raid_items: String,
    quest_stash_items: String,
    // first_panel: {} // TODO: Type unknown
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Skills {
    common: Vec<CommonSkill>,
    mastering: Vec<MasteringSkill>,
    points: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommonSkill {
    id: String,
    progress: f64,
    points_earned_during_session: f64,
    last_access: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MasteringSkill {
    id: String,
    progress: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Stats {
    session_counters: SessionCounters,
    overall_counters: OverallCounters,
    session_experience_mult: u64,
    experience_bonus_mult: u64,
    total_session_experience: u64,
    last_session_date: u64,
    aggressor: StatsAggressor,
    total_in_game_time: u64,
    survivor_class: String
    // TODO: Unknown types:
    // dropped_items: [],
    // found_in_raid_items: [],
    // victims: [],
    // carried_quest_items: [],
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SessionCounters {
    items: Vec<SessionItem>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OverallCounters {
    items: Vec<SessionItem>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SessionItem {
    key: Vec<String>,
    value: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StatsAggressor {
    name: String,
    side: String,
    body_part: String,
    head_segment: Option<String>,
    weapon_name: String,
    category: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConditionCounters {
    counters: Vec<ConditionCounter>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionCounter {
    id: String,
    value: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackendCounter {
    id: String,
    qid: String,
    value: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bonus {
    #[serde(rename = "type")]
    bonus_type: String,
    template_id: Option<String>,
    value: Option<i64>,
    passive: Option<bool>,
    visible: Option<bool>,
    production: Option<bool>,
    filter: Option<Vec<String>>,
    id: Option<String>,
    icon: Option<String>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quest {
    qid: String,
    start_time: u64,
    status: u64,
    status_timers: HashMap<String, u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ragfair {
    rating: f64,
    is_rating_growing: bool,
    // offers: []
}


#[derive(Debug, err_derive::Error)]
pub enum ProfileError {
    #[error(display = "not authorized or game profile not selected")]
    NotAuthorized,
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

#[derive(Debug, err_derive::Error)]
pub enum SelectError {
    #[error(display = "invalid user id selected")]
    InvalidUserID,
}

impl Tarkov {
    pub async fn get_profiles(&self) -> Result<Vec<ProfileData>> {
        let url = format!("{}/client/game/profile/list", PROD_ENDPOINT);
        let res: ProfileResponse = self.post_json(&url, &{}).await?;
        match res.error.code {
            0 => Ok(res
                .data
                .expect("API returned no errors but `data` is unavailable.")),
            201 => Err(ProfileError::NotAuthorized)?,
            _ => Err(Error::UnknownAPIError(res.error.code)),
        }
    }

    pub async fn select_profile(&self, user_id: &str) -> Result<()> {
        let url = format!("{}/client/game/profile/select", PROD_ENDPOINT);
        let res: SelectResponse = self.post_json(&url, &SelectRequest { uid: user_id }).await?;
        match res.error.code {
            0 => Ok(()),
            205 => Err(SelectError::InvalidUserID)?,
            _ => Err(Error::UnknownAPIError(res.error.code)),
        }
    }
}

// XXX: I shouldn't have to do this if tarkov devs know what types are.
struct LocationVisitor;

impl<'de> de::Visitor<'de> for LocationVisitor {
    type Value = Option<Location>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Location struct")
    }

    fn visit_none<E>(self) -> std::result::Result<Self::Value, E>
        where
            E: de::Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, d: D) -> std::result::Result<Self::Value, D::Error>
        where
            D: de::Deserializer<'de>,
    {
        Ok(None)
    }
}

pub fn deserialize_location_or_none<'de, D>(d: D) -> std::result::Result<Option<Location>, D::Error>
    where
        D: de::Deserializer<'de>,
{
    d.deserialize_option(LocationVisitor)
}