use crate::{ErrorResponse, Result, Tarkov, PROD_ENDPOINT};

use crate::bad_json::StringOrInt;
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

/// Localization item
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Item {
    /// Item ID
    #[serde(rename = "_id")]
    pub id: String,
    /// Item name
    #[serde(rename = "_name")]
    pub name: String,
    /// Item parent ID
    #[serde(rename = "_parent")]
    pub parent: String,
    /// Item type
    #[serde(rename = "_type")]
    pub item_type: String,
    /// Item properties
    #[serde(rename = "_props")]
    pub props: Props,
    /// ?
    #[serde(rename = "_proto")]
    pub proto: Option<String>,
}

/// All item properties.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Props {
    /// Full item name
    pub name: Option<String>,
    /// Short item name
    pub short_name: Option<String>,
    /// Item description
    pub description: Option<String>,
    /// Item weight
    pub weight: Option<f64>,
    /// Item background color
    pub background_color: Option<String>,
    /// Item width
    pub width: Option<u64>,
    /// Item height
    pub height: Option<u64>,
    /// Item maximum stack size
    pub stack_max_size: Option<u64>,
    /// Item rarity
    pub rarity: Option<String>,
    /// Item spawn chance
    pub spawn_chance: Option<f64>,
    /// Item price?
    pub credits_price: Option<u64>,
    /// Item sound
    pub item_sound: Option<String>,
    /// Item prefab
    pub prefab: Option<Prefab>,
    /// Item prefab?
    pub use_prefab: Option<Prefab>,
    /// ?
    pub stack_objects_count: Option<u64>,
    /// ?
    pub not_shown_in_slot: Option<bool>,
    /// Item is examined by default.
    pub examined_by_default: Option<bool>,
    /// Time it takes to examine an item in seconds.
    pub examine_time: Option<u64>,
    /// Item cannot be deleted.
    pub is_undiscardable: Option<bool>,
    /// Item cannot be sold.
    pub is_unsaleable: Option<bool>,
    /// Item cannot be bought.
    pub is_unbuyable: Option<bool>,
    /// Item cannot be given?.
    pub is_ungivable: Option<bool>,
    /// Item locked after equipping?.
    #[serde(rename = "IsLockedafterEquip")]
    pub is_locked_after_equip: Option<bool>,
    /// Item is needed for quests.
    pub quest_item: Option<bool>,
    /// Experience for looting item.
    pub loot_experience: Option<u64>,
    /// Experience for examining item.
    pub examine_experience: Option<u64>,
    /// ?
    pub hide_entrails: Option<bool>,
    /// Item repair cost
    pub repair_cost: Option<u64>,
    /// Item repair speed
    pub repair_speed: Option<u64>,
    /// ?
    pub extra_size_left: Option<u64>,
    /// ?
    pub extra_size_right: Option<u64>,
    /// ?
    pub extra_size_up: Option<u64>,
    /// ?
    pub extra_size_down: Option<u64>,
    /// ?
    pub extra_size_force_add: Option<bool>,
    /// ?
    pub merges_with_children: Option<bool>,
    /// Item can be sold on the flea market.
    pub can_sell_on_ragfair: Option<bool>,
    /// Item can be traded on the flea market.
    pub can_require_on_ragfair: Option<bool>,
    /// Item banned from the flea market.
    pub banned_from_ragfair: Option<bool>,
    /// ?
    pub conflicting_items: Option<Vec<String>>,
    /// Item fixed price
    pub fixed_price: Option<bool>,
    /// Item cannot be looted.
    pub unlootable: Option<bool>,
    /// Item cannot be looted from slot.
    pub unlootable_from_slot: Option<String>,
    /// Item cannot be looted from side.
    pub unlootable_from_side: Option<Vec<Side>>,
    /// ?
    #[serde(rename = "ChangePriceCoef")]
    pub change_price_coefficient: Option<u64>,
    /// Item spawns locations
    pub allow_spawn_on_locations: Option<Vec<String>>,
    /// ?
    pub send_to_client: Option<bool>,
    /// ?
    pub animation_variants_number: Option<u64>,
    /// ?
    pub discarding_block: Option<bool>,
    /// ?
    pub max_resource: Option<u64>,
    /// ?
    pub resource: Option<u64>,
    /// ?
    pub dog_tag_qualities: Option<bool>,
    /// Item grids
    pub grids: Option<Vec<Grid>>,
    /// Item slots
    pub slots: Option<Vec<Slot>>,
    /// Items can be equipped during a raid.
    pub can_put_into_during_the_raid: Option<bool>,
    /// Item cannot be removed from slots during a raid.
    pub cant_remove_from_slots_during_raid: Option<Vec<String>>,
    /// Item key IDs
    pub key_ids: Option<Vec<String>>,
    /// Item tag color
    pub tag_color: Option<u64>,
    /// Item tag name
    pub tag_name: Option<String>,
    /// Item durability
    pub durability: Option<u64>,
    /// Weapon accuracy
    pub accuracy: Option<i64>,
    /// Weapon recoil
    pub recoil: Option<f64>,
    /// Weapon loudness
    pub loudness: Option<i64>,
    /// Weapon effective distance
    pub effective_distance: Option<u64>,
    /// Item ergonomics?
    pub ergonomics: Option<f64>,
    /// Item velocity
    pub velocity: Option<f64>,
    /// ?
    pub raid_moddable: Option<bool>,
    /// ?
    pub tool_moddable: Option<bool>,
    /// ?
    pub blocks_folding: Option<bool>,
    /// ?
    pub blocks_collapsible: Option<bool>,
    /// ?
    pub is_animated: Option<bool>,
    /// Weapon has a buttstock.
    pub has_shoulder_contact: Option<bool>,
    /// Weapon sighting range
    pub sighting_range: Option<u64>,
    /// Weapon firing modes
    pub modes_count: Option<u64>,
    /// Weapon muzzle mod type
    #[serde(rename = "muzzleModType")]
    pub muzzle_mod_type: Option<String>,
    /// Weapon sight mod type
    #[serde(rename = "sightModType")]
    pub sight_mod_type: Option<String>,
    /// Weapon has a telescopic sight equipped.
    #[serde(rename = "variableZoom")]
    pub variable_zoom: Option<bool>,
    /// Weapon telescopic sight magnification levels.
    #[serde(rename = "varZoomCount")]
    pub var_zoom_count: Option<u64>,
    /// Weapon telescopic sight magnification?
    #[serde(rename = "varZoomAdd")]
    pub var_zoom_add: Option<u64>,
    /// Weapon aiming sensitivity
    #[serde(rename = "aimingSensitivity")]
    pub aiming_sensitivity: Option<f64>,
    /// Weapon sight mode count
    pub sight_modes_count: Option<u64>,
    /// Weapon sight calibration distances
    pub optic_calibration_distances: Option<Vec<u64>>,
    /// ?
    pub intensity: Option<f64>,
    /// ?
    pub mask: Option<String>,
    /// ?
    pub mask_size: Option<f64>,
    /// Item noise intensity
    pub noise_intensity: Option<f64>,
    /// Item noise scale
    pub noise_scale: Option<u64>,
    /// Item color
    pub color: Option<Color>,
    /// ?
    pub diffuse_intensity: Option<f64>,
    /// ?
    pub has_hinge: Option<bool>,
    /// ?
    pub ramp_palette: Option<String>,
    /// ?
    pub depth_fade: Option<f64>,
    /// ?
    #[serde(rename = "RoughnessCoef")]
    pub roughness_coefficient: Option<f64>,
    /// ?
    #[serde(rename = "SpecularCoef")]
    pub specular_coefficient: Option<f64>,
    /// ?
    #[serde(rename = "MainTexColorCoef")]
    pub main_tex_color_coefficient: Option<f64>,
    /// ?
    pub minimum_temperature_value: Option<f64>,
    /// ?
    pub ramp_shift: Option<f64>,
    /// ?
    pub heat_min: Option<f64>,
    /// ?
    pub cold_max: Option<f64>,
    /// ?
    pub is_noisy: Option<bool>,
    /// ?
    pub is_fps_stuck: Option<bool>,
    /// ?
    pub is_glitch: Option<bool>,
    /// ?
    pub is_motion_blurred: Option<bool>,
    /// ?
    pub is_pixelated: Option<bool>,
    /// ?
    pub pixelation_block_count: Option<u64>,
    /// ?
    #[serde(rename = "magAnimationIndex")]
    pub mag_animation_index: Option<u64>,
    /// Weapon cartridge/ammo
    pub cartridges: Option<Vec<Cartridge>>,
    /// ?
    pub can_fast: Option<bool>,
    /// ?
    pub can_hit: Option<bool>,
    /// ?
    pub can_admin: Option<bool>,
    /// ?
    pub load_unload_modifier: Option<i64>,
    /// ?
    pub check_time_modifier: Option<i64>,
    /// ?
    pub check_override: Option<u64>,
    /// ?
    pub reload_mag_type: Option<String>,
    /// ?
    pub visible_ammo_ranges_string: Option<String>,
    /// Weapon has a buttstock.
    pub is_shoulder_contact: Option<bool>,
    /// Weapon stock is foldable.
    pub foldable: Option<bool>,
    /// Weapon stock is retractable?
    pub retractable: Option<bool>,
    /// ?
    pub size_reduce_right: Option<u64>,
    /// ?
    pub center_of_impact: Option<f64>,
    /// Shotgun shot dispersion
    pub shotgun_dispersion: Option<f64>,
    /// Weapon has a suppressor.
    pub is_silencer: Option<bool>,
    /// Item search sound
    pub search_sound: Option<String>,
    /// ?
    pub blocks_armor_vest: Option<bool>,
    /// ?
    #[serde(rename = "speedPenaltyPercent")]
    pub speed_penalty_percent: Option<i64>,
    /// ?
    pub grid_layout_name: Option<String>,
    /// ?
    pub spawn_filter: Option<Vec<String>>,
    /// Unknown type
    #[serde(rename = "containType")]
    pub contain_type: Option<serde_json::Value>,
    /// Item width in inventory.
    #[serde(rename = "sizeWidth")]
    pub size_width: Option<u64>,
    /// Item height in inventory.
    #[serde(rename = "sizeHeight")]
    pub size_height: Option<u64>,
    /// ?
    #[serde(rename = "isSecured")]
    pub is_secured: Option<bool>,
    /// ?
    #[serde(rename = "spawnTypes")]
    pub spawn_types: Option<String>,
    /// Unknown type
    #[serde(rename = "lootFilter")]
    pub loot_filter: Option<serde_json::Value>,
    /// Item spawn rarity
    #[serde(rename = "spawnRarity")]
    pub spawn_rarity: Option<String>,
    /// ?
    #[serde(rename = "minCountSpawn")]
    pub min_count_spawn: Option<u64>,
    /// ?
    #[serde(rename = "maxCountSpawn")]
    pub max_count_spawn: Option<u64>,
    /// Unknown type
    #[serde(rename = "openedByKeyID")]
    pub opened_by_key_id: Option<serde_json::Value>,
    /// Item rig layout name
    pub rig_layout_name: Option<String>,
    /// Item maximum durability
    pub max_durability: Option<u64>,
    /// Item armor zone
    #[serde(rename = "armorZone")]
    pub armor_zone: Option<Vec<ArmorZone>>,
    // #[serde(rename = "armorClass")]
    // pub armor_class: Option<u64>, // XXX: armorClass can be both String and int...
    /// ?
    #[serde(rename = "mousePenalty")]
    pub mouse_penalty: Option<i64>,
    /// ?
    #[serde(rename = "weaponErgonomicPenalty")]
    pub weapon_ergonomic_penalty: Option<i64>,
    /// ?
    pub blunt_throughput: Option<f64>,
    /// Item armor material
    pub armor_material: Option<String>,
    /// Weapon class
    #[serde(rename = "weapClass")]
    pub weapon_class: Option<String>,
    /// Weapon type
    #[serde(rename = "weapUseType")]
    pub weapon_use_type: Option<String>,
    /// Weapon ammo caliber
    pub ammo_caliber: Option<String>,
    /// ?
    pub operating_resource: Option<u64>,
    /// ?
    pub repair_complexity: Option<u64>,
    /// Item spawn minimum durability chance
    #[serde(rename = "durabSpawnMin")]
    pub durability_spawn_min: Option<u64>,
    /// Item spawn maximum durability chance
    #[serde(rename = "durabSpawnMax")]
    pub durability_spawn_max: Option<u64>,
    /// Weapon fast reload
    #[serde(rename = "isFastReload")]
    pub is_fast_reload: Option<bool>,
    /// Weapon recoil vertical force
    pub recoil_force_up: Option<u64>,
    /// Weapon recoil back force
    pub recoil_force_back: Option<u64>,
    /// ?
    pub convergence: Option<f64>,
    /// Weapon recoil angle
    pub recoil_angle: Option<u64>,
    /// Weapon fire modes
    #[serde(rename = "weapFireType")]
    pub weapon_fire_type: Option<Vec<FireMode>>,
    /// Weapon recoil dispersion rate
    #[serde(rename = "RecolDispersion")]
    pub recoil_dispersion: Option<u64>,
    /// Weapon fire mode
    #[serde(rename = "bFirerate")]
    pub firerate: Option<u64>,
    /// Weapon effective distance?
    #[serde(rename = "bEffDist")]
    pub eff_dist: Option<u64>,
    /// Weapon maximum sound distance
    #[serde(rename = "bHearDist")]
    pub hear_dist: Option<u64>,
    /// Weapon has a round in the chamber.
    #[serde(rename = "isChamberLoad")]
    pub is_chamber_load: Option<bool>,
    /// ?
    #[serde(rename = "chamberAmmoCount")]
    pub chamber_ammo_count: Option<u64>,
    /// Weapon bolt catch is engaged.
    #[serde(rename = "isBoltCatch")]
    pub is_bolt_catch: Option<bool>,
    /// Weapon magazine type?.
    #[serde(rename = "defMagType")]
    pub def_mag_type: Option<String>,
    /// Weapon ammo type?.
    #[serde(rename = "defAmmo")]
    pub def_ammo: Option<String>,
    /// Weapon chamber?.
    pub chambers: Option<Vec<Chamber>>,
    /// ?
    pub camera_recoil: Option<f64>,
    /// ?
    pub camera_snap: Option<f64>,
    /// Weapon reload mode
    pub reload_mode: Option<String>,
    /// ?
    pub aim_plane: Option<f64>,
    /// ?
    pub deviation_curve: Option<u64>,
    /// ?
    pub deviation_max: Option<u64>,
    /// ?
    #[serde(rename = "TacticalReloadStiffnes")]
    pub tactical_reload_stiffness: Option<Coordinate>,
    /// ?
    pub tactical_reload_fixation: Option<f64>,
    /// ?
    pub recoil_center: Option<Coordinate>,
    /// ?
    pub rotation_center: Option<Coordinate>,
    /// ?
    pub rotation_center_no_stock: Option<Coordinate>,
    /// ?
    pub folded_slot: Option<String>,
    /// ?
    pub compact_handling: Option<bool>,
    /// Item minimum repair degradation
    pub min_repair_degradation: Option<u64>,
    /// Item maximum repair degradation
    pub max_repair_degradation: Option<f64>,
    /// Weapon iron sight zero
    pub iron_sight_range: Option<u64>,
    /// Weapon's bolt catch must be engaged for external reload (pressing R).
    #[serde(rename = "MustBoltBeOpennedForExternalReload")]
    pub must_bolt_be_opened_for_external_reload: Option<bool>,
    /// Weapon's bolt catch must be engaged for internal reload (reload inside inventory).
    #[serde(rename = "MustBoltBeOpennedForInternalReload")]
    pub must_bolt_be_opened_for_internal_reload: Option<bool>,
    /// Weapon is bolt action operated.
    pub bolt_action: Option<bool>,
    /// ?
    pub hip_accuracy_restoration_delay: Option<f64>,
    /// ?
    pub hip_accuracy_restoration_speed: Option<u64>,
    /// ?
    #[serde(rename = "HipInnaccuracyGain")]
    pub hip_inaccuracy_gain: Option<f64>,
    /// ?
    pub manual_bolt_catch: Option<bool>,
    /// Item blocks earpiece.
    pub blocks_earpiece: Option<bool>,
    /// Item blocks eye wear.
    #[serde(rename = "BlocksEyewear")]
    pub blocks_eye_wear: Option<bool>,
    /// Item blocks head wear.
    #[serde(rename = "BlocksHeadwear")]
    pub blocks_head_wear: Option<bool>,
    /// Item blocks face cover.
    pub blocks_face_cover: Option<bool>,
    /// Time it takes to consume food.
    #[serde(rename = "foodUseTime")]
    pub food_use_time: Option<u64>,
    /// Food effect type.
    #[serde(rename = "foodEffectType")]
    pub food_effect_type: Option<String>,
    /// ?
    pub stimulator_buffs: Option<String>,
    /// Health effects on player
    #[serde(rename = "effects_health")]
    pub effects_health: Option<HealthEffects>,
    /// Damage effects on player
    #[serde(rename = "effects_damage")]
    pub effects_damage: Option<DamageEffects>,
    /// Speed effects on player
    #[serde(rename = "effects_speed")]
    pub effects_speed: Option<SpeedEffects>,
    /// Maximum item usage
    pub maximum_number_of_usage: Option<u64>,
    /// Knife hit delay
    #[serde(rename = "knifeHitDelay")]
    pub knife_hit_delay: Option<u64>,
    /// Knife slash rate
    #[serde(rename = "knifeHitSlashRate")]
    pub knife_hit_slash_rate: Option<u64>,
    /// Knife stab rate
    #[serde(rename = "knifeHitStabRate")]
    pub knife_hit_stab_rate: Option<u64>,
    /// Knife effective hit radius
    #[serde(rename = "knifeHitRadius")]
    pub knife_hit_radius: Option<f64>,
    /// Knife slash damage
    #[serde(rename = "knifeHitSlashDam")]
    pub knife_hit_slash_damage: Option<u64>,
    /// Knife stab damage
    #[serde(rename = "knifeHitStabDam")]
    pub knife_hit_stab_damage: Option<u64>,
    /// Knife durability
    #[serde(rename = "knifeDurab")]
    pub knife_durability: Option<u64>,
    /// ?
    pub primary_distance: Option<f64>,
    /// ?
    #[serde(rename = "SecondryDistance")]
    pub secondary_distance: Option<f64>,
    /// ?
    pub slash_penetration: Option<u64>,
    /// ?
    pub stab_penetration: Option<u64>,
    /// ?
    pub primary_consumption: Option<u64>,
    /// ?
    #[serde(rename = "SecondryConsumption")]
    pub secondary_consumption: Option<u64>,
    /// ?
    pub deflection_consumption: Option<u64>,
    /// ?
    pub config_path_str: Option<String>,
    /// ?
    pub max_markers_count: Option<u64>,
    /// ?
    #[serde(rename = "scaleMin")]
    pub scale_min: Option<f64>,
    /// ?
    #[serde(rename = "scaleMax")]
    pub scale_max: Option<f64>,
    /// Time it takes to consume medkit.
    #[serde(rename = "medUseTime")]
    pub med_use_time: Option<u64>,
    /// Medkit effect type
    #[serde(rename = "medEffectType")]
    pub med_effect_type: Option<String>,
    /// ?
    pub max_hp_resource: Option<u64>,
    /// ?
    #[serde(rename = "hpResourceRate")]
    pub hp_resource_rate: Option<u64>,
    /// ?
    pub max_efficiency: Option<u64>,
    /// ?
    pub addiction: Option<u64>,
    /// ?
    pub overdose: Option<u64>,
    /// ?
    pub overdose_recovery: Option<u64>,
    /// ?
    pub addiction_recovery: Option<u64>,
    // pub buffs: Option<serde_json::Value>, XXX: bad devs, multiple types...
    /// ?
    #[serde(rename = "apResource")]
    pub ap_resource: Option<u64>,
    /// ?
    #[serde(rename = "krResource")]
    pub kr_resource: Option<u64>,
    /// ?
    pub stack_min_random: Option<u64>,
    /// ?
    pub stack_max_random: Option<u64>,
    /// Ammo type
    #[serde(rename = "ammoType")]
    pub ammo_type: Option<String>,
    /// Ammo damage
    pub damage: Option<u64>,
    /// Ammo accuracy
    #[serde(rename = "ammoAccr")]
    pub ammo_accr: Option<i64>,
    /// Ammo recoil
    #[serde(rename = "ammoRec")]
    pub ammo_rec: Option<i64>,
    /// Ammo effective distance
    #[serde(rename = "ammoDist")]
    pub ammo_dist: Option<u64>,
    /// Buckshot bullet count?
    #[serde(rename = "buckshotBullets")]
    pub buckshot_bullets: Option<u64>,
    /// Ammo penetration power
    pub penetration_power: Option<u64>,
    /// Ammo ?
    #[serde(rename = "penetration_power_diviation")]
    pub penetration_power_deviation: Option<f64>,
    /// Ammo ?
    #[serde(rename = "ammoHear")]
    pub ammo_hear: Option<i64>,
    /// Ammo sound effect
    #[serde(rename = "ammoSfx")]
    pub ammo_sfx: Option<String>,
    /// Item chance of misfire
    pub misfire_chance: Option<f64>,
    /// ?
    pub min_fragments_count: Option<u64>,
    /// ?
    pub max_fragments_count: Option<u64>,
    /// ?
    #[serde(rename = "ammoShiftChance")]
    pub ammo_shift_chance: Option<u64>,
    /// Ammo casing name
    #[serde(rename = "casingName")]
    pub casing_name: Option<String>,
    /// Ammo casing ejection power
    #[serde(rename = "casingEjectPower")]
    pub casing_eject_power: Option<u64>,
    /// Ammo casing mass
    #[serde(rename = "casingMass")]
    pub casing_mass: Option<f64>,
    /// Ammo casing sound
    #[serde(rename = "casingSounds")]
    pub casing_sounds: Option<String>,
    /// Ammo projectile count
    pub projectile_count: Option<u64>,
    /// Ammo initial speed
    pub initial_speed: Option<u64>,
    /// Ammo penetration chance
    pub penetration_chance: Option<f64>,
    /// Ammo ricochet chance
    pub ricochet_chance: Option<f64>,
    /// Ammo fragmentation chance
    pub fragmentation_chance: Option<f64>,
    /// Ammo ballistic coefficient
    #[serde(rename = "BallisticCoeficient")]
    pub ballistic_coefficient: Option<f64>,
    /// ?
    pub deterioration: Option<u64>,
    /// ?
    pub speed_retardation: Option<f64>,
    /// Ammo is a tracer round
    pub tracer: Option<bool>,
    /// Tracer color
    pub tracer_color: Option<String>,
    /// Tracer distance
    pub tracer_distance: Option<f64>,
    /// Ammo armor damage
    pub armor_damage: Option<u64>,
    /// Ammo caliber
    pub caliber: Option<String>,
    /// ?
    pub stamina_burn_per_damage: Option<f64>,
    /// ?
    pub show_bullet: Option<bool>,
    /// ?
    pub has_grenader_component: Option<bool>,
    /// ?
    pub fuze_arm_time_sec: Option<f64>,
    /// Item explosion strength
    pub explosion_strength: Option<u64>,
    /// Item minimum explosion distance
    pub min_explosion_distance: Option<f64>,
    /// Item maximum explosion distance
    pub max_explosion_distance: Option<f64>,
    /// Explosion fragment count
    pub fragments_count: Option<u64>,
    /// Explosion fragment type
    pub fragment_type: Option<String>,
    /// ?
    pub show_hit_effect_on_explode: Option<bool>,
    /// Explosion type
    pub explosion_type: Option<String>,
    /// ?
    pub ammo_life_time_sec: Option<u64>,
    /// ?
    pub stack_slots: Option<Vec<StackSlot>>,
    /// Item type
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// ?
    #[serde(rename = "eqMin")]
    pub eq_min: Option<u64>,
    /// ?
    #[serde(rename = "eqMax")]
    pub eq_max: Option<u64>,
    /// ?
    #[serde(rename = "rate")]
    pub rate: Option<u64>,
    /// ?
    pub throw_type: Option<String>,
    /// ?
    pub strength: Option<u64>,
    /// ?
    pub contusion_distance: Option<u64>,
    /// ?
    #[serde(rename = "throwDamMax")]
    pub throw_dam_max: Option<u64>,
    /// ?
    pub expl_delay: Option<f64>,
    /// ?
    pub blindness: Option<Coordinate>,
    /// ?
    pub contusion: Option<Coordinate>,
    /// ?
    pub emit_time: Option<u64>,
    /// ?
    pub can_be_hidden_during_throw: Option<bool>,
    /// ?
    pub indestructibility: Option<f64>,
    /// ?
    #[serde(rename = "headSegments")]
    pub head_segments: Option<Vec<HeadSegment>>,
    /// ?
    pub face_shield_component: Option<bool>,
    /// ?
    pub face_shield_mask: Option<String>,
    /// ?
    pub material_type: Option<String>,
    /// ?
    pub ricochet_params: Option<Coordinate>,
    /// ?
    pub deaf_strength: Option<String>,
    /// ?
    pub distortion: Option<f64>,
    /// ?
    #[serde(rename = "CompressorTreshold")]
    pub compressor_threshold: Option<i64>,
    /// ?
    pub compressor_attack: Option<u64>,
    /// ?
    pub compressor_release: Option<u64>,
    /// ?
    pub compressor_gain: Option<u64>,
    /// ?
    pub cutoff_freq: Option<u64>,
    /// ?
    pub resonance: Option<f64>,
    /// ?
    pub compressor_volume: Option<i64>,
    /// Item ambient volume
    pub ambient_volume: Option<i64>,
    /// Item dry volume
    pub dry_volume: Option<i64>,
}

