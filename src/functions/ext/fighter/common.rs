#![allow(dead_code, improper_ctypes_definitions)]
use super::*;

//A variety of extern C functions mainly regarding custom game modes and other offsets in Main
extern "C" {
    #[link_name = "\u{1}_ZN3app8lua_bind28PostureModule__rot_y_lr_implEPNS_26BattleObjectModuleAccessorE"]
	pub fn imported_rot_y_lr(boma: &mut BattleObjectModuleAccessor) -> f32;
    
    #[link_name = "\u{1}_ZN3app9curryshot15is_preview_modeEv"]
	pub fn is_preview_mode() -> bool;
    
    #[link_name = "\u{1}_ZN3app9holywater35HOLYWATER_FIRE_PILLAR_GRAVITY_ACCELENS_11FighterKindE"]
    pub fn fire_pillar_gravity_accel(kind: FighterKind) -> f32;

    #[link_name = "\u{1}_ZN3app9holywater39HOLYWATER_FIRE_PILLAR_GRAVITY_ACCEL_MAXENS_11FighterKindE"]
    pub fn fire_pillar_gravity_accel_max(kind: FighterKind) -> f32;

    #[link_name = "\u{1}_ZN3app9holywater29HOLYWATER_FIRE_PILLAR_SPEED_YENS_11FighterKindE"]
    pub fn fire_pillar_speed_y(kind: FighterKind) -> f32;
    
    #[link_name = "\u{1}_ZN3app16kiiladarzmanager15set_visible_hudEb"]
    pub fn set_vis_hud(param_1: bool);

    #[link_name = "\u{1}_ZN3app17sv_camera_manager10dead_rangeEP9lua_State"]
	pub fn dead_range(lua_state: u64) -> Vector4f;

    #[link_name = "\u{1}_ZN3app17sv_camera_manager12camera_rangeEv"]
	pub fn camera_range() -> Vector4f;

    #[link_name = "\u{1}_ZN3app18kinetic_energy_rot12set_rotationEP9lua_StateRKN3phx8Vector3fE"]
    pub fn kinetic_energy_rot_set_rotation(lua_state: u64, rotation: *const Vector3f);

    #[link_name = "\u{1}_ZN3app19FighterCutInManager13is_one_on_oneEv"]
    pub fn FighterCutInManager__is_one_on_one() -> bool;
    
    #[link_name = "\u{1}_ZN3app22kinetic_energy_control6enableEP9lua_State"]
    pub fn kinetic_energy_control_enable(lua_state: u64);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_control9set_accelEP9lua_StateRKN3phx8Vector2fE"]
    pub fn kinetic_energy_control_set_accel(lua_state: u64, accel: *const Vector2f);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_control9set_brakeEP9lua_StateRKN3phx8Vector2fE"]
    pub fn kinetic_energy_control_set_brake(lua_state: u64, accel: *const Vector2f);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_control16set_stable_speedEP9lua_StateRKN3phx8Vector2fE"]
    pub fn kinetic_energy_control_set_stable_speed(lua_state: u64, accel: *const Vector2f);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_control15set_limit_speedEP9lua_StateRKN3phx8Vector2fE"]
    pub fn kinetic_energy_control_set_limit_speed(lua_state: u64, accel: *const Vector2f);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_control9set_speedEP9lua_StateRKN3phx8Vector2fE"]
    pub fn kinetic_energy_control_set_speed(lua_state: u64, accel: *const Vector2f);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_gravity9set_accelEP9lua_Statef"]
    pub fn kinetic_energy_gravity_set_accel(lua_state: u64, accel: f32);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_gravity15set_limit_speedEP9lua_Statef"]
    pub fn kinetic_energy_gravity_set_limit_speed(lua_state: u64, accel: f32);

    #[link_name = "\u{1}_ZN3app24FighterSpecializer_Luigi14delete_plungerERNS_7FighterEb"]
	pub fn delete_plunger(fighter: *mut smash::app::Fighter, param: bool) -> u64;

    #[link_name = "\u{1}_ZN3app24FighterSpecializer_Sonic17start_final_sonicERNS_7FighterE"]
	pub fn start_final_sonic(instance: *mut smash::app::Fighter) -> u64;

    #[link_name = "\u{1}_ZN3app24FighterSpecializer_Sonic24start_final_sonic_cameraERNS_7FighterE"]
	pub fn start_final_sonic_camera(instance: *mut smash::app::Fighter) -> u64;

    #[link_name = "\u{1}_ZN3app25FighterSpecializer_Dedede29end_special_n_shot_object_hitERNS_7FighterE"]
	pub fn end_special_n_shot_object_hit(fighter: *mut smash::app::Fighter) -> u64;

    #[link_name = "\u{1}_ZN3app26kinetic_energy_control_rot12set_rotationEP9lua_StateRKN3phx8Vector3fE"]
    pub fn kinetic_energy_control_rot_set_rotation(lua_state: u64, rotation: *const Vector3f);

    #[link_name = "\u{1}_ZN3app8lua_bind38FighterManager__get_fighter_entry_implEPNS_14FighterManagerENS_14FighterEntryIDE"]
    pub fn get_fighter_entry(manager: *mut smash::app::FighterManager, entry_id: u32) -> *mut u8;

    pub fn change_version_string(arg: u64, string: *const c_char);
}

