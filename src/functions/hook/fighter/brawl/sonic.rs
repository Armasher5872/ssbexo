use super::*;

const SONIC_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x11d56f0; //Sonic only
const SONIC_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const SONIC_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x11d5820; //Sonic only
const SONIC_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x11d7b20; //Sonic only

unsafe extern "C" fn sonic_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    }
    0.into()
}

//Sonic Startup Initialization
#[skyline::hook(offset = SONIC_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn sonic_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let lua_module_fighter = get_fighter_common_from_accessor(&mut *boma);
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
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_ATTACK_ANGLE);
    FULL_HOP_ENABLE_DELAY[entry_id] = 0;
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    WorkModule::set_flag(boma, false, FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_BOOST_GRAVITY);
    WorkModule::set_int(boma, 0, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_GAUGE);
    WorkModule::set_int(boma, 0, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_GAUGE_DECREASE);
    WorkModule::set_int(boma, 0, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_EFFECT_COUNTER);
    WorkModule::set_int(boma, 0, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_FINAL_SMASH_TIMER);
    WorkModule::set_float(boma, 0.0, FIGHTER_SONIC_INSTANCE_WORK_ID_FLOAT_BOOST_SPEED);
    lua_module_fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    lua_module_fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(sonic_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Sonic Reset Initialization
#[skyline::hook(offset = SONIC_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn sonic_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter.battle_object.kind == *FIGHTER_KIND_SONIC as u32 {
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
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_ATTACK_ANGLE);
        FULL_HOP_ENABLE_DELAY[entry_id] = 0;
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
        WorkModule::set_flag(boma, false, FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_BOOST_GRAVITY);
        WorkModule::set_int(boma, 0, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_GAUGE);
        WorkModule::set_int(boma, 0, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_GAUGE_DECREASE);
        WorkModule::set_int(boma, 0, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_EFFECT_COUNTER);
        WorkModule::set_int(boma, 0, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_FINAL_SMASH_TIMER);
        WorkModule::set_float(boma, 0.0, FIGHTER_SONIC_INSTANCE_WORK_ID_FLOAT_BOOST_SPEED);
    }
    original!()(vtable, fighter)
}

//Sonic Death Initialization
#[skyline::hook(offset = SONIC_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn sonic_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
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
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_ATTACK_ANGLE);
    FULL_HOP_ENABLE_DELAY[entry_id] = 0;
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    WorkModule::set_flag(boma, false, FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_BOOST_GRAVITY);
    WorkModule::set_int(boma, 0, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_GAUGE);
    WorkModule::set_int(boma, 0, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_GAUGE_DECREASE);
    WorkModule::set_int(boma, 0, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_EFFECT_COUNTER);
    WorkModule::set_int(boma, 0, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_FINAL_SMASH_TIMER);
    WorkModule::set_float(boma, 0.0, FIGHTER_SONIC_INSTANCE_WORK_ID_FLOAT_BOOST_SPEED);
    original!()(vtable, fighter)
}

