use super::*;

unsafe extern "C" fn sonic_special_lw_hold_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn sonic_special_lw_hold_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn sonic_special_lw_hold_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
    if situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_hold"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_SONIC_SPECIAL_AIR_LW);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_hold"), 0.0, 1.0, false, 0.0, false, false);
    }
    MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new_raw(0x1c2f32b79c), -1);
    fighter.sub_shift_status_main(L2CValue::Ptr(sonic_special_lw_hold_main_loop as *const () as _))
}

unsafe extern "C" fn sonic_special_lw_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let rot_speed = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_LW_HOLD_WORK_FLOAT_ROT_SPEED);
    let charge_level = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_LW_HOLD_WORK_INT_CHARGE_LEVEL);
    let squat_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("squat_stick_y"));
    let special_lw_end_rot_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_end_rot_speed"));
    let special_lw_start_rot_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_start_rot_speed"));
    let current_charge = (rot_speed-special_lw_start_rot_speed)/(special_lw_end_rot_speed-special_lw_start_rot_speed);
    let mut can_go_into_dash = false;
    let mut dash_check = false;
    if rot_speed >= 60.0 {
        dash_check = true;
    }
    if squat_stick_y < stick_y {
        can_go_into_dash = true;
    }
    if dash_check {
        can_go_into_dash = true;
    }
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        WorkModule::add_float(fighter.module_accessor, 1.0, *FIGHTER_SONIC_STATUS_SPECIAL_LW_HOLD_WORK_FLOAT_ROT_SPEED);
    }
    if charge_level != 0 {
        if charge_level != 1 {
            if current_charge <= 0.66 {
                fun_71000156a0(fighter);
                MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new_raw(0x204be4482f), -1);
                WorkModule::set_int(fighter.module_accessor, charge_level-1, *FIGHTER_SONIC_STATUS_SPECIAL_LW_HOLD_WORK_INT_CHARGE_LEVEL);
            }
        }
        else {
            if 0.66 >= current_charge {
                if current_charge <= 0.33 {
                    fun_71000156a0(fighter);
                    MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new_raw(0x1c2f32b79c), -1);
                    WorkModule::set_int(fighter.module_accessor, charge_level+1, *FIGHTER_SONIC_STATUS_SPECIAL_LW_HOLD_WORK_INT_CHARGE_LEVEL);
                }
            }
            else {
                fun_71000156a0(fighter);
                MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new_raw(0x1cb1514ff), -1);
                WorkModule::set_int(fighter.module_accessor, charge_level+1, *FIGHTER_SONIC_STATUS_SPECIAL_LW_HOLD_WORK_INT_CHARGE_LEVEL);
            }
        }
    }
    else {
        if 0.33 < current_charge {
            fun_71000156a0(fighter);
            MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new_raw(0x204be4482f), -1);
            WorkModule::set_int(fighter.module_accessor, charge_level+1, *FIGHTER_SONIC_STATUS_SPECIAL_LW_HOLD_WORK_INT_CHARGE_LEVEL);
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if prev_situation_kind == *SITUATION_KIND_GROUND
        && situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_SONIC_SPECIAL_AIR_LW);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_hold"), -1.0, 1.0, 0.0, false, false);
        }
        if prev_situation_kind == *SITUATION_KIND_AIR
        && situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH.into(), true.into());
        }
    }
    if can_go_into_dash {
        fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH.into(), true.into());
        return 1.into();
    }
    if fun_7100015020(fighter).get_bool() {
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn fun_71000156a0(fighter: &mut L2CFighterCommon) {
    fighter.clear_lua_stack();
    lua_args!(fighter, *MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("sonic_idling_flash1"), true, true);
    sv_module_access::effect(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);
    fighter.clear_lua_stack();
    lua_args!(fighter, *MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("sonic_idling_flash2"), true, true);
    sv_module_access::effect(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);
    fighter.clear_lua_stack();
    lua_args!(fighter, *MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("sonic_idling_flash3"), true, true);
    sv_module_access::effect(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);
    fighter.clear_lua_stack();
    lua_args!(fighter, *MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("sonic_spinblur"), true, true);
    sv_module_access::effect(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);
    fighter.clear_lua_stack();
    lua_args!(fighter, *MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("sonic_spinblur_middle"), true, true);
    sv_module_access::effect(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);
    fighter.clear_lua_stack();
    lua_args!(fighter, *MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("sonic_spinblur_max"), true, true);
    sv_module_access::effect(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);
}

unsafe extern "C" fn sonic_special_lw_hold_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn sonic_special_lw_hold_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND {
        if status_kind != *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
    }
    0.into()
}

unsafe extern "C" fn sonic_special_lw_hold_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("sonic")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_HOLD, sonic_special_lw_hold_pre_status)
    .status(Init, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_HOLD, sonic_special_lw_hold_init_status)
    .status(Main, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_HOLD, sonic_special_lw_hold_main_status)
    .status(Exec, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_HOLD, sonic_special_lw_hold_exec_status)
    .status(End, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_HOLD, sonic_special_lw_hold_end_status)
    .status(Exit, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_HOLD, sonic_special_lw_hold_exit_status)
    .install()
    ;
}