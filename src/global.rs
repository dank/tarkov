use crate::{ErrorResponse, Result, Tarkov, PROD_ENDPOINT};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::profile::Side;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ItemsRequest {
    crc: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemsResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Option<HashMap<String, Item>>,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "_name")]
    name: String,
    #[serde(rename = "_parent")]
    parent: String,
    #[serde(rename = "_type")]
    item_type: String,
    #[serde(rename = "_props")]
    props: Props,
    #[serde(rename = "_proto")]
    proto: Option<String>,
}

#[derive(Debug, Deserialize)]
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
    pub cartridges: Option<serde_json::Value>,
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
    #[serde(rename = "lootFilter")]
    pub loot_filter: Option<serde_json::Value>,
    #[serde(rename = "spawnRarity")]
    pub spawn_rarity: Option<String>,
    #[serde(rename = "minCountSpawn")]
    pub min_count_spawn: Option<u64>,
    #[serde(rename = "maxCountSpawn")]
    pub max_count_spawn: Option<u64>,
    #[serde(rename = ";ef")]
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
    pub chambers: Option<serde_json::Value>,
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
    pub effects_health: Option<serde_json::Value>,
    #[serde(rename = "effects_damage")]
    pub effects_damage: Option<serde_json::Value>,
    #[serde(rename = "effects_speed")]
    pub effects_speed: Option<serde_json::Value>,
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
    pub buffs: Option<serde_json::Value>,
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
    pub stack_slots: Option<serde_json::Value>,
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

#[derive(Debug, Deserialize)]
pub struct Prefab {
    path: String,
    rcid: String,
}

#[derive(Debug, Deserialize)]
pub struct Grid {
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "_name")]
    name: String,
    #[serde(rename = "_parent")]
    parent: String,
    #[serde(rename = "_props")]
    props: GridProps,
    #[serde(rename = "_proto")]
    proto: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridProps {
    filters: Vec<GridFilter>,
    cells_h: u64,
    cells_v: u64,
    min_count: u64,
    max_count: u64,
    max_weight: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GridFilter {
    filter: Vec<String>,
    excluded_filter: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Slot {
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "_name")]
    name: String,
    #[serde(rename = "_parent")]
    parent: String,
    #[serde(rename = "_props")]
    props: SlotProps,
    #[serde(rename = "_required")]
    required: bool,
    #[serde(rename = "_mergeSlotWithChildren")]
    merge_slot_with_children: bool,
    #[serde(rename = "_proto")]
    proto: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SlotProps {
    slot: Option<i64>,
    animation_index: Option<i64>,
    filters: Option<Vec<SlotFilter>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SlotFilter {
    slot: Option<i64>,
    animation_index: Option<i64>,
    filters: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub enum ArmorZone {
    Head,
    Chest,
    Stomach,
    LeftArm,
    RightArm,
    LeftLeg,
    RightLeg,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FireMode {
    Single,
    Burst,
    FullAuto,
}

#[derive(Debug, Deserialize)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Debug, Deserialize)]
pub struct Coordinate {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Deserialize)]
pub enum HeadSegment {
    Top,
    Nape,
    Ears,
    Eyes,
    Jaws
}

impl Tarkov {
    pub async fn get_items(&self) -> Result<()> {
        let url = format!("{}/client/items", PROD_ENDPOINT);
        let res: ItemsResponse = self.post_json(&url, &ItemsRequest { crc: 0 }).await?;

        println!("{:?}", res);

        Ok(())
    }
}