pub unsafe extern "C" fn should_remove_projectile(weapon: &mut L2CWeaponCommon) -> bool {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if GroundModule::is_wall_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32)
    || WorkModule::is_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HIT_WALL)
    || life <= 0 {
        return true;
    }
    return false;
}

pub unsafe extern "C" fn should_use_special_n_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE) {
        return 0.into();
    }
    1.into()
}

pub unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE) {
        return 0.into();
    }
    1.into()
}

pub unsafe extern "C" fn should_use_special_hi_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE) {
        return 0.into();
    }
    1.into()
}

pub unsafe extern "C" fn should_use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE) {
        return 0.into();
    }
    1.into()
}

//Related to CheckAttack
pub unsafe fn get_table_value(table: *mut smash2::lib::L2CTable, key: &str) -> smash2::lib::L2CValue {
    let hash = if key.starts_with("0x") {
        smash2::phx::Hash40::from_hex_str(key).unwrap()
    } 
    else {
        smash2::phx::hash40(key)
    };
    (*table).get_map(hash).unwrap().clone()
}

pub unsafe extern "C" fn empty_waza_customize() -> L2CValue {
    0.into()
}

//Gets the original Waza Customize
pub unsafe fn get_original_customizer(fighter: &mut L2CFighterCommon) -> Option<unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue> {
    let ptr = fighter.global_table["move_customizer_original"].get_ptr();
    if !ptr.is_null() {
        Some(std::mem::transmute(ptr))
    } 
    else {
        None
    }
}

//Related to Waza Customize
pub unsafe fn set_move_customizer(fighter: &mut L2CFighterCommon, customizer: unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue) {
    if fighter.global_table["move_customizer_set"].get_bool() {
        return;
    }
    let waza_customize_control = fighter.global_table[WAZA_CUSTOMIZE_CONTROL].clone();
    fighter.global_table["move_customizer_set"].assign(&L2CValue::Bool(true));
    fighter.global_table["move_customizer_original"].assign(&waza_customize_control);
    fighter.global_table[WAZA_CUSTOMIZE_CONTROL].assign(&L2CValue::Ptr(customizer as *const () as _));
}

//Gets the necessary grab animation for throws
pub unsafe extern "C" fn grabbed_anim_selector(fighter: &mut L2CFighterCommon, anim_name: &str, mot_rate: f32) {
    let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
    let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
    if capture_id != 0x50000000 {
        let motion_share = WorkModule::get_param_int(capture_boma,0xcad2ee25e,0xc07d88ea0);
        let mut motion = hash40(anim_name);
        if motion_share == *FIGHTER_MOTION_SHARE_TYPE_TARO {
            motion = FighterMotionModuleImpl::add_body_type_hash(capture_boma, Hash40::new_raw(motion), *BODY_TYPE_MOTION_DX);
        }
        else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
            motion = FighterMotionModuleImpl::add_body_type_hash(capture_boma, Hash40::new_raw(motion), *BODY_TYPE_MOTION_GIRL);
        }
        else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_BIG {
            motion = FighterMotionModuleImpl::add_body_type_hash(capture_boma, Hash40::new_raw(motion), *BODY_TYPE_MOTION_BIG);
        }
        MotionModule::change_motion(capture_boma, Hash40::new_raw(motion), 0.0, mot_rate, false, 0.0, false, false);
    }
}