/// Item prefab
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Prefab {
    /// Prefab path
    pub path: String,
    /// ?
    pub rcid: String,
}

/// Item grid
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Grid {
    /// Grid ID
    #[serde(rename = "_id")]
    pub id: String,
    /// Grid name
    #[serde(rename = "_name")]
    pub name: String,
    /// Grid parent ID
    #[serde(rename = "_parent")]
    pub parent: String,
    /// Grid properties
    #[serde(rename = "_props")]
    pub props: GridProps,
    /// ?
    #[serde(rename = "_proto")]
    pub proto: String,
}

/// Item grid properties
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GridProps {
    /// Grid filters
    pub filters: Vec<GridFilter>,
    /// Grid height
    pub cells_h: u64,
    /// Grid width?
    #[serde(rename = "cellsV")]
    pub cells_w: u64,
    /// Minimum grid space
    pub min_count: u64,
    /// Maximum grid space
    pub max_count: u64,
    /// Grid maximum weight
    pub max_weight: u64,
}

/// Item grid filter
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct GridFilter {
    /// Grid filters
    pub filter: Vec<String>,
    /// Grid exclusion filter
    pub excluded_filter: Vec<String>,
}

/// Item slot
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Slot {
    /// Slot ID
    #[serde(rename = "_id")]
    pub id: String,
    /// Slot name
    #[serde(rename = "_name")]
    pub name: String,
    /// Slot parent ID
    #[serde(rename = "_parent")]
    pub parent: String,
    /// Slot properties
    #[serde(rename = "_props")]
    pub props: SlotProps,
    /// Slot is required
    #[serde(rename = "_required")]
    pub required: bool,
    /// Merge slot with children
    #[serde(rename = "_mergeSlotWithChildren")]
    pub merge_slot_with_children: bool,
    /// ?
    #[serde(rename = "_proto")]
    pub proto: String,
}

