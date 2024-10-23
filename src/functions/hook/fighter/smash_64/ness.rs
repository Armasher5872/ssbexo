#![allow(dead_code)]
use super::*;

const NESS_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xdefdf0; //Ness only
const NESS_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const NESS_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xdf0050; //Ness only
const NESS_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xdf0360; //Ness only
const NESS_VTABLE_REFLECT_ATTACK_EVENT_OFFSET: usize = 0xdf0400; //Ness only

//Ness Startup Initialization
#[skyline::hook(offset = NESS_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ness_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let attack_air_f_data = ShieldDataResource::new(0.0, 6.8, 10.0, 0.0, 6.8, 10.0, 6.0, Hash40::new("top"), *COLLISION_SHAPE_TYPE_CAPSULE as u8, *SHIELD_TYPE_UNDEFINED as u8);
    let attack_air_f_2_data = ShieldDataResource::new(0.0, 4.8, 9.6, 0.0, 4.8, 9.6, 10.0, Hash40::new("top"), *COLLISION_SHAPE_TYPE_CAPSULE as u8, *SHIELD_TYPE_UNDEFINED as u8);
    let attack_air_f_datas = &mut (ShieldDatas::new().add(attack_air_f_data, 0));
    let attack_air_f_2_datas = &mut (ShieldDatas::new().add(attack_air_f_2_data, 0));
    let resource_attack_air_f = &mut ShieldGroupResource::new(attack_air_f_datas, 1, 0, false, false, false);
    let resource_attack_air_f_2 = &mut ShieldGroupResource::new(attack_air_f_2_datas, 1, 0, false, false, false);
    add_shield_group(boma, resource_attack_air_f, FIGHTER_NESS_SHIELD_GROUP_KIND_PSI_ATTACK_AIR_F_GUARD);
    add_shield_group(boma, resource_attack_air_f_2, FIGHTER_NESS_SHIELD_GROUP_KIND_PSI_ATTACK_AIR_F_2_GUARD);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALL_LAST_STOCK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
    COUNTERHIT_CHECK[entry_id] = false;
    COUNTERHIT_SUCCESS[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
    FIGHTER_BOOL_1[entry_id] = false;
    FIGHTER_BOOL_2[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
    WorkModule::set_flag(boma, sv_information::is_ready_go(), FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    SPECIAL_WALL_JUMP = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED);
    LAST_DAMAGE[entry_id] = 0.0;
    SIZE0[entry_id] = 0.0;
    SIZE1[entry_id] = 0.0;
    SIZE2[entry_id] = 0.0;
    SIZE3[entry_id] = 0.0;
    FULL_HOP_ENABLE_DELAY[entry_id] = 0;
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    WorkModule::set_flag(boma, false, FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_ATTACK_S4_HIT_REFLECT);
    WorkModule::set_flag(boma, false, FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP);
    WorkModule::set_int(boma, 0, FIGHTER_NESS_INSTANCE_WORK_ID_INT_OFFENSE_UP_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_NESS_INSTANCE_WORK_ID_INT_OFFENSE_UP_EFFECT_TIMER);
    original!()(vtable, fighter)
}

//Ness Reset Initialization
#[skyline::hook(offset = NESS_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ness_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter.battle_object.kind == *FIGHTER_KIND_NESS as u32 {
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALL_LAST_STOCK);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
        COUNTERHIT_CHECK[entry_id] = false;
        COUNTERHIT_SUCCESS[entry_id] = false;
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
        FIGHTER_BOOL_1[entry_id] = false;
        FIGHTER_BOOL_2[entry_id] = false;
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
        WorkModule::set_flag(boma, sv_information::is_ready_go(), FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        SPECIAL_WALL_JUMP = false;
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
        WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
        WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED);
        LAST_DAMAGE[entry_id] = 0.0;
        SIZE0[entry_id] = 0.0;
        SIZE1[entry_id] = 0.0;
        SIZE2[entry_id] = 0.0;
        SIZE3[entry_id] = 0.0;
        FULL_HOP_ENABLE_DELAY[entry_id] = 0;
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
        WorkModule::set_flag(boma, false, FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_ATTACK_S4_HIT_REFLECT);
        WorkModule::set_flag(boma, false, FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP);
        WorkModule::set_int(boma, 0, FIGHTER_NESS_INSTANCE_WORK_ID_INT_OFFENSE_UP_TIMER);
        WorkModule::set_int(boma, 0, FIGHTER_NESS_INSTANCE_WORK_ID_INT_OFFENSE_UP_EFFECT_TIMER);
    }
    original!()(vtable, fighter)
}

//Ness Death Initialization
#[skyline::hook(offset = NESS_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ness_death_initialization(param_1: u64, param_2: u64, param_3: u64, param_4: u64, vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
    WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
    COUNTERHIT_CHECK[entry_id] = false;
    COUNTERHIT_SUCCESS[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
    FIGHTER_BOOL_1[entry_id] = false;
    FIGHTER_BOOL_2[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    SPECIAL_WALL_JUMP = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED);
    LAST_DAMAGE[entry_id] = 0.0;
    SIZE0[entry_id] = 0.0;
    SIZE1[entry_id] = 0.0;
    SIZE2[entry_id] = 0.0;
    SIZE3[entry_id] = 0.0;
    FULL_HOP_ENABLE_DELAY[entry_id] = 0;
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    WorkModule::set_flag(boma, false, FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_ATTACK_S4_HIT_REFLECT);
    WorkModule::set_flag(boma, false, FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP);
    WorkModule::set_int(boma, 0, FIGHTER_NESS_INSTANCE_WORK_ID_INT_OFFENSE_UP_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_NESS_INSTANCE_WORK_ID_INT_OFFENSE_UP_EFFECT_TIMER);
    original!()(param_1, param_2, param_3, param_4, vtable, fighter)
}

//Ness Once Per Fighter Frame
#[skyline::hook(offset = NESS_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn ness_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let offense_up_timer = WorkModule::get_int(boma, FIGHTER_NESS_INSTANCE_WORK_ID_INT_OFFENSE_UP_TIMER);
    let offense_up_effect_timer = WorkModule::get_int(boma, FIGHTER_NESS_INSTANCE_WORK_ID_INT_OFFENSE_UP_EFFECT_TIMER);
    if WorkModule::is_flag(boma, FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        //Timer Mechanics
        if offense_up_timer > 0 {
            WorkModule::dec_int(boma, FIGHTER_NESS_INSTANCE_WORK_ID_INT_OFFENSE_UP_TIMER);
            if offense_up_timer <= 10 {
                EffectModule::kill_kind(boma, Hash40::new("ness_pkfl_hold"), false, false);
                EffectModule::kill_kind(boma, Hash40::new("sys_status_attack_up"), false, false);
            }
        }
        else {
            WorkModule::set_flag(boma, false, FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP);
            WorkModule::set_int(boma, 0, FIGHTER_NESS_INSTANCE_WORK_ID_INT_OFFENSE_UP_EFFECT_TIMER);
        }
        //Effect Mechanics
        WorkModule::inc_int(boma, FIGHTER_NESS_INSTANCE_WORK_ID_INT_OFFENSE_UP_EFFECT_TIMER);
        if offense_up_effect_timer > 25 {
            WorkModule::set_int(boma, 0, FIGHTER_NESS_INSTANCE_WORK_ID_INT_OFFENSE_UP_EFFECT_TIMER);
        }
        if offense_up_effect_timer == 10 {
            EffectModule::req_follow(boma, Hash40::new("ness_pkfl_hold"), Hash40::new("handl"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::req_follow(boma, Hash40::new("ness_pkfl_hold"), Hash40::new("handr"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::req_follow(boma, Hash40::new("sys_status_attack_up"), Hash40::new("hip"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        }
        if offense_up_effect_timer >= 20 {
            EffectModule::kill_kind(boma, Hash40::new("ness_pkfl_hold"), false, false);
            EffectModule::kill_kind(boma, Hash40::new("sys_status_attack_up"), false, false);
            EffectModule::req_follow(boma, Hash40::new("ness_pkfl_hold"), Hash40::new("handl"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::req_follow(boma, Hash40::new("ness_pkfl_hold"), Hash40::new("handr"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::req_follow(boma, Hash40::new("sys_status_attack_up"), Hash40::new("hip"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        }
        //Game Mechanics
        AttackModule::set_power_up(boma, 1.1);
        AttackModule::set_reaction_mul(boma, 0.87);
    }
    original!()(vtable, fighter)
}

//Ness Reflect Attack Event
#[skyline::hook(offset = NESS_VTABLE_REFLECT_ATTACK_EVENT_OFFSET)]
unsafe extern "C" fn ness_reflect_attack_event(vtable: u64, fighter: &mut Fighter, unk: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 {
        WorkModule::set_flag(boma, true, FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_ATTACK_S4_HIT_REFLECT);   
    }
    original!()(vtable, fighter, unk)
}

pub fn install() {
    skyline::install_hooks!(
        ness_start_initialization,
        ness_reset_initialization,
        //ness_death_initialization
        ness_opff,
        ness_reflect_attack_event
    );
}