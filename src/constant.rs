use crate::{ErrorResponse, Result, Tarkov, PROD_ENDPOINT};

use crate::badjson::StringOrInt;
use crate::profile::Side;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Request {
    crc: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ItemsResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Option<HashMap<String, Item>>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Item {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_name")]
    pub name: String,
    #[serde(rename = "_parent")]
    pub parent: String,
    #[serde(rename = "_type")]
    pub item_type: String,
    #[serde(rename = "_props")]
    pub props: Props,
    #[serde(rename = "_proto")]
    pub proto: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Props {
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub description: Option<String>,
    pub weight: Option<f64>,
    pub background_color: Option<String>,
    pub width: Option<u64>,
    pub height: Option<u64>,
    pub stack_max_size: Option<u64>,
    pub rarity: Option<String>,
    pub spawn_chance: Option<f64>,
    pub credits_price: Option<u64>,
    pub item_sound: Option<String>,
    pub prefab: Option<Prefab>,
    pub use_prefab: Option<Prefab>,
    pub stack_objects_count: Option<u64>,
    pub not_shown_in_slot: Option<bool>,
    pub examined_by_default: Option<bool>,
    pub examine_time: Option<u64>,
    pub is_undiscardable: Option<bool>,
    pub is_unsaleable: Option<bool>,
    pub is_unbuyable: Option<bool>,
    pub is_ungivable: Option<bool>,
    #[serde(rename = "IsLockedafterEquip")]
    pub is_locked_after_equip: Option<bool>,
    pub quest_item: Option<bool>,
    pub loot_experience: Option<u64>,
    pub examine_experience: Option<u64>,
    pub hide_entrails: Option<bool>,
    pub repair_cost: Option<u64>,
    pub repair_speed: Option<u64>,
    pub extra_size_left: Option<u64>,
    pub extra_size_right: Option<u64>,
    pub extra_size_up: Option<u64>,
    pub extra_size_down: Option<u64>,
    pub extra_size_force_add: Option<bool>,
    pub merges_with_children: Option<bool>,
    pub can_sell_on_ragfair: Option<bool>,
    pub can_require_on_ragfair: Option<bool>,
    pub banned_from_ragfair: Option<bool>,
    pub conflicting_items: Option<Vec<String>>,
    pub fixed_price: Option<bool>,
    pub unlootable: Option<bool>,
    pub unlootable_from_slot: Option<String>,
    pub unlootable_from_side: Option<Vec<Side>>,
    #[serde(rename = "ChangePriceCoef")]
    pub change_price_coefficient: Option<u64>,
    pub allow_spawn_on_locations: Option<Vec<String>>,
    pub send_to_client: Option<bool>,
    pub animation_variants_number: Option<u64>,
    pub discarding_block: Option<bool>,
    pub max_resource: Option<u64>,
    pub resource: Option<u64>,
    pub dog_tag_qualities: Option<bool>,
    pub grids: Option<Vec<Grid>>,
    pub slots: Option<Vec<Slot>>,
    pub can_put_into_during_the_raid: Option<bool>,
    pub cant_remove_from_slots_during_raid: Option<Vec<String>>,
    pub key_ids: Option<Vec<String>>,
    pub tag_color: Option<u64>,
    pub tag_name: Option<String>,
    pub durability: Option<u64>,
    pub accuracy: Option<i64>,
    pub recoil: Option<f64>,
    pub loudness: Option<i64>,
    pub effective_distance: Option<u64>,
    pub ergonomics: Option<f64>,
    pub velocity: Option<f64>,
    pub raid_moddable: Option<bool>,
    pub tool_moddable: Option<bool>,
    pub blocks_folding: Option<bool>,
    pub blocks_collapsible: Option<bool>,
    pub is_animated: Option<bool>,
    pub has_shoulder_contact: Option<bool>,
    pub sighting_range: Option<u64>,
    pub modes_count: Option<u64>,
    #[serde(rename = "muzzleModType")]
    pub muzzle_mod_type: Option<String>,
    #[serde(rename = "sightModType")]
    pub sight_mod_type: Option<String>,
    #[serde(rename = "variableZoom")]
    pub variable_zoom: Option<bool>,
    #[serde(rename = "varZoomCount")]
    pub var_zoom_count: Option<u64>,
    #[serde(rename = "varZoomAdd")]
    pub var_zoom_add: Option<u64>,
    #[serde(rename = "aimingSensitivity")]
    pub aiming_sensitivity: Option<f64>,
    pub sight_modes_count: Option<u64>,
    pub optic_calibration_distances: Option<Vec<u64>>,
    pub intensity: Option<f64>,
    pub mask: Option<String>,
    pub mask_size: Option<f64>,
    pub noise_intensity: Option<f64>,
    pub noise_scale: Option<u64>,
    pub color: Option<Color>,
    pub diffuse_intensity: Option<f64>,
    pub has_hinge: Option<bool>,
    pub ramp_palette: Option<String>,
    pub depth_fade: Option<f64>,
    #[serde(rename = "RoughnessCoef")]
    pub roughness_coefficient: Option<f64>,
    #[serde(rename = "SpecularCoef")]
    pub specular_coefficient: Option<f64>,
    #[serde(rename = "MainTexColorCoef")]
    pub main_tex_color_coefficient: Option<f64>,
    pub minimum_temperature_value: Option<f64>,
    pub ramp_shift: Option<f64>,
    pub heat_min: Option<f64>,
    pub cold_max: Option<f64>,
    pub is_noisy: Option<bool>,
    pub is_fps_stuck: Option<bool>,
    pub is_glitch: Option<bool>,
    pub is_motion_blurred: Option<bool>,
    pub is_pixelated: Option<bool>,
    pub pixelation_block_count: Option<u64>,
    #[serde(rename = "magAnimationIndex")]
    pub mag_animation_index: Option<u64>,
    pub cartridges: Option<Vec<Cartridge>>,
    pub can_fast: Option<bool>,
    pub can_hit: Option<bool>,
    pub can_admin: Option<bool>,
    pub load_unload_modifier: Option<i64>,
    pub check_time_modifier: Option<i64>,
    pub check_override: Option<u64>,
    pub reload_mag_type: Option<String>,
    pub visible_ammo_ranges_string: Option<String>,
    pub is_shoulder_contact: Option<bool>,
    pub foldable: Option<bool>,
    pub retractable: Option<bool>,
    pub size_reduce_right: Option<u64>,
    pub center_of_impact: Option<f64>,
    pub shotgun_dispersion: Option<f64>,
    pub is_silencer: Option<bool>,
    pub search_sound: Option<String>,
    pub blocks_armor_vest: Option<bool>,
    #[serde(rename = "speedPenaltyPercent")]
    pub speed_penalty_percent: Option<i64>,
    pub grid_layout_name: Option<String>,
    pub spawn_filter: Option<Vec<String>>,
    /// Unknown type
    #[serde(rename = "containType")]
    pub contain_type: Option<serde_json::Value>,
    #[serde(rename = "sizeWidth")]
    pub size_width: Option<u64>,
    #[serde(rename = "sizeHeight")]
    pub size_height: Option<u64>,
    #[serde(rename = "isSecured")]
    pub is_secured: Option<bool>,
    #[serde(rename = "spawnTypes")]
    pub spawn_types: Option<String>,
    /// Unknown type
    #[serde(rename = "lootFilter")]
    pub loot_filter: Option<serde_json::Value>,
    #[serde(rename = "spawnRarity")]
    pub spawn_rarity: Option<String>,
    #[serde(rename = "minCountSpawn")]
    pub min_count_spawn: Option<u64>,
    #[serde(rename = "maxCountSpawn")]
    pub max_count_spawn: Option<u64>,
    /// Unknown type
    #[serde(rename = "openedByKeyID")]
    pub opened_by_key_id: Option<serde_json::Value>,
    pub rig_layout_name: Option<String>,
    pub max_durability: Option<u64>,
    #[serde(rename = "armorZone")]
    pub armor_zone: Option<Vec<ArmorZone>>,
    // #[serde(rename = "armorClass")]
    // pub armor_class: Option<u64>, // armorClass can be both String and int...
    #[serde(rename = "mousePenalty")]
    pub mouse_penalty: Option<i64>,
    #[serde(rename = "weaponErgonomicPenalty")]
    pub weapon_ergonomic_penalty: Option<i64>,
    pub blunt_throughput: Option<f64>,
    pub armor_material: Option<String>,
    #[serde(rename = "weapClass")]
    pub weapon_class: Option<String>,
    #[serde(rename = "weapUseType")]
    pub weapon_use_type: Option<String>,
    pub ammo_caliber: Option<String>,
    pub operating_resource: Option<u64>,
    pub repair_complexity: Option<u64>,
    #[serde(rename = "durabSpawnMin")]
    pub durability_spawn_min: Option<u64>,
    #[serde(rename = "durabSpawnMax")]
    pub durability_spawn_max: Option<u64>,
    #[serde(rename = "isFastReload")]
    pub is_fast_reload: Option<bool>,
    pub recoil_force_up: Option<u64>,
    pub recoil_force_back: Option<u64>,
    pub convergence: Option<f64>,
    pub recoil_angle: Option<u64>,
    #[serde(rename = "weapFireType")]
    pub weapon_fire_type: Option<Vec<FireMode>>,
    #[serde(rename = "RecolDispersion")]
    pub recoil_dispersion: Option<u64>,
    #[serde(rename = "bFirerate")]
    pub firerate: Option<u64>,
    #[serde(rename = "bEffDist")]
    pub eff_dist: Option<u64>,
    #[serde(rename = "bHearDist")]
    pub hear_dist: Option<u64>,
    #[serde(rename = "isChamberLoad")]
    pub is_chamber_load: Option<bool>,
    #[serde(rename = "chamberAmmoCount")]
    pub chamber_ammo_count: Option<u64>,
    #[serde(rename = "isBoltCatch")]
    pub is_bolt_catch: Option<bool>,
    #[serde(rename = "defMagType")]
    pub def_mag_type: Option<String>,
    #[serde(rename = "defAmmo")]
    pub def_ammo: Option<String>,
    pub chambers: Option<Vec<Chamber>>,
    pub camera_recoil: Option<f64>,
    pub camera_snap: Option<f64>,
    pub reload_mode: Option<String>,
    pub aim_plane: Option<f64>,
    pub deviation_curve: Option<u64>,
    pub deviation_max: Option<u64>,
    #[serde(rename = "TacticalReloadStiffnes")]
    pub tactical_reload_stiffness: Option<Coordinate>,
    pub tactical_reload_fixation: Option<f64>,
    pub recoil_center: Option<Coordinate>,
    pub rotation_center: Option<Coordinate>,
    pub rotation_center_no_stock: Option<Coordinate>,
    pub folded_slot: Option<String>,
    pub compact_handling: Option<bool>,
    pub min_repair_degradation: Option<u64>,
    pub max_repair_degradation: Option<f64>,
    pub iron_sight_range: Option<u64>,
    #[serde(rename = "MustBoltBeOpennedForExternalReload")]
    pub must_bolt_be_opened_for_external_reload: Option<bool>,
    #[serde(rename = "MustBoltBeOpennedForInternalReload")]
    pub must_bolt_be_opened_for_internal_reload: Option<bool>,
    pub bolt_action: Option<bool>,
    pub hip_accuracy_restoration_delay: Option<f64>,
    pub hip_accuracy_restoration_speed: Option<u64>,
    #[serde(rename = "HipInnaccuracyGain")]
    pub hip_inaccuracy_gain: Option<f64>,
    pub manual_bolt_catch: Option<bool>,
    pub blocks_earpiece: Option<bool>,
    #[serde(rename = "BlocksEyewear")]
    pub blocks_eye_wear: Option<bool>,
    #[serde(rename = "BlocksHeadwear")]
    pub blocks_head_wear: Option<bool>,
    pub blocks_face_cover: Option<bool>,
    #[serde(rename = "foodUseTime")]
    pub food_use_time: Option<u64>,
    #[serde(rename = "foodEffectType")]
    pub food_effect_type: Option<String>,
    pub stimulator_buffs: Option<String>,
    #[serde(rename = "effects_health")]
    pub effects_health: Option<HealthEffects>,
    #[serde(rename = "effects_damage")]
    pub effects_damage: Option<DamageEffects>,
    #[serde(rename = "effects_speed")]
    pub effects_speed: Option<SpeedEffects>,
    pub maximum_number_of_usage: Option<u64>,
    #[serde(rename = "knifeHitDelay")]
    pub knife_hit_delay: Option<u64>,
    #[serde(rename = "knifeHitSlashRate")]
    pub knife_hit_slash_rate: Option<u64>,
    #[serde(rename = "knifeHitStabRate")]
    pub knife_hit_stab_rate: Option<u64>,
    #[serde(rename = "knifeHitRadius")]
    pub knife_hit_radius: Option<f64>,
    #[serde(rename = "knifeHitSlashDam")]
    pub knife_hit_slash_dam: Option<u64>,
    #[serde(rename = "knifeHitStabDam")]
    pub knife_hit_stab_dam: Option<u64>,
    #[serde(rename = "knifeDurab")]
    pub knife_durability: Option<u64>,
    pub primary_distance: Option<f64>,
    #[serde(rename = "SecondryDistance")]
    pub secondary_distance: Option<f64>,
    pub slash_penetration: Option<u64>,
    pub stab_penetration: Option<u64>,
    pub primary_consumption: Option<u64>,
    #[serde(rename = "SecondryConsumption")]
    pub secondary_consumption: Option<u64>,
    pub deflection_consumption: Option<u64>,
    pub config_path_str: Option<String>,
    pub max_markers_count: Option<u64>,
    #[serde(rename = "scaleMin")]
    pub scale_min: Option<f64>,
    #[serde(rename = "scaleMax")]
    pub scale_max: Option<f64>,
    #[serde(rename = "medUseTime")]
    pub med_use_time: Option<u64>,
    #[serde(rename = "medEffectType")]
    pub med_effect_type: Option<String>,
    pub max_hp_resource: Option<u64>,
    #[serde(rename = "hpResourceRate")]
    pub hp_resource_rate: Option<u64>,
    pub max_efficiency: Option<u64>,
    pub addiction: Option<u64>,
    pub overdose: Option<u64>,
    pub overdose_recovery: Option<u64>,
    pub addiction_recovery: Option<u64>,
    // pub buffs: Option<serde_json::Value>, bad devs, multiple types...
    #[serde(rename = "apResource")]
    pub ap_resource: Option<u64>,
    #[serde(rename = "krResource")]
    pub kr_resource: Option<u64>,
    pub stack_min_random: Option<u64>,
    pub stack_max_random: Option<u64>,
    #[serde(rename = "ammoType")]
    pub ammo_type: Option<String>,
    pub damage: Option<u64>,
    #[serde(rename = "ammoAccr")]
    pub ammo_accr: Option<i64>,
    #[serde(rename = "ammoRec")]
    pub ammo_rec: Option<i64>,
    #[serde(rename = "ammoDist")]
    pub ammo_dist: Option<u64>,
    #[serde(rename = "buckshotBullets")]
    pub buckshot_bullets: Option<u64>,
    pub penetration_power: Option<u64>,
    pub penetration_power_diviation: Option<f64>,
    #[serde(rename = "ammoHear")]
    pub ammo_hear: Option<i64>,
    #[serde(rename = "ammoSfx")]
    pub ammo_sfx: Option<String>,
    pub misfire_chance: Option<f64>,
    pub min_fragments_count: Option<u64>,
    pub max_fragments_count: Option<u64>,
    #[serde(rename = "ammoShiftChance")]
    pub ammo_shift_chance: Option<u64>,
    #[serde(rename = "casingName")]
    pub casing_name: Option<String>,
    #[serde(rename = "casingEjectPower")]
    pub casing_eject_power: Option<u64>,
    #[serde(rename = "casingMass")]
    pub casing_mass: Option<f64>,
    #[serde(rename = "casingSounds")]
    pub casing_sounds: Option<String>,
    pub projectile_count: Option<u64>,
    pub initial_speed: Option<u64>,
    pub penetration_chance: Option<f64>,
    pub ricochet_chance: Option<f64>,
    pub fragmentation_chance: Option<f64>,
    #[serde(rename = "BallisticCoeficient")]
    pub ballistic_coefficient: Option<f64>,
    pub deterioration: Option<u64>,
    pub speed_retardation: Option<f64>,
    pub tracer: Option<bool>,
    pub tracer_color: Option<String>,
    pub tracer_distance: Option<f64>,
    pub armor_damage: Option<u64>,
    pub caliber: Option<String>,
    pub stamina_burn_per_damage: Option<f64>,
    pub show_bullet: Option<bool>,
    pub has_grenader_component: Option<bool>,
    pub fuze_arm_time_sec: Option<f64>,
    pub explosion_strength: Option<u64>,
    pub min_explosion_distance: Option<f64>,
    pub max_explosion_distance: Option<f64>,
    pub fragments_count: Option<u64>,
    pub fragment_type: Option<String>,
    pub show_hit_effect_on_explode: Option<bool>,
    pub explosion_type: Option<String>,
    pub ammo_life_time_sec: Option<u64>,
    pub stack_slots: Option<Vec<StackSlot>>,
    #[serde(rename = "type")]
    pub item_item: Option<String>,
    #[serde(rename = "eqMin")]
    pub eq_min: Option<u64>,
    #[serde(rename = "eqMax")]
    pub eq_max: Option<u64>,
    #[serde(rename = "rate")]
    pub rate: Option<u64>,
    pub throw_type: Option<String>,
    pub strength: Option<u64>,
    pub contusion_distance: Option<u64>,
    #[serde(rename = "throwDamMax")]
    pub throw_dam_max: Option<u64>,
    pub expl_delay: Option<f64>,
    pub blindness: Option<Coordinate>,
    pub contusion: Option<Coordinate>,
    pub emit_time: Option<u64>,
    pub can_be_hidden_during_throw: Option<bool>,
    pub indestructibility: Option<f64>,
    #[serde(rename = "headSegments")]
    pub head_segments: Option<Vec<HeadSegment>>,
    pub face_shield_component: Option<bool>,
    pub face_shield_mask: Option<String>,
    pub material_type: Option<String>,
    pub ricochet_params: Option<Coordinate>,
    pub deaf_strength: Option<String>,
    pub distortion: Option<f64>,
    #[serde(rename = "CompressorTreshold")]
    pub compressor_threshold: Option<i64>,
    pub compressor_attack: Option<u64>,
    pub compressor_release: Option<u64>,
    pub compressor_gain: Option<u64>,
    pub cutoff_freq: Option<u64>,
    pub resonance: Option<f64>,
    pub compressor_volume: Option<i64>,
    pub ambient_volume: Option<i64>,
    pub dry_volume: Option<i64>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Prefab {
    pub path: String,
    pub rcid: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Grid {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_name")]
    pub name: String,
    #[serde(rename = "_parent")]
    pub parent: String,
    #[serde(rename = "_props")]
    pub props: GridProps,
    #[serde(rename = "_proto")]
    pub proto: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GridProps {
    pub filters: Vec<GridFilter>,
    pub cells_h: u64,
    pub cells_v: u64,
    pub min_count: u64,
    pub max_count: u64,
    pub max_weight: u64,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct GridFilter {
    pub filter: Vec<String>,
    pub excluded_filter: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Slot {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_name")]
    pub name: String,
    #[serde(rename = "_parent")]
    pub parent: String,
    #[serde(rename = "_props")]
    pub props: SlotProps,
    #[serde(rename = "_required")]
    pub required: bool,
    #[serde(rename = "_mergeSlotWithChildren")]
    pub merge_slot_with_children: bool,
    #[serde(rename = "_proto")]
    pub proto: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SlotProps {
    pub slot: Option<i64>,
    pub animation_index: Option<i64>,
    pub filters: Option<Vec<SlotFilter>>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SlotFilter {
    pub slot: Option<i64>,
    pub animation_index: Option<i64>,
    pub filters: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Cartridge {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_name")]
    pub name: String,
    #[serde(rename = "_parent")]
    pub parent: String,
    #[serde(rename = "_max_count")]
    pub max_count: u64,
    #[serde(rename = "_props")]
    pub props: CartridgeProps,
    #[serde(rename = "_proto")]
    pub proto: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct CartridgeProps {
    pub filters: Vec<CartridgeFilter>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CartridgeFilter {
    pub filter: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum ArmorZone {
    Head,
    Chest,
    Stomach,
    LeftArm,
    RightArm,
    LeftLeg,
    RightLeg,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FireMode {
    Single,
    Burst,
    FullAuto,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Chamber {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_name")]
    pub name: String,
    #[serde(rename = "_parent")]
    pub parent: String,
    #[serde(rename = "_props")]
    pub props: ChamberProps,
    #[serde(rename = "_required")]
    pub required: bool,
    #[serde(rename = "_mergeSlotWithChildren")]
    pub merge_slot_with_children: bool,
    #[serde(rename = "_proto")]
    pub proto: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChamberProps {
    pub filters: Vec<ChamberFilter>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ChamberFilter {
    pub filter: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum HeadSegment {
    Top,
    Nape,
    Ears,
    Eyes,
    Jaws,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct HealthEffects {
    pub common: Health,
    pub head: Health,
    pub arm_left: Health,
    pub arm_right: Health,
    pub chest: Health,
    pub tummy: Health,
    pub leg_left: Health,
    pub leg_right: Health,
    pub energy: Health,
    pub hydration: Health,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Health {
    pub value: i64,
    pub percent: bool,
    pub time: u64,
    pub duration: u64,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DamageEffects {
    pub bloodloss: Damage,
    pub fracture: Damage,
    pub pain: Damage,
    pub contusion: Damage,
    pub toxication: Damage,
    #[serde(rename = "radExposure")]
    pub radiation_exposure: Damage,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Damage {
    pub remove: bool,
    pub time: u64,
    pub duration: u64,
    pub fade_out: Option<u64>,
    pub cost: Option<u64>,
    pub health_penalty_min: Option<u64>,
    pub health_penalty_max: Option<u64>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SpeedEffects {
    pub mobility: Speed,
    pub recoil: Speed,
    pub reload_speed: Speed,
    pub loot_speed: Speed,
    pub unlock_speed: Speed,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Speed {
    pub value: i64,
    pub percent: bool,
    pub time: u64,
    pub duration: u64,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct StackSlot {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_name")]
    pub name: String,
    #[serde(rename = "_parent")]
    pub parent: String,
    #[serde(rename = "_max_count")]
    pub max_count: u64,
    #[serde(rename = "_props")]
    pub props: StackSlotProps,
    #[serde(rename = "_proto")]
    pub proto: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct StackSlotProps {
    pub filters: Vec<StackSlotFilter>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StackSlotFilter {
    pub filter: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct LocationsResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Option<Locations>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Locations {
    pub locations: HashMap<String, Location>,
    pub paths: Vec<Path>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Location {
    pub enabled: bool,
    pub locked: bool,
    pub insurance: bool,
    pub safe_location: bool,
    pub name: String,
    pub description: String,
    pub scene: Scene,
    pub area: f64,
    pub required_player_level: u64,
    #[serde(rename = "surv_gather_minutes")]
    pub surv_gather_minutes: u64,
    pub min_players: u64,
    pub max_players: u64,
    #[serde(rename = "sav_gather_minutes")]
    pub scav_gather_minutes: u64,
    #[serde(rename = "exit_count")]
    pub exit_count: u64,
    #[serde(rename = "exit_access_time")]
    pub exit_access_time: u64,
    #[serde(rename = "exit_time")]
    pub exit_time: u64,
    pub preview: Preview,
    pub icon_x: u64,
    pub icon_y: u64,
    /// Unknown type
    #[serde(rename = "filter_ex")]
    pub filter_ex: Vec<serde_json::Value>,
    #[serde(rename = "waves")]
    pub waves: Vec<Wave>,
    /// Unknown type
    #[serde(rename = "limits")]
    pub limits: Vec<serde_json::Value>,
    pub average_play_time: u64,
    pub average_player_level: u64,
    #[serde(rename = "escape_time_limit")]
    pub escape_time_limit: u64,
    pub rules: String,
    pub is_secret: bool,
    /// Unknown type
    #[serde(rename = "doors")]
    pub doors: Vec<serde_json::Value>,
    #[serde(rename = "tmp_location_field_remove_me")]
    pub tmp_location_field_remove_me: u64,
    #[serde(rename = "MinDistToExitPoint")]
    pub min_distance_to_exit_point: u64,
    #[serde(rename = "MinDistToFreePoint")]
    pub min_distance_to_free_point: u64,
    #[serde(rename = "MaxDistToFreePoint")]
    pub max_distance_to_free_point: u64,
    pub max_bot_per_zone: u64,
    pub open_zones: String,
    #[serde(rename = "OcculsionCullingEnabled")]
    pub occlusion_culling_enabled: bool,
    pub global_loot_chance_modifier: f64,
    pub old_spawn: bool,
    pub new_spawn: bool,
    pub bot_max: u64,
    pub bot_start: u64,
    pub bot_stop: u64,
    pub bot_max_time_player: u64,
    pub bot_spawn_time_on_min: u64,
    pub bot_spawn_time_on_max: u64,
    pub bot_spawn_time_off_min: u64,
    pub bot_spawn_time_off_max: u64,
    pub bot_max_player: u64,
    pub bot_easy: u64,
    pub bot_normal: u64,
    pub bot_hard: u64,
    pub bot_impossible: u64,
    pub bot_assault: u64,
    pub bot_marksman: u64,
    pub disabled_scav_exits: String,
    pub access_keys: Vec<String>,
    pub min_max_bots: Vec<MinMaxBot>,
    pub bot_location_modifier: BotLocationModifier,
    #[serde(rename = "exits")]
    pub exits: Vec<Exit>,
    pub disabled_for_scav: bool,
    pub boss_location_spawn: Vec<BossSpawn>,
    #[serde(rename = "Id")]
    pub name_id: String,
    #[serde(rename = "_Id")]
    pub id: String,
    /// Unknown type
    pub loot: Vec<serde_json::Value>,
    /// Unknown type
    pub spawn_areas: Vec<serde_json::Value>,
    pub banners: Vec<Banner>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Scene {
    pub path: String,
    pub rcid: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Path {
    pub source: String,
    pub destination: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Preview {
    pub path: String,
    pub rcid: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Wave {
    #[serde(rename = "number")]
    pub number: u64,
    #[serde(rename = "time_min")]
    pub time_min: u64,
    #[serde(rename = "time_max")]
    pub time_max: u64,
    #[serde(rename = "slots_min")]
    pub slots_min: u64,
    #[serde(rename = "slots_max")]
    pub slots_max: u64,
    pub spawn_points: String,
    pub bot_side: Side,
    pub bot_preset: BotDifficulty,
    #[serde(rename = "isPlayers")]
    pub is_players: bool,
    pub wild_spawn_type: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct MinMaxBot {
    pub min: u64,
    pub max: u64,
    #[serde(rename = "WildSpawnType")]
    pub wild_spawn_type: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BotLocationModifier {
    pub accuracy_speed: u64,
    pub scattering: u64,
    pub gain_sight: u64,
    #[serde(rename = "MarksmanAccuratyCoef")]
    pub marksman_accuracy_coefficient: u64,
    pub visible_distance: u64,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Exit {
    pub name: String,
    pub entry_points: String,
    pub chance: u64,
    pub min_time: u64,
    pub max_time: u64,
    pub players_count: u64,
    pub exfiltration_time: u64,
    pub passage_requirement: Option<String>,
    pub exfiltration_type: Option<String>,
    pub required_slot: Option<String>,
    pub count: Option<u64>,
    pub id: String,
    pub requirement_tip: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BossSpawn {
    #[serde(rename = "BossName")]
    pub name: String,
    #[serde(rename = "BossChance")]
    pub chance: u64,
    #[serde(rename = "BossZone")]
    pub zone: String,
    #[serde(rename = "BossPlayer")]
    pub player: bool,
    #[serde(rename = "BossDifficult")]
    pub difficulty: BotDifficulty,
    #[serde(rename = "BossEscortType")]
    pub escort_type: String,
    #[serde(rename = "BossEscortDifficult")]
    pub escort_difficulty: BotDifficulty,
    #[serde(rename = "BossEscortAmount")]
    pub escort_amount: String,
    pub time: i64,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BotDifficulty {
    Easy,
    Normal,
    Hard,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Banner {
    pub id: String,
    pub pic: BannerPic,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct BannerPic {
    pub path: String,
    pub rcid: String,
}

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Option<WeatherData>,
}

#[derive(Debug, Deserialize)]
struct WeatherData {
    weather: Weather,
    date: String,
    time: String,
    acceleration: u64,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Weather {
    pub timestamp: u64,
    pub cloud: f64,
    pub wind_speed: u64,
    pub wind_direction: u64,
    pub wind_gustiness: f64,
    pub rain: u64,
    pub rain_intensity: f64,
    pub fog: f64,
    pub temp: u64,
    pub pressure: u64,
    pub date: String,
    pub time: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LocalizationResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Option<Localization>,
}

/// EFT localization table
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Localization {
    /// Localization table for UI elements.
    pub interface: HashMap<String, String>,
    /// Unknown type
    #[serde(rename = "enum")]
    pub enums: serde_json::Value,
    /// Localization table for errors.
    pub error: HashMap<String, String>,
    /// Localization table for automated messages (eg, from traders).
    pub mail: HashMap<String, StringOrInt>,
    /// Localization table for quest missions.
    pub quest: HashMap<String, Quest>,
    pub preset: HashMap<String, Preset>,
    /// Localization table for flea market categories.
    pub handbook: HashMap<String, String>,
    pub season: HashMap<String, String>,
    /// Localization table for items.
    pub templates: HashMap<String, Template>,
    /// Localization table for locations/maps.
    pub locations: HashMap<String, LocationLocalization>,
    pub banners: HashMap<String, BannerLocalization>,
    /// Localization table for traders.
    #[serde(rename = "trading")]
    pub traders: HashMap<String, Trader>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Quest {
    pub name: String,
    pub description: String,
    pub note: Option<String>,
    pub fail_message_text: String,
    pub started_message_text: String,
    pub success_message_text: String,
    pub conditions: HashMap<String, String>,
    pub location: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Preset {
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Template {
    pub name: String,
    pub short_name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LocationLocalization {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BannerLocalization {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Trader {
    pub full_name: String,
    pub first_name: String,
    pub nickname: String,
    pub location: String,
    pub description: String,
}

impl Tarkov {
    /// Get a list of all in-game items.
    pub async fn get_all_items(&self) -> Result<HashMap<String, Item>> {
        let url = format!("{}/client/items", PROD_ENDPOINT);
        let res: ItemsResponse = self.post_json(&url, &Request { crc: 0 }).await?;

        self.handle_error(res.error, res.data)
    }

    /// Get a list of all locations/maps.
    pub async fn get_all_locations(&self) -> Result<Locations> {
        let url = format!("{}/client/locations", PROD_ENDPOINT);
        let res: LocationsResponse = self.post_json(&url, &Request { crc: 0 }).await?;

        self.handle_error(res.error, res.data)
    }

    /// Get the current forecast and time.
    pub async fn get_weather(&self) -> Result<Weather> {
        let url = format!("{}/client/weather", PROD_ENDPOINT);
        let res: WeatherResponse = self.post_json(&url, &{}).await?;

        self.handle_error(res.error, res.data).map(|w| w.weather)
    }

    /// Get the localization table. Pass a valid ISO 639-1 language code.
    pub async fn get_i18n(&self, language: &str) -> Result<Localization> {
        let url = format!("{}/client/locale/{}", PROD_ENDPOINT, language);
        let res: LocalizationResponse = self.post_json(&url, &{}).await?;

        self.handle_error(res.error, res.data)
    }
}