//Sonic Once Per Fighter Frame
#[skyline::hook(offset = SONIC_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn sonic_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let motion_kind = MotionModule::motion_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    let frame = MotionModule::frame(boma);
    let rand_num_10 = sv_math::rand(hash40("fighter"), 10);
    let boost_gauge = WorkModule::get_int(boma, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_GAUGE);
    let boost_effect_counter = WorkModule::get_int(boma, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_EFFECT_COUNTER);
    let lua_module_fighter = get_fighter_common_from_accessor(&mut *boma);
    if ![*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_OFF, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_REBOUND, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD_JUMP, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HOMING_START, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HOMING, *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
        EffectModule::kill_kind(boma, Hash40::new("sonic_spintrace_homing"), false, true);
        EffectModule::kill_kind(boma, Hash40::new("sonic_spintrace_middle"), false, true);
        EffectModule::kill_kind(boma, Hash40::new("sys_shield"), false, true);
    };
    if ![*FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_REBOUND, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD_JUMP, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
        WorkModule::set_float(boma, KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)*PostureModule::lr(boma), FIGHTER_SONIC_INSTANCE_WORK_ID_FLOAT_BOOST_SPEED);
    };
    if frame < 2.0 && ![*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_CATCH_ATTACK, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) { 
        WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
    };
    if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && ![*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_CATCH_ATTACK, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
        WorkModule::inc_int(boma, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_GAUGE);
    };
    //Wait
    if status_kind == *FIGHTER_STATUS_KIND_WAIT {
        if motion_kind == hash40("wait_2") && frame == 40.0  && rand_num_10 <= 2 {
            MotionModule::change_motion(boma, Hash40::new("wait_2_a"), 1.0, 1.0, false, 0.0, false, false);
        };
        if motion_kind == hash40("wait_2_a") && frame >= 157.0 {
            MotionModule::change_motion(boma, Hash40::new("wait_2_b"), 1.0, 1.0, false, 0.0, false, false);
        };
        if motion_kind == hash40("wait_2_b") && frame >= 38.0 {
            MotionModule::change_motion(boma, Hash40::new("wait_2_b"), 1.0, 1.0, false, 0.0, false, false);
        };
    };
    //Sonic Taunt Holding
    if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
        if [hash40("appeal_s_r"), hash40("appeal_s_l")].contains(&motion_kind) && frame >= 45.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)  || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                MotionModule::set_frame_sync_anim_cmd(boma, 29.0, true, true, false);
            }
        }
        if [hash40("appeal_lw_r"), hash40("appeal_lw_l")].contains(&motion_kind) && frame >= 53.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                MotionModule::set_frame_sync_anim_cmd(boma, 13.0, true, true, false);
            }
        }
    }
    //Jab Cancel
    if fighter.battle_object.magic_series() == 1 {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
    }
    if fighter.battle_object.magic_series() == 2 {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
    }
    if fighter.battle_object.magic_series() == 3 {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
    }
    //Dash Attack Speed
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_SONIC_DASH);
        if (1.0..5.0).contains(&frame)
        && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            macros::SET_SPEED_EX(lua_module_fighter, 4.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else if (1.0..5.0).contains(&frame)
        && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD){
            macros::SET_SPEED_EX(lua_module_fighter, 0.15, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        if (6.0..11.0).contains(&frame)
        && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            macros::SET_SPEED_EX(lua_module_fighter, 3.25, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else if (6.0..11.0).contains(&frame)
        && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            macros::SET_SPEED_EX(lua_module_fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        if (11.0..=20.0).contains(&frame)
        && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD){
            macros::SET_SPEED_EX(lua_module_fighter, 1.05, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else if (11.0..=20.0).contains(&frame)
        && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            macros::SET_SPEED_EX(lua_module_fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        if frame >= 21.0
        && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            macros::SET_SPEED_EX(lua_module_fighter, 0.45, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else if frame >= 21.0
        && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            macros::SET_SPEED_EX(lua_module_fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    }
    //Boost Effect
    WorkModule::inc_int(boma, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_EFFECT_COUNTER);
    if boost_effect_counter > 25 {
        WorkModule::set_int(boma, 0, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_EFFECT_COUNTER);
    }
    if boost_gauge >= 15 {
        if boost_effect_counter == 10 {
            let waist_aura = EffectModule::req_follow(boma, Hash40::new("sys_aura_light"), Hash40::new("waist"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
            let bust_aura = EffectModule::req_follow(boma, Hash40::new("sys_aura_light"), Hash40::new("bust"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
            EffectModule::set_rgb(boma, waist_aura as u32, 0.0, 0.3, 1.0);
            EffectModule::set_rgb(boma, bust_aura as u32, 0.0, 0.3, 1.0);
        }
        if boost_effect_counter >= 20 {
            EffectModule::kill_kind(boma, Hash40::new("sys_aura_light"), false, false);
            let waist_aura = EffectModule::req_follow(boma, Hash40::new("sys_aura_light"), Hash40::new("waist"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
            let bust_aura = EffectModule::req_follow(boma, Hash40::new("sys_aura_light"), Hash40::new("bust"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
            EffectModule::set_rgb(boma, waist_aura as u32, 0.0, 0.3, 1.0);
            EffectModule::set_rgb(boma, bust_aura as u32, 0.0, 0.3, 1.0);
        }
    }
    else {
        EffectModule::kill_kind(boma, Hash40::new("sys_aura_light"), false, false);
    };
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        sonic_start_initialization,
        sonic_reset_initialization,
        sonic_death_initialization,
        sonic_opff
    );
}