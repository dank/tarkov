use crate::{Result, Tarkov, PROD_ENDPOINT, ErrorResponse};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub name: Option<String>, // Name
    pub short_name: Option<String>, // ShortName
    pub description: Option<String>, // Description
    pub weight: Option<f64>, // Weight
    pub background_color: Option<String>, // BackgroundColor
    pub width: Option<u64>, // Width
    pub height: Option<u64>, // Height
    pub stack_max_size: Option<u64>, // StackMaxSize
    pub rarity: Option<String>, // Rarity
    pub spawn_chance: Option<f64>, // SpawnChance
    pub credits_price: Option<u64>, // CreditsPrice
    pub item_sound: Option<String>, // ItemSound
    pub prefab: Option<Prefab>, // Prefab
    pub use_prefab: Option<Prefab>, // UsePrefab
    pub stack_objects_count: Option<u64>, // StackObjectsCount
    pub not_shown_in_slot: Option<bool>, // NotShownInSlot
    pub examined_by_default: Option<bool>, // ExaminedByDefault
    pub examine_time: Option<u64>, // ExamineTime
    pub is_undiscardable: Option<bool>, // IsUndiscardable
    pub is_unsaleable: Option<bool>, // IsUnsaleable
    pub is_unbuyable: Option<bool>, // IsUnbuyable
    pub is_ungivable: Option<bool>, // IsUngivable
    #[serde(rename = "IsLockedafterEquip")]
    pub is_locked_after_equip: Option<bool>, // IsLockedafterEquip
    pub quest_item: Option<bool>, // QuestItem
    pub loot_experience: Option<u64>, // LootExperience
    pub examine_experience: Option<u64>, // ExamineExperience
    pub hide_entrails: Option<bool>, // HideEntrails
    pub repair_cost: Option<u64>, // RepairCost
    pub repair_speed: Option<u64>, // RepairSpeed
    pub extra_size_left: Option<u64>, // ExtraSizeLeft
    pub extra_size_right: Option<u64>, // ExtraSizeRight
    pub extra_size_up: Option<u64>, // ExtraSizeUp
    pub extra_size_down: Option<u64>, // ExtraSizeDown
    pub extra_size_force_add: Option<bool>, // ExtraSizeForceAdd
    pub merges_with_children: Option<bool>, // MergesWithChildren
    pub can_sell_on_ragfair: Option<bool>, // CanSellOnRagfair
    pub can_require_on_ragfair: Option<bool>, // CanRequireOnRagfair
    pub banned_from_ragfair: Option<bool>, // BannedFromRagfair
    pub conflicting_items: Option<serde_json::Value>, // ConflictingItems
    pub fixed_price: Option<bool>, // FixedPrice
    pub unlootable: Option<bool>, // Unlootable
    pub unlootable_from_slot: Option<String>, // UnlootableFromSlot
    pub unlootable_from_side: Option<serde_json::Value>, // UnlootableFromSide
    #[serde(rename = "ChangePriceCoef")]
    pub change_price_coefficient: Option<u64>, // ChangePriceCoef
    pub allow_spawn_on_locations: Option<serde_json::Value>, // AllowSpawnOnLocations
    pub send_to_client: Option<bool>, // SendToClient
    pub animation_variants_number: Option<u64>, // AnimationVariantsNumber
    pub discarding_block: Option<bool>, // DiscardingBlock
    pub max_resource: Option<u64>, // MaxResource
    pub resource: Option<u64>, // Resource
    pub dog_tag_qualities: Option<bool>, // DogTagQualities
    pub grids: Option<serde_json::Value>, // Grids
    pub slots: Option<serde_json::Value>, // Slots
    pub can_put_into_during_the_raid: Option<bool>, // CanPutIntoDuringTheRaid
    pub cant_remove_from_slots_during_raid: Option<serde_json::Value>, // CantRemoveFromSlotsDuringRaid
    pub key_ids: Option<serde_json::Value>, // KeyIds
    pub tag_color: Option<u64>, // TagColor
    pub tag_name: Option<String>, // TagName
    pub durability: Option<u64>, // Durability
    pub accuracy: Option<i64>, // Accuracy
    pub recoil: Option<f64>, // Recoil
    pub loudness: Option<i64>, // Loudness
    pub effective_distance: Option<u64>, // EffectiveDistance
    pub ergonomics: Option<f64>, // Ergonomics
    pub velocity: Option<f64>, // Velocity
    pub raid_moddable: Option<bool>, // RaidModdable
    pub tool_moddable: Option<bool>, // ToolModdable
    pub blocks_folding: Option<bool>, // BlocksFolding
    pub blocks_collapsible: Option<bool>, // BlocksCollapsible
    pub is_animated: Option<bool>, // IsAnimated
    pub has_shoulder_contact: Option<bool>, // HasShoulderContact
    pub sighting_range: Option<u64>, // SightingRange
    pub modes_count: Option<u64>, // ModesCount
    #[serde(rename = "muzzleModType")]
    pub muzzle_mod_type: Option<String>, // muzzleModType
    #[serde(rename = "sightModType")]
    pub sight_mod_type: Option<String>, // sightModType
    #[serde(rename = "variableZoom")]
    pub variable_zoom: Option<bool>, // variableZoom
    #[serde(rename = "varZoomCount")]
    pub var_zoom_count: Option<u64>, // varZoomCount
    #[serde(rename = "varZoomAdd")]
    pub var_zoom_add: Option<u64>, // varZoomAdd
    #[serde(rename = "aimingSensitivity")]
    pub aiming_sensitivity: Option<f64>, // aimingSensitivity
    pub sight_modes_count: Option<u64>, // SightModesCount
    pub optic_calibration_distances: Option<serde_json::Value>, // OpticCalibrationDistances
    pub intensity: Option<f64>, // Intensity
    pub mask: Option<String>, // Mask
    pub mask_size: Option<f64>, // MaskSize
    pub noise_intensity: Option<f64>, // NoiseIntensity
    pub noise_scale: Option<u64>, // NoiseScale
    pub color: Option<serde_json::Value>, // Color
    pub diffuse_intensity: Option<f64>, // DiffuseIntensity
    pub has_hinge: Option<bool>, // HasHinge
    pub ramp_palette: Option<String>, // RampPalette
    pub depth_fade: Option<f64>, // DepthFade
    #[serde(rename = "RoughnessCoef")]
    pub roughness_coefficient: Option<f64>, // RoughnessCoef
    #[serde(rename = "SpecularCoef")]
    pub specular_coefficient: Option<f64>, // SpecularCoef
    #[serde(rename = "MainTexColorCoef")]
    pub main_tex_color_coefficient: Option<f64>, // MainTexColorCoef
    pub minimum_temperature_value: Option<f64>, // MinimumTemperatureValue
    pub ramp_shift: Option<f64>, // RampShift
    pub heat_min: Option<f64>, // HeatMin
    pub cold_max: Option<f64>, // ColdMax
    pub is_noisy: Option<bool>, // IsNoisy
    pub is_fps_stuck: Option<bool>, // IsFpsStuck
    pub is_glitch: Option<bool>, // IsGlitch
    pub is_motion_blurred: Option<bool>, // IsMotionBlurred
    pub is_pixelated: Option<bool>, // IsPixelated
    pub pixelation_block_count: Option<u64>, // PixelationBlockCount
    #[serde(rename = "magAnimationIndex")]
    pub mag_animation_index: Option<u64>, // magAnimationIndex
    pub cartridges: Option<serde_json::Value>, // Cartridges
    pub can_fast: Option<bool>, // CanFast
    pub can_hit: Option<bool>, // CanHit
    pub can_admin: Option<bool>, // CanAdmin
    pub load_unload_modifier: Option<i64>, // LoadUnloadModifier
    pub check_time_modifier: Option<i64>, // CheckTimeModifier
    pub check_override: Option<u64>, // CheckOverride
    pub reload_mag_type: Option<String>, // ReloadMagType
    pub visible_ammo_ranges_string: Option<String>, // VisibleAmmoRangesString
    pub is_shoulder_contact: Option<bool>, // IsShoulderContact
    pub foldable: Option<bool>, // Foldable
    pub retractable: Option<bool>, // Retractable
    pub size_reduce_right: Option<u64>, // SizeReduceRight
    pub center_of_impact: Option<f64>, // CenterOfImpact
    pub shotgun_dispersion: Option<f64>, // ShotgunDispersion
    pub is_silencer: Option<bool>, // IsSilencer
    pub search_sound: Option<String>, // SearchSound
    pub blocks_armor_vest: Option<bool>, // BlocksArmorVest
    #[serde(rename = "speedPenaltyPercent")]
    pub speed_penalty_percent: Option<i64>, // speedPenaltyPercent
    pub grid_layout_name: Option<String>, // GridLayoutName
    pub spawn_filter: Option<serde_json::Value>, // SpawnFilter
    #[serde(rename = "containType")]
    pub contain_type: Option<serde_json::Value>, // containType
    #[serde(rename = "sizeWidth")]
    pub size_width: Option<u64>, // sizeWidth
    #[serde(rename = "sizeHeight")]
    pub size_height: Option<u64>, // sizeHeight
    #[serde(rename = "isSecured")]
    pub is_secured: Option<bool>, // isSecured
    #[serde(rename = "spawnTypes")]
    pub spawn_types: Option<String>, // spawnTypes
    #[serde(rename = "lootFilter")]
    pub loot_filter: Option<serde_json::Value>, // lootFilter
    #[serde(rename = "spawnRarity")]
    pub spawn_rarity: Option<String>, // spawnRarity
    #[serde(rename = "minCountSpawn")]
    pub min_count_spawn: Option<u64>, // minCountSpawn
    #[serde(rename = "maxCountSpawn")]
    pub max_count_spawn: Option<u64>, // maxCountSpawn
    #[serde(rename = "openedByKeyID")]
    pub opened_by_key_i_d: Option<serde_json::Value>, // openedByKeyID
    pub rig_layout_name: Option<String>, // RigLayoutName
    pub max_durability: Option<u64>, // MaxDurability
    #[serde(rename = "armorZone")]
    pub armor_zone: Option<serde_json::Value>, // armorZone
    // #[serde(rename = "armorClass")]
    // pub armor_class: Option<u64>, // armorClass can be both String and int...
    #[serde(rename = "mousePenalty")]
    pub mouse_penalty: Option<i64>, // mousePenalty
    #[serde(rename = "weaponErgonomicPenalty")]
    pub weapon_ergonomic_penalty: Option<i64>, // weaponErgonomicPenalty
    pub blunt_throughput: Option<f64>, // BluntThroughput
    pub armor_material: Option<String>, // ArmorMaterial
    #[serde(rename = "weapClass")]
    pub weapon_class: Option<String>, // weapClass
    #[serde(rename = "weapUseType")]
    pub weapon_use_type: Option<String>, // weapUseType
    pub ammo_caliber: Option<String>, // ammoCaliber
    pub operating_resource: Option<u64>, // OperatingResource
    pub repair_complexity: Option<u64>, // RepairComplexity
    #[serde(rename = "durabSpawnMin")]
    pub durability_spawn_min: Option<u64>, // durabSpawnMin
    #[serde(rename = "durabSpawnMax")]
    pub durability_spawn_max: Option<u64>, // durabSpawnMax
    #[serde(rename = "isFastReload")]
    pub is_fast_reload: Option<bool>, // isFastReload
    pub recoil_force_up: Option<u64>, // RecoilForceUp
    pub recoil_force_back: Option<u64>, // RecoilForceBack
    pub convergence: Option<f64>, // Convergence
    pub recoil_angle: Option<u64>, // RecoilAngle
    #[serde(rename = "weapFireType")]
    pub weapon_fire_type: Option<serde_json::Value>, // weapFireType
    #[serde(rename = "RecolDispersion")]
    pub recoil_dispersion: Option<u64>, // RecolDispersion
    #[serde(rename = "bFirerate")]
    pub firerate: Option<u64>, // bFirerate
    #[serde(rename = "bEffDist")]
    pub eff_dist: Option<u64>, // bEffDist
    #[serde(rename = "bHearDist")]
    pub hear_dist: Option<u64>, // bHearDist
    #[serde(rename = "isChamberLoad")]
    pub is_chamber_load: Option<bool>, // isChamberLoad
    #[serde(rename = "chamberAmmoCount")]
    pub chamber_ammo_count: Option<u64>, // chamberAmmoCount
    #[serde(rename = "isBoltCatch")]
    pub is_bolt_catch: Option<bool>, // isBoltCatch
    #[serde(rename = "defMagType")]
    pub def_mag_type: Option<String>, // defMagType
    #[serde(rename = "defAmmo")]
    pub def_ammo: Option<String>, // defAmmo
    pub chambers: Option<serde_json::Value>, // Chambers
    pub camera_recoil: Option<f64>, // CameraRecoil
    pub camera_snap: Option<f64>, // CameraSnap
    pub reload_mode: Option<String>, // ReloadMode
    pub aim_plane: Option<f64>, // AimPlane
    pub deviation_curve: Option<u64>, // DeviationCurve
    pub deviation_max: Option<u64>, // DeviationMax
    pub tactical_reload_stiffnes: Option<serde_json::Value>, // TacticalReloadStiffnes
    pub tactical_reload_fixation: Option<f64>, // TacticalReloadFixation
    pub recoil_center: Option<serde_json::Value>, // RecoilCenter
    pub rotation_center: Option<serde_json::Value>, // RotationCenter
    pub rotation_center_no_stock: Option<serde_json::Value>, // RotationCenterNoStock
    pub folded_slot: Option<String>, // FoldedSlot
    pub compact_handling: Option<bool>, // CompactHandling
    pub min_repair_degradation: Option<u64>, // MinRepairDegradation
    pub max_repair_degradation: Option<f64>, // MaxRepairDegradation
    pub iron_sight_range: Option<u64>, // IronSightRange
    pub must_bolt_be_openned_for_external_reload: Option<bool>, // MustBoltBeOpennedForExternalReload
    pub must_bolt_be_openned_for_internal_reload: Option<bool>, // MustBoltBeOpennedForInternalReload
    pub bolt_action: Option<bool>, // BoltAction
    pub hip_accuracy_restoration_delay: Option<f64>, // HipAccuracyRestorationDelay
    pub hip_accuracy_restoration_speed: Option<u64>, // HipAccuracyRestorationSpeed
    pub hip_innaccuracy_gain: Option<f64>, // HipInnaccuracyGain
    pub manual_bolt_catch: Option<bool>, // ManualBoltCatch
    pub blocks_earpiece: Option<bool>, // BlocksEarpiece
    pub blocks_eyewear: Option<bool>, // BlocksEyewear
    pub blocks_headwear: Option<bool>, // BlocksHeadwear
    pub blocks_face_cover: Option<bool>, // BlocksFaceCover
    #[serde(rename = "foodUseTime")]
    pub food_use_time: Option<u64>, // foodUseTime
    #[serde(rename = "foodEffectType")]
    pub food_effect_type: Option<String>, // foodEffectType
    pub stimulator_buffs: Option<String>, // StimulatorBuffs
    #[serde(rename = "effects_health")]
    pub effects_health: Option<serde_json::Value>, // effects_health
    #[serde(rename = "effects_damage")]
    pub effects_damage: Option<serde_json::Value>, // effects_damage
    #[serde(rename = "effects_speed")]
    pub effects_speed: Option<serde_json::Value>, // effects_speed
    pub maximum_number_of_usage: Option<u64>, // MaximumNumberOfUsage
    #[serde(rename = "knifeHitDelay")]
    pub knife_hit_delay: Option<u64>, // knifeHitDelay
    #[serde(rename = "knifeHitSlashRate")]
    pub knife_hit_slash_rate: Option<u64>, // knifeHitSlashRate
    #[serde(rename = "knifeHitStabRate")]
    pub knife_hit_stab_rate: Option<u64>, // knifeHitStabRate
    #[serde(rename = "knifeHitRadius")]
    pub knife_hit_radius: Option<f64>, // knifeHitRadius
    #[serde(rename = "knifeHitSlashDam")]
    pub knife_hit_slash_dam: Option<u64>, // knifeHitSlashDam
    #[serde(rename = "knifeHitStabDam")]
    pub knife_hit_stab_dam: Option<u64>, // knifeHitStabDam
    #[serde(rename = "knifeDurab")]
    pub knife_durab: Option<u64>, // knifeDurab
    pub primary_distance: Option<f64>, // PrimaryDistance
    pub secondry_distance: Option<f64>, // SecondryDistance
    pub slash_penetration: Option<u64>, // SlashPenetration
    pub stab_penetration: Option<u64>, // StabPenetration
    pub primary_consumption: Option<u64>, // PrimaryConsumption
    pub secondry_consumption: Option<u64>, // SecondryConsumption
    pub deflection_consumption: Option<u64>, // DeflectionConsumption
    pub config_path_str: Option<String>, // ConfigPathStr
    pub max_markers_count: Option<u64>, // MaxMarkersCount
    #[serde(rename = "scaleMin")]
    pub scale_min: Option<f64>, // scaleMin
    #[serde(rename = "scaleMax")]
    pub scale_max: Option<f64>, // scaleMax
    #[serde(rename = "medUseTime")]
    pub med_use_time: Option<u64>, // medUseTime
    #[serde(rename = "medEffectType")]
    pub med_effect_type: Option<String>, // medEffectType
    pub max_hp_resource: Option<u64>, // MaxHpResource
    #[serde(rename = "hpResourceRate")]
    pub hp_resource_rate: Option<u64>, // hpResourceRate
    pub max_efficiency: Option<u64>, // MaxEfficiency
    pub addiction: Option<u64>, // Addiction
    pub overdose: Option<u64>, // Overdose
    pub overdose_recovery: Option<u64>, // OverdoseRecovery
    pub addiction_recovery: Option<u64>, // AddictionRecovery
    pub buffs: Option<serde_json::Value>, // Buffs
    #[serde(rename = "apResource")]
    pub ap_resource: Option<u64>, // apResource
    #[serde(rename = "krResource")]
    pub kr_resource: Option<u64>, // krResource
    pub stack_min_random: Option<u64>, // StackMinRandom
    pub stack_max_random: Option<u64>, // StackMaxRandom
    #[serde(rename = "ammoType")]
    pub ammo_type: Option<String>, // ammoType
    pub damage: Option<u64>, // Damage
    #[serde(rename = "ammoAccr")]
    pub ammo_accr: Option<i64>, // ammoAccr
    #[serde(rename = "ammoRec")]
    pub ammo_rec: Option<i64>, // ammoRec
    #[serde(rename = "ammoDist")]
    pub ammo_dist: Option<u64>, // ammoDist
    #[serde(rename = "buckshotBullets")]
    pub buckshot_bullets: Option<u64>, // buckshotBullets
    pub penetration_power: Option<u64>, // PenetrationPower
    pub penetration_power_diviation: Option<f64>, // PenetrationPowerDiviation
    #[serde(rename = "ammoHear")]
    pub ammo_hear: Option<i64>, // ammoHear
    #[serde(rename = "ammoSfx")]
    pub ammo_sfx: Option<String>, // ammoSfx
    pub misfire_chance: Option<f64>, // MisfireChance
    pub min_fragments_count: Option<u64>, // MinFragmentsCount
    pub max_fragments_count: Option<u64>, // MaxFragmentsCount
    #[serde(rename = "ammoShiftChance")]
    pub ammo_shift_chance: Option<u64>, // ammoShiftChance
    #[serde(rename = "casingName")]
    pub casing_name: Option<String>, // casingName
    #[serde(rename = "casingEjectPower")]
    pub casing_eject_power: Option<u64>, // casingEjectPower
    #[serde(rename = "casingMass")]
    pub casing_mass: Option<f64>, // casingMass
    #[serde(rename = "casingSounds")]
    pub casing_sounds: Option<String>, // casingSounds
    pub projectile_count: Option<u64>, // ProjectileCount
    pub initial_speed: Option<u64>, // InitialSpeed
    pub penetration_chance: Option<f64>, // PenetrationChance
    pub ricochet_chance: Option<f64>, // RicochetChance
    pub fragmentation_chance: Option<f64>, // FragmentationChance
    #[serde(rename = "BallisticCoeficient")]
    pub ballistic_coefficient: Option<f64>, // BallisticCoeficient
    pub deterioration: Option<u64>, // Deterioration
    pub speed_retardation: Option<f64>, // SpeedRetardation
    pub tracer: Option<bool>, // Tracer
    pub tracer_color: Option<String>, // TracerColor
    pub tracer_distance: Option<f64>, // TracerDistance
    pub armor_damage: Option<u64>, // ArmorDamage
    pub caliber: Option<String>, // Caliber
    pub stamina_burn_per_damage: Option<f64>, // StaminaBurnPerDamage
    pub show_bullet: Option<bool>, // ShowBullet
    pub has_grenader_component: Option<bool>, // HasGrenaderComponent
    pub fuze_arm_time_sec: Option<f64>, // FuzeArmTimeSec
    pub explosion_strength: Option<u64>, // ExplosionStrength
    pub min_explosion_distance: Option<f64>, // MinExplosionDistance
    pub max_explosion_distance: Option<f64>, // MaxExplosionDistance
    pub fragments_count: Option<u64>, // FragmentsCount
    pub fragment_type: Option<String>, // FragmentType
    pub show_hit_effect_on_explode: Option<bool>, // ShowHitEffectOnExplode
    pub explosion_type: Option<String>, // ExplosionType
    pub ammo_life_time_sec: Option<u64>, // AmmoLifeTimeSec
    pub stack_slots: Option<serde_json::Value>, // StackSlots
    #[serde(rename = "type")]
    pub item_item: Option<String>, // type
    #[serde(rename = "eqMin")]
    pub eq_min: Option<u64>, // eqMin
    #[serde(rename = "eqMax")]
    pub eq_max: Option<u64>, // eqMax
    #[serde(rename = "rate")]
    pub rate: Option<u64>, // rate
    pub throw_type: Option<String>, // ThrowType
    pub strength: Option<u64>, // Strength
    pub contusion_distance: Option<u64>, // ContusionDistance
    #[serde(rename = "throwDamMax")]
    pub throw_dam_max: Option<u64>, // throwDamMax
    pub expl_delay: Option<f64>, // ExplDelay
    pub blindness: Option<serde_json::Value>, // Blindness
    pub contusion: Option<serde_json::Value>, // Contusion
    pub emit_time: Option<u64>, // EmitTime
    pub can_be_hidden_during_throw: Option<bool>, // CanBeHiddenDuringThrow
    pub indestructibility: Option<f64>, // Indestructibility
    #[serde(rename = "headSegments")]
    pub head_segments: Option<serde_json::Value>, // headSegments
    pub face_shield_component: Option<bool>, // FaceShieldComponent
    pub face_shield_mask: Option<String>, // FaceShieldMask
    pub material_type: Option<String>, // MaterialType
    pub ricochet_params: Option<serde_json::Value>, // RicochetParams
    pub deaf_strength: Option<String>, // DeafStrength
    pub distortion: Option<f64>, // Distortion
    pub compressor_treshold: Option<i64>, // CompressorTreshold
    pub compressor_attack: Option<u64>, // CompressorAttack
    pub compressor_release: Option<u64>, // CompressorRelease
    pub compressor_gain: Option<u64>, // CompressorGain
    pub cutoff_freq: Option<u64>, // CutoffFreq
    pub resonance: Option<f64>, // Resonance
    pub compressor_volume: Option<i64>, // CompressorVolume
    pub ambient_volume: Option<i64>, // AmbientVolume
    pub dry_volume: Option<i64>, // DryVolume
}

#[derive(Debug, Deserialize)]
pub struct Prefab {
    path: String,
    rcid: String,
}

impl Tarkov {
    pub async fn get_items(&self) -> Result<()> {
        let url = format!("{}/client/items", PROD_ENDPOINT);
        let res: ItemsResponse = self
            .post_json(&url, &ItemsRequest { crc: 0 })
            .await?;

        println!("{:?}", res);

        Ok(())
    }
}