/// Item slot properties
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SlotProps {
    /// ?
    pub slot: Option<i64>,
    /// Slot animation
    pub animation_index: Option<i64>,
    /// Slot filters
    pub filters: Option<Vec<SlotFilter>>,
}

/// Item slot filter
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SlotFilter {
    /// ?
    pub slot: Option<i64>,
    /// Slot filter animation index
    pub animation_index: Option<i64>,
    /// Slot filter filters
    pub filters: Vec<String>,
}

/// Item cartridge/ammo
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Cartridge {
    /// Cartridge ID
    #[serde(rename = "_id")]
    pub id: String,
    /// Cartridge name
    #[serde(rename = "_name")]
    pub name: String,
    /// Cartridge parent ID
    #[serde(rename = "_parent")]
    pub parent: String,
    /// Maximum number of cartridges
    #[serde(rename = "_max_count")]
    pub max_count: u64,
    /// Cartridge properties
    #[serde(rename = "_props")]
    pub props: CartridgeProps,
    /// ?
    #[serde(rename = "_proto")]
    pub proto: String,
}

/// Cartridge properties
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct CartridgeProps {
    /// Cartridge filters
    pub filters: Vec<CartridgeFilter>,
}

/// Cartridge filter
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CartridgeFilter {
    /// Cartridge filter filters?
    pub filter: Vec<String>,
}

/// Item armor zone
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum ArmorZone {
    /// Head
    Head,
    /// Chest
    Chest,
    /// Stomach
    Stomach,
    /// Left arm
    LeftArm,
    /// Right arm
    RightArm,
    /// Left leg
    LeftLeg,
    /// Right leg
    RightLeg,
}

/// Item fire mode
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FireMode {
    /// Single fire
    Single,
    /// Burst mode
    Burst,
    /// Full auto
    FullAuto,
}

/// Item chamber
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Chamber {
    /// Chamber ID
    #[serde(rename = "_id")]
    pub id: String,
    /// Chamber name
    #[serde(rename = "_name")]
    pub name: String,
    /// Chamber parent ID
    #[serde(rename = "_parent")]
    pub parent: String,
    /// Chamber properties
    #[serde(rename = "_props")]
    pub props: ChamberProps,
    /// Chamber is required
    #[serde(rename = "_required")]
    pub required: bool,
    /// Merge chamber slot with children
    #[serde(rename = "_mergeSlotWithChildren")]
    pub merge_slot_with_children: bool,
    /// ?
    #[serde(rename = "_proto")]
    pub proto: String,
}

/// Chamber properties
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChamberProps {
    /// Chamber filters
    pub filters: Vec<ChamberFilter>,
}

/// Chamber filter
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ChamberFilter {
    /// Chamber filter filters?
    pub filter: Vec<String>,
}

/// RGB color
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Color {
    /// Red
    pub r: u8,
    /// Green
    pub g: u8,
    /// Blue
    pub b: u8,
    /// Alpha
    pub a: u8,
}

/// 3D Coordinate
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Coordinate {
    /// x plane
    pub x: f64,
    /// y plane
    pub y: f64,
    /// z plane
    pub z: f64,
}

/// Head parts
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum HeadSegment {
    /// Top of head
    Top,
    /// Nape
    Nape,
    /// Ears
    Ears,
    /// Eyes
    Eyes,
    /// Jaws
    Jaws,
}

/// Health effects on player
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct HealthEffects {
    /// Body effects
    pub common: Health,
    /// Head effects
    pub head: Health,
    /// Left arm effects
    pub arm_left: Health,
    /// Right arm effects
    pub arm_right: Health,
    /// Chest effects
    pub chest: Health,
    /// Stomach effects
    pub tummy: Health,
    /// Left leg effects
    pub leg_left: Health,
    /// Right leg effects
    pub leg_right: Health,
    /// Energy effects
    pub energy: Health,
    /// Hydration effects
    pub hydration: Health,
}

/// Health effect
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Health {
    /// Effect value
    pub value: i64,
    /// Effect percent
    pub percent: bool,
    /// ?
    pub time: u64,
    /// Effect duration
    pub duration: u64,
}

/// Damage effects on player
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DamageEffects {
    /// Bloodloss effect
    pub bloodloss: Damage,
    /// Fracture effect
    pub fracture: Damage,
    /// Pain effect
    pub pain: Damage,
    /// Contusion effect
    pub contusion: Damage,
    /// Toxication effect
    pub toxication: Damage,
    /// Radiation exposure
    #[serde(rename = "radExposure")]
    pub radiation_exposure: Damage,
}

/// Damage effect
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Damage {
    /// ?
    pub remove: bool,
    /// ?
    pub time: u64,
    /// Damage effect duration
    pub duration: u64,
    /// ?
    pub fade_out: Option<u64>,
    /// ?
    pub cost: Option<u64>,
    /// Damage minimum health penalty
    pub health_penalty_min: Option<u64>,
    /// Damage maximum health penalty
    pub health_penalty_max: Option<u64>,
}

/// Speed effects on player
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SpeedEffects {
    /// Mobility effect
    pub mobility: Speed,
    /// Recoil effect
    pub recoil: Speed,
    /// Reload effect
    pub reload_speed: Speed,
    /// Loot effect
    pub loot_speed: Speed,
    /// Unlock effect
    pub unlock_speed: Speed,
}

/// Speed effect
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Speed {
    /// Effect value
    pub value: i64,
    /// Effect percent
    pub percent: bool,
    /// ?
    pub time: u64,
    /// Effect duration
    pub duration: u64,
}

/// ?
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct StackSlot {
    /// Stack slot ID
    #[serde(rename = "_id")]
    pub id: String,
    /// Stack slot name
    #[serde(rename = "_name")]
    pub name: String,
    /// Stack slot parent ID
    #[serde(rename = "_parent")]
    pub parent: String,
    /// Maximum stack slots
    #[serde(rename = "_max_count")]
    pub max_count: u64,
    /// Stack slot properties
    #[serde(rename = "_props")]
    pub props: StackSlotProps,
    /// ?
    #[serde(rename = "_proto")]
    pub proto: String,
}

/// Stack slot properties
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct StackSlotProps {
    /// Stack slot filters
    pub filters: Vec<StackSlotFilter>,
}

/// Stack slot filter
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StackSlotFilter {
    /// Stack slot filter filters?
    pub filter: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct LocationsResponse {
    #[serde(flatten)]
    error: ErrorResponse,
    data: Option<Locations>,
}

/// All in-game locations
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Locations {
    /// Locations
    pub locations: HashMap<String, Location>,
    /// Location paths
    pub paths: Vec<Path>,
}

/// In-game location
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Location {
    /// Location is enabled
    pub enabled: bool,
    /// Location is locked
    pub locked: bool,
    /// Insurance is available
    pub insurance: bool,
    /// ?
    pub safe_location: bool,
    /// Location name
    pub name: String,
    /// Location description
    pub description: String,
    /// Location scene prefab
    pub scene: Scene,
    /// Location area
    pub area: f64,
    /// Required level for location access
    pub required_player_level: u64,
    /// ?
    #[serde(rename = "surv_gather_minutes")]
    pub surv_gather_minutes: u64,
    /// Minimum players on location
    pub min_players: u64,
    /// Maximum players on location
    pub max_players: u64,
    /// ?
    #[serde(rename = "sav_gather_minutes")]
    pub scav_gather_minutes: u64,
    /// Number of extraction points
    #[serde(rename = "exit_count")]
    pub exit_count: u64,
    /// ?
    #[serde(rename = "exit_access_time")]
    pub exit_access_time: u64,
    /// ?
    #[serde(rename = "exit_time")]
    pub exit_time: u64,
    /// Location preview
    pub preview: Preview,
    /// Location icon X
    pub icon_x: u64,
    /// Location icon Y
    pub icon_y: u64,
    /// Unknown type
    #[serde(rename = "filter_ex")]
    pub filter_ex: Vec<serde_json::Value>,
    /// NPC waves on location
    #[serde(rename = "waves")]
    pub waves: Vec<Wave>,
    /// Unknown type
    #[serde(rename = "limits")]
    pub limits: Vec<serde_json::Value>,
    /// Average play time on location
    pub average_play_time: u64,
    /// Average player level on location
    pub average_player_level: u64,
    /// Extraction time limit
    #[serde(rename = "escape_time_limit")]
    pub escape_time_limit: u64,
    /// Location rules
    pub rules: String,
    /// Location is secret
    pub is_secret: bool,
    /// Unknown type
    #[serde(rename = "doors")]
    pub doors: Vec<serde_json::Value>,
    /// ?
    #[serde(rename = "tmp_location_field_remove_me")]
    pub tmp_location_field_remove_me: u64,
    /// Minimum distance to extraction from spawn
    #[serde(rename = "MinDistToExitPoint")]
    pub min_distance_to_exit_point: u64,
    /// Minimum distance to "free point" from spawn
    #[serde(rename = "MinDistToFreePoint")]
    pub min_distance_to_free_point: u64,
    /// Maximum distance to "free point" from spawn
    #[serde(rename = "MaxDistToFreePoint")]
    pub max_distance_to_free_point: u64,
    /// Maximum number of bots per zone
    pub max_bot_per_zone: u64,
    /// Location open zones
    pub open_zones: String,
    /// ?
    #[serde(rename = "OcculsionCullingEnabled")]
    pub occlusion_culling_enabled: bool,
    /// Location loot chance modifier
    pub global_loot_chance_modifier: f64,
    /// ?
    pub old_spawn: bool,
    /// ?
    pub new_spawn: bool,
    /// Maximum number of bots
    pub bot_max: u64,
    /// ?
    pub bot_start: u64,
    /// ?
    pub bot_stop: u64,
    /// ?
    pub bot_max_time_player: u64,
    /// ?
    pub bot_spawn_time_on_min: u64,
    /// ?
    pub bot_spawn_time_on_max: u64,
    /// ?
    pub bot_spawn_time_off_min: u64,
    /// ?
    pub bot_spawn_time_off_max: u64,
    /// ?
    pub bot_max_player: u64,
    /// Bot difficulty is "easy"
    pub bot_easy: u64,
    /// Bot difficulty is "normal"
    pub bot_normal: u64,
    /// Bot difficulty is "hard"
    pub bot_hard: u64,
    /// Bot difficulty is "impossible"
    pub bot_impossible: u64,
    /// ?
    pub bot_assault: u64,
    /// ?
    pub bot_marksman: u64,
    /// ?
    pub disabled_scav_exits: String,
    /// ?
    pub access_keys: Vec<String>,
    /// ?
    pub min_max_bots: Vec<MinMaxBot>,
    /// Bot stats modifier
    pub bot_location_modifier: BotLocationModifier,
    /// Extraction points
    #[serde(rename = "exits")]
    pub exits: Vec<Exit>,
    /// Location disabled for SCAVs
    pub disabled_for_scav: bool,
    /// Boss spawns
    pub boss_location_spawn: Vec<BossSpawn>,
    /// Location ID?
    #[serde(rename = "Id")]
    pub name_id: String,
    /// Location ID?
    #[serde(rename = "_Id")]
    pub id: String,
    /// Unknown type
    pub loot: Vec<serde_json::Value>,
    /// Unknown type
    pub spawn_areas: Vec<serde_json::Value>,
    /// Location banners
    pub banners: Vec<Banner>,
}

/// Location scene prefab
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Scene {
    /// Scene path
    pub path: String,
    /// ?
    pub rcid: String,
}

/// Location path?
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Path {
    /// ?
    pub source: String,
    /// ?
    pub destination: String,
}

/// Location preview
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Preview {
    /// Location preview path
    pub path: String,
    /// ?
    pub rcid: String,
}

/// Bot wave
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Wave {
    /// Number of bots in wave?
    #[serde(rename = "number")]
    pub number: u64,
    /// Minimum wave time
    #[serde(rename = "time_min")]
    pub time_min: u64,
    /// Maximum wave time
    #[serde(rename = "time_max")]
    pub time_max: u64,
    /// ?
    #[serde(rename = "slots_min")]
    pub slots_min: u64,
    /// ?
    #[serde(rename = "slots_max")]
    pub slots_max: u64,
    /// Wave spawn point
    pub spawn_points: String,
    /// Bot side
    pub bot_side: Side,
    /// Bot difficulty
    pub bot_preset: BotDifficulty,
    /// ?
    #[serde(rename = "isPlayers")]
    pub is_players: bool,
    /// ?
    pub wild_spawn_type: String,
}

/// ?
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct MinMaxBot {
    /// ?
    pub min: u64,
    /// ?
    pub max: u64,
    /// ?
    #[serde(rename = "WildSpawnType")]
    pub wild_spawn_type: String,
}

/// Location bot stat modifier
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BotLocationModifier {
    /// Bot accuracy speed modifier
    pub accuracy_speed: u64,
    /// Bot scattering modifier
    pub scattering: u64,
    /// Bot gain sight modifier
    pub gain_sight: u64,
    /// Bot marksmen accuracy modifier
    #[serde(rename = "MarksmanAccuratyCoef")]
    pub marksman_accuracy_coefficient: u64,
    /// Bot visible distance modifier
    pub visible_distance: u64,
}

/// Extraction point
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Exit {
    /// Extraction name
    pub name: String,
    /// ?
    pub entry_points: String,
    /// Extraction availability chance
    pub chance: u64,
    /// Extraction minimum availability time
    pub min_time: u64,
    /// Extraction maximum availability time
    pub max_time: u64,
    /// ?
    pub players_count: u64,
    /// ?
    pub exfiltration_time: u64,
    /// ?
    pub passage_requirement: Option<String>,
    /// ?
    pub exfiltration_type: Option<String>,
    /// ?
    pub required_slot: Option<String>,
    /// ?
    pub count: Option<u64>,
    /// Extraction ID
    pub id: String,
    /// ?
    pub requirement_tip: Option<String>,
}

/// Boss bots
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BossSpawn {
    /// Boss name
    #[serde(rename = "BossName")]
    pub name: String,
    /// Boss spawn chance
    #[serde(rename = "BossChance")]
    pub chance: u64,
    /// Boss spawn zone
    #[serde(rename = "BossZone")]
    pub zone: String,
    /// ?
    #[serde(rename = "BossPlayer")]
    pub player: bool,
    /// Boss difficulty
    #[serde(rename = "BossDifficult")]
    pub difficulty: BotDifficulty,
    /// Boss escort type
    #[serde(rename = "BossEscortType")]
    pub escort_type: String,
    /// Boss escort difficulty
    #[serde(rename = "BossEscortDifficult")]
    pub escort_difficulty: BotDifficulty,
    /// Number of boss escorts
    #[serde(rename = "BossEscortAmount")]
    pub escort_amount: String,
    /// ?
    pub time: i64,
}

/// Boss difficulty
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BotDifficulty {
    /// Easy difficulty
    Easy,
    /// Normal difficulty
    Normal,
    /// Hard difficulty
    Hard,
    /// Impossible difficulty
    Impossible,
}

/// Location banner
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Banner {
    /// Banner ID
    pub id: String,
    /// Banner picture
    pub pic: BannerPic,
}

/// Location banner picture
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct BannerPic {
    /// Picture path
    pub path: String,
    /// ?
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

/// Weather
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Weather {
    /// Timestamp
    pub timestamp: u64,
    /// Cloud
    pub cloud: f64,
    /// Wind speed
    pub wind_speed: u64,
    /// Wind direction
    pub wind_direction: u64,
    /// Wind intensity
    pub wind_gustiness: f64,
    /// Rain
    pub rain: u64,
    /// Rain intensity
    pub rain_intensity: f64,
    /// Fog
    pub fog: f64,
    /// Temperature
    pub temp: u64,
    /// Atmospheric pressure
    pub pressure: u64,
    /// Date
    pub date: String,
    /// Time
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
    /// ?
    pub preset: HashMap<String, Preset>,
    /// Localization table for flea market categories.
    pub handbook: HashMap<String, String>,
    /// Localization table for seasons.
    pub season: HashMap<String, String>,
    /// Localization table for items.
    pub templates: HashMap<String, ItemLocalization>,
    /// Localization table for locations/maps.
    pub locations: HashMap<String, LocationLocalization>,
    /// Localization table for banners.
    pub banners: HashMap<String, BannerLocalization>,
    /// Localization table for traders.
    #[serde(rename = "trading")]
    pub traders: HashMap<String, TraderLocalization>,
}

/// Quest
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Quest {
    /// Quest name
    pub name: String,
    /// Quest description
    pub description: String,
    /// Quest note
    pub note: Option<String>,
    /// Quest fail message
    pub fail_message_text: String,
    /// Quest start message
    pub started_message_text: String,
    /// Quest success message
    pub success_message_text: String,
    /// Quest conditions
    pub conditions: HashMap<String, String>,
    /// Quest location
    pub location: String,
}

/// ?
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Preset {
    /// Preset name
    pub name: Option<String>,
}

/// Item localization
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ItemLocalization {
    /// Item name
    pub name: String,
    /// Item short name
    pub short_name: String,
    /// Item description
    pub description: String,
}

/// Location localization
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LocationLocalization {
    /// Location name
    pub name: String,
    /// Location description
    pub description: String,
}

/// Banner localization
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BannerLocalization {
    /// Banner name
    pub name: Option<String>,
    /// Banner description
    pub description: Option<String>,
}

/// Trader localization
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TraderLocalization {
    /// Trader full name
    pub full_name: String,
    /// Trader first name
    pub first_name: String,
    /// Trader nickname
    pub nickname: String,
    /// Trader location
    pub location: String,
    /// Trader description
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
