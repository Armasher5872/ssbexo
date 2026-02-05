use super::*;

unsafe extern "C" fn sonic_special_s_dash_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_S_DASH_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_S_DASH_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_S_DASH_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn sonic_special_s_dash_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
    if prev_status_kind != *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_TURN {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_FLAG_SPECIAL_LW_HOLD) {
            let special_lw_charge_level = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_WORK_INT_SPECIAL_LW_CHARGE_LEVEL);
            if special_lw_charge_level != 0 {
                if special_lw_charge_level != 1 {
                    MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_speciallwdashchargehi"), -1);
                }
                else {
                    MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_speciallwdashchargemiddle"), -1);
                }
            }
            else {
                MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_speciallwdashchargelw"), -1);
            }
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_SONIC_SPECIAL_S_DASH);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_SONIC_SPECIAL_S_AIR_DASH);
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(0x1c90a5cf37), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(fighter.module_accessor) {
        fun_71000197e0(fighter);
    }
    fighter.global_table[PREV_SUB_STATUS].assign(&L2CValue::Ptr(fun_71000197e0 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(sonic_special_s_dash_main_loop as *const () as _))
}

unsafe extern "C" fn fun_71000197e0(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::dec_int(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_WORK_INT_ADVANCE_COUNTER);
    let special_s_dash_advance_counter = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_WORK_INT_ADVANCE_COUNTER);
    if special_s_dash_advance_counter < 0 {
        fighter.global_table[PREV_SUB_STATUS].assign(&L2CValue::I32(0));
        fighter.global_table[SUB_STATUS].assign(&L2CValue::I32(0));
    }
    0.into()
}

unsafe extern "C" fn sonic_special_s_dash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let get_sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = PostureModule::lr(fighter.module_accessor);
    let mtrans_type = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_WORK_INT_WAIT_MTRANS_TYPE);
    let special_s_dash_advance_counter = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_WORK_INT_ADVANCE_COUNTER);
    let special_s_dash_stop_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_dash_stop_speed"));
    let special_s_turn_fail_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_turn_fail_speed"));
    let turn_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_stick_x"));
    let x_stick = stick_x*lr;
    let mut can_change = false;
    let slow_check  = if !StopModule::is_stop(fighter.module_accessor) {SlowModule::is_skip(fighter.module_accessor)} else {true};
    let turn_check  = if x_stick > turn_stick_x {cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0} else {true};
    if !slow_check {
        if fun_710001aff0(fighter).get_bool() {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_FLAG_INFLICT_OCCUR) {
                fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPIN_JUMP.into(), true.into());
                return 1.into();
            }
        }
    }
    if mtrans_type != *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_WAIT_MTRANS_TYPE_SITUATION_GROUND {
        if mtrans_type == *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_WAIT_MTRANS_TYPE_SITUATION_AIR {
            if situation_kind == *SITUATION_KIND_AIR {
                if prev_situation_kind == *SITUATION_KIND_GROUND {
                    can_change = true;
                }
            }
        }
    }
    else {
        if situation_kind == *SITUATION_KIND_GROUND {
            if prev_situation_kind != *SITUATION_KIND_GROUND {
                can_change = true;
            }
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) {
            if 0.0 < lr {
                fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_REBOUND.into(), false.into());
                return 1.into();
            }
        }
        if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) {
            if lr < 0.0 {
                fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_REBOUND.into(), false.into());
                return 1.into();
            }
        }
        if get_sum_speed_x.abs() <= special_s_dash_stop_speed {
            fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END.into(), true.into());
            return 1.into();
        }
        if turn_check {
            if get_sum_speed_x.abs() > special_s_turn_fail_speed {
                fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_TURN.into(), true.into());
                return 1.into();
            }
            else {
                fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END.into(), true.into());
                return 1.into();
            }
        }
    }
    if can_change {
        if situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_SONIC_SPECIAL_S_DASH);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_SONIC_SPECIAL_S_AIR_DASH);
        }
    }
    if special_s_dash_advance_counter < 0 {
        fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        return 1.into();
    }
    if fun_7100015020(fighter).get_bool() {
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn fun_710001aff0(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND {
        if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON != 0 {
            return true.into();
        }
        if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0 {
            if ControlModule::is_enable_flick_jump(fighter.module_accessor) {
                return true.into();
            }
        }
    }
    false.into()
}

pub fn install() {
    Agent::new("sonic")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH, sonic_special_s_dash_pre_status)
    .status(Main, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH, sonic_special_s_dash_main_status)
    .install()
    ;
}