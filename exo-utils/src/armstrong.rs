#![allow(improper_ctypes_definitions)] //Addresses `extern` fn uses type `str`, which is not FFI-safe
use super::*;

pub unsafe extern "C" fn is_armstrong_slots(boma: *mut BattleObjectModuleAccessor) -> bool {
    MARKED_COLORS[WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) as usize]
}

pub unsafe extern "C" fn armstrong_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_CLEAR_CHARGE);
    WorkModule::off_flag(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_SPECIAL_S_RUN);
    WorkModule::off_flag(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_SPECIAL_S_JUMP);
    WorkModule::off_flag(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_COUNTER_ACTIVE);
    WorkModule::off_flag(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_NANOMACHINES);
    WorkModule::off_flag(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_THROW);
    WorkModule::set_float(boma, 1.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
    WorkModule::set_float(boma, 0.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CURRENT_DAMAGE);
    WorkModule::set_float(boma, 1.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
    WorkModule::set_float(boma, 0.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_NEUTRAL_SPECIAL_CHARGE);
    WorkModule::set_float(boma, 0.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_INIT_Y_VEL);
    WorkModule::set_int(boma, 0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_FLAME_PILLAR_ID);
    WorkModule::set_int(boma, 0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_SPECIAL_S_RUN_TIME);
}

pub unsafe extern "C" fn armstrong_charge_move(fighter: &mut L2CFighterCommon, charge_start: f32, charge_end: f32, motion_rate_mul: f32, armor_value: f32, charging: bool, is_armored_move: bool, charge_effect_bone: &str) {
    let current_frame = MotionModule::frame(fighter.module_accessor);
    let boma = fighter.module_accessor;
    let armor_multiplier = WorkModule::get_float(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
    let damage_multiplier = WorkModule::get_float(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
    let charge_frames = WorkModule::get_int(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
    let max_charge: f32 = 20.0;
    let motion_rate= motion_rate_mul*(charge_frames as f32);
    if (charge_start..charge_end).contains(&current_frame) && charge_frames <= (max_charge as i32) && charging {
        MotionModule::set_rate(boma, motion_rate);
        WorkModule::set_float(boma, 1.0+((1.0/14.0)*(charge_frames as f32)), *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
        WorkModule::set_float(boma, 1.0+(0.02*(charge_frames as f32)), *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
        armstrong_activate_bonuses(boma, charge_frames, damage_multiplier, armor_multiplier, armor_value, is_armored_move);
        armstrong_charge_effect(fighter, charge_frames, charge_effect_bone);
        WorkModule::inc_int(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
    }
    else {
        armstrong_activate_bonuses(boma, charge_frames, damage_multiplier, armor_multiplier, armor_value, is_armored_move);
        MotionModule::set_rate(boma, 1.0);
    }
}

unsafe extern "C" fn armstrong_charge_effect(fighter: &mut L2CFighterCommon, charge_frames: i32, bone: &str) {
    if charge_frames == 0 {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new(bone), 0, 0, 0, 0, -45, 0, 1, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.88, 0.35, 0.13);
    }
    if [0, 3, 6, 9, 12, 15, 18].contains(&charge_frames) {
        fighter.clear_lua_stack();
        lua_args!(fighter, 0.88, 0.35, 0.13, 0.5);
        sv_animcmd::FLASH_NO_STOP(fighter.lua_state_agent);
    }
    if [2, 5, 8, 11, 14, 17].contains(&charge_frames) || charge_frames >= 20 {
        COL_NORMAL(fighter);
    }
}

pub unsafe extern "C" fn armstrong_activate_bonuses(boma: *mut BattleObjectModuleAccessor, charge_frames: i32, damage_multiplier: f32, armor_multiplier: f32, armor_value: f32, is_armored_move: bool) {
    if charge_frames > 0 {
        AttackModule::set_power_up(boma, damage_multiplier);
        if is_armored_move {
            DamageModule::set_reaction_mul(boma, 0.85/armor_multiplier);
            if WorkModule::is_flag(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_NANOMACHINES) {
                DamageModule::set_no_reaction_mode_status(boma, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_ALWAYS as u8}, -1.0, -1.0, -1);
                DamageModule::set_no_reaction_no_effect(boma, true);
            }
            else {
                DamageModule::set_no_reaction_mode_status(boma, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER as u8}, armor_value*armor_multiplier, -1.0, -1);
            }
        }
    }
}

pub unsafe extern "C" fn armstrong_clear_charge(boma: *mut BattleObjectModuleAccessor) {
    let fighter = get_fighter_common_from_accessor(&mut *boma);
    AttackModule::set_power_up(boma, 1.0);
    COL_NORMAL(fighter);
    DamageModule::set_reaction_mul(boma, 1.0);
    DamageModule::reset_no_reaction_mode_status(boma);
    damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
    WorkModule::set_int(boma, 0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
    WorkModule::set_float(boma, 0.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
    WorkModule::set_float(boma, 0.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CURRENT_DAMAGE);
    WorkModule::set_float(boma, 0.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
}