//Condenses the initial reseting of variables into one function
pub unsafe extern "C" fn common_initialization_variable_reset(boma: &mut BattleObjectModuleAccessor) {
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let flags = [
        FIGHTER_INSTANCE_WORK_ID_FLAG_ALL_LAST_STOCK, FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE,
        FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED,
        FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_AIR_FLIP, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT, 
        FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK, FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH, 
        FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE, 
        FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH
    ];
    let floats = [FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED];
    let ints = [
        FIGHTER_INSTANCE_WORK_ID_INT_ATTACK_ANGLE, FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER, FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT, FIGHTER_INSTANCE_WORK_ID_INT_MASHING,
        FIGHTER_INSTANCE_WORK_ID_INT_PARRIED, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_EFFECT_ID,
        FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX
    ];
    for x in 0..flags.len() {
        WorkModule::off_flag(boma, flags[x]);
    };
    for y in 0..floats.len() {
        WorkModule::set_float(boma, 0.0, floats[y]);
    }
    for z in 0..ints.len() {
        WorkModule::set_int(boma, 0, ints[z]);
    }
    WorkModule::set_flag(boma, sv_information::is_ready_go(), FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO);
    BALL_ID = 0;
    BALL_OWNER = 9;
    BALL_VICTIMS[entry_id] = 4;
    COUNTERHIT_CHECK[entry_id] = false;
    COUNTERHIT_SUCCESS[entry_id] = false;
    FULL_HOP_ENABLE_DELAY[entry_id] = 0;
    LAST_ALT_STICK[0] = 0.0;
    LAST_ALT_STICK[1] = 0.0;
    LAST_ANALOG = 0.0;
    LAST_ATTACK_HITBOX_ID = 0;
    LAST_ATTACK_HITBOX_LOCATION_X = 0.0;
    LAST_ATTACK_HITBOX_LOCATION_Y = 0.0;
    LAST_ATTACK_HITBOX_LOCATION_Z = 0.0;
    LAST_DAMAGE[entry_id] = 0.0;
    LAST_TO_HIT_BALL = 9;
    READY_GO_TIMER = 0;
    SIZE0[entry_id] = 0.0;
    SIZE1[entry_id] = 0.0;
    SPAWN_SIDE[entry_id] = false;
    SPECIAL_SMASH_BODY = 0;
    SPECIAL_SMASH_GRAVITY = 0;
    SPECIAL_SMASH_HEAD = 0;
    SPECIAL_SMASH_RATE = 0;
    SPECIAL_SMASH_SIZE = 0;
    SPECIAL_SMASH_STATUS = 0;
    SPECIAL_WALL_JUMP = false;
    STOCK_COUNT[entry_id] = 99;
    TOTAL_FIGHTER = 1;
}

//Condenses the reset event reseting of variables into one function
pub unsafe extern "C" fn common_reset_variable_reset(boma: &mut BattleObjectModuleAccessor) {
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let flags = [
        FIGHTER_INSTANCE_WORK_ID_FLAG_ALL_LAST_STOCK, FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE,
        FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED,
        FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_AIR_FLIP, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT, 
        FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK, FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH,
        FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE,
        FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH
    ];
    let floats = [FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED];
    let ints = [
        FIGHTER_INSTANCE_WORK_ID_INT_ATTACK_ANGLE, FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER, FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT, FIGHTER_INSTANCE_WORK_ID_INT_MASHING,
        FIGHTER_INSTANCE_WORK_ID_INT_PARRIED, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_EFFECT_ID,
        FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX
    ];
    for x in 0..flags.len() {
        WorkModule::off_flag(boma, flags[x]);
    };
    for y in 0..floats.len() {
        WorkModule::set_float(boma, 0.0, floats[y]);
    }
    for z in 0..ints.len() {
        WorkModule::set_int(boma, 0, ints[z]);
    }
    WorkModule::set_flag(boma, sv_information::is_ready_go(), FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO);
    BALL_ID = 0;
    BALL_OWNER = 9;
    BALL_VICTIMS[entry_id] = 4;
    COUNTERHIT_CHECK[entry_id] = false;
    COUNTERHIT_SUCCESS[entry_id] = false;
    FULL_HOP_ENABLE_DELAY[entry_id] = 0;
    LAST_ALT_STICK[0] = 0.0;
    LAST_ALT_STICK[1] = 0.0;
    LAST_ANALOG = 0.0;
    LAST_ATTACK_HITBOX_ID = 0;
    LAST_ATTACK_HITBOX_LOCATION_X = 0.0;
    LAST_ATTACK_HITBOX_LOCATION_Y = 0.0;
    LAST_ATTACK_HITBOX_LOCATION_Z = 0.0;
    LAST_DAMAGE[entry_id] = 0.0;
    LAST_TO_HIT_BALL = 9;
    READY_GO_TIMER = 0;
    SIZE0[entry_id] = 0.0;
    SIZE1[entry_id] = 0.0;
    SPAWN_SIDE[entry_id] = false;
    SPECIAL_SMASH_BODY = 0;
    SPECIAL_SMASH_GRAVITY = 0;
    SPECIAL_SMASH_HEAD = 0;
    SPECIAL_SMASH_RATE = 0;
    SPECIAL_SMASH_SIZE = 0;
    SPECIAL_SMASH_STATUS = 0;
    SPECIAL_WALL_JUMP = false;
    STOCK_COUNT[entry_id] = 99;
    TOTAL_FIGHTER = 1;
}

//Condenses the reseting of variables on death into one function
pub unsafe extern "C" fn common_death_variable_reset(boma: &mut BattleObjectModuleAccessor) {
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let flags = [
        FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL, 
        FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED, FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE,
        FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_AIR_FLIP, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT, FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK,
        FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC, 
        FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS, 
        FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH
    ];
    let floats = [FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED];
    let ints = [
        FIGHTER_INSTANCE_WORK_ID_INT_ATTACK_ANGLE, FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER, FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT, FIGHTER_INSTANCE_WORK_ID_INT_MASHING,
        FIGHTER_INSTANCE_WORK_ID_INT_PARRIED, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_EFFECT_ID,
        FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX
    ];
    for x in 0..flags.len() {
        WorkModule::off_flag(boma, flags[x]);
    };
    for y in 0..floats.len() {
        WorkModule::set_float(boma, 0.0, floats[y]);
    }
    for z in 0..ints.len() {
        WorkModule::set_int(boma, 0, ints[z]);
    }
    COUNTERHIT_CHECK[entry_id] = false;
    COUNTERHIT_SUCCESS[entry_id] = false;
    FULL_HOP_ENABLE_DELAY[entry_id] = 0;
    LAST_ALT_STICK[0] = 0.0;
    LAST_ALT_STICK[1] = 0.0;
    LAST_ANALOG = 0.0;
    LAST_ATTACK_HITBOX_ID = 0;
    LAST_ATTACK_HITBOX_LOCATION_X = 0.0;
    LAST_ATTACK_HITBOX_LOCATION_Y = 0.0;
    LAST_ATTACK_HITBOX_LOCATION_Z = 0.0;
    LAST_DAMAGE[entry_id] = 0.0;
    READY_GO_TIMER = 0;
    SIZE0[entry_id] = 0.0;
    SIZE1[entry_id] = 0.0;
    SPAWN_SIDE[entry_id] = false;
    SPECIAL_WALL_JUMP = false;
}

//Credited to WuBoyTH, enables characters having command inputs buttons to work properly
pub unsafe extern "C" fn set_command_input_button(boma: *mut BattleObjectModuleAccessor, command: usize, buttons: u8) {
    let control_module = *(boma as *const *const u64).add(0x48/8);
    let command_input = *control_module.add((0x7f0+(command*8))/8) as *mut u8;
    *command_input.add(0xb) = buttons;
}