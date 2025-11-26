use super::*;

unsafe extern "C" fn littlemac_attack_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_AttackCommon();
    if !StopModule::is_stop(fighter.module_accessor) {
        littlemac_check_attack_mtrans(fighter);
    }
    fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(littlemac_check_attack_mtrans as *const () as _));
    fighter.sub_status_AttackComboCommon();
    fighter.sub_shift_status_main(L2CValue::Ptr(littlemac_attack_main_loop as *const () as _))
}

unsafe extern "C" fn littlemac_check_attack_mtrans(fighter: &mut L2CFighterCommon) -> L2CValue {
    let combo_count = ComboModule::count(fighter.module_accessor);
    let attack_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_max"), 0);
    if !StatusModule::is_changing(fighter.module_accessor) {
        if combo_count as i32 >= attack_combo_max {
            return 0.into();
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO) {
            return 0.into();
        }
    }
    fighter.attack_mtrans_pre_process();
    littlemac_attack_mtrans_post_process(fighter);
    0.into()
}

unsafe extern "C" fn littlemac_attack_mtrans_post_process(fighter: &mut L2CFighterCommon) {
    let status_kind_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    //let stick_x = fighter.global_table[STICK_X].get_f32();
    //let stick_y = fighter.global_table[STICK_Y].get_f32();
    //let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let count = ComboModule::count(fighter.module_accessor);
    //let lr = PostureModule::lr(fighter.module_accessor);
    let mini_jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    let reserve_log_attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    let attack_jump_mini_attack_enable_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("attack_jump_mini_attack_enable_frame"));
    let status_attack = fighter.status_attack();
    let log_infos = status_attack["log_infos"].clone();
    let attack_11 = log_infos["attack_11"].get_i64();
    let attack_12 = log_infos["attack_12"].get_i64();
    let attack_13 = log_infos["attack_13"].get_i64();
    let mut cont = false;
    if !StatusModule::is_changing(fighter.module_accessor) {
        if prev_status_kind != status_kind_interrupt {
            if prev_status_kind != *FIGHTER_STATUS_KIND_ESCAPE {
                cont = true;
            }
        }
        else {
            if FighterMotionModuleImpl::is_valid_cancel_frame(fighter.module_accessor, -1, true) {
                cont = true;
            }
        }
    }
    if count != 1 {
        if count != 2 {
            if count != 3 {
                if StatusModule::is_changing(fighter.module_accessor) {
                    if cont {
                        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
                            WorkModule::set_int(fighter.module_accessor, attack_jump_mini_attack_enable_frame+1, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
                            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
                        }
                    }
                }
                else {
                    if 0 < mini_jump_attack_frame {
                        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
                        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
                        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
                    }
                }
                if mini_jump_attack_frame != 0 {
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
                        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART_ATTACK);
                        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO_TRIGGER);
                    }
                }
                if 0 < reserve_log_attack_kind {
                    FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, reserve_log_attack_kind);
                    WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
                }
            }
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_CAN_INPUT_DREAMLAND_EXPRESS) {
                WorkModule::inc_int(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SUCCESSFUL_DREAMLAND_EXPRESS_INPUTS);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_CAN_INPUT_DREAMLAND_EXPRESS);
            }
            /*
            if stick_x*lr > 0.7 || (cmd_cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S3.into(), true.into());
            }
            else if stick_y > 0.7 || (cmd_cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI3.into(), true.into());
            }
            else if stick_y < -0.7 || (cmd_cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW3.into(), true.into());
            }
            else {*/
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_13"), 0.0, 1.0, false, 0.0, false, false);
                fighter.clear_lua_stack();
                sv_kinetic_energy::set_motion_energy_update_flag(fighter.lua_state_agent);
                WorkModule::set_int64(fighter.module_accessor, attack_13, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            /*}*/
        }
        else {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_CAN_INPUT_DREAMLAND_EXPRESS) {
                WorkModule::inc_int(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SUCCESSFUL_DREAMLAND_EXPRESS_INPUTS);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_CAN_INPUT_DREAMLAND_EXPRESS);
            }
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_12"), 0.0, 1.0, false, 0.0, false, false);
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
            sv_kinetic_energy::clear_speed_ex(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            sv_kinetic_energy::set_motion_energy_update_flag(fighter.lua_state_agent);
            WorkModule::set_int64(fighter.module_accessor, attack_12, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_11"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::set_int64(fighter.module_accessor, attack_11, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
}

unsafe extern "C" fn littlemac_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    littlemac_status_attack_main_button(fighter, CONTROL_PAD_BUTTON_ATTACK.into())
}

unsafe extern "C" fn littlemac_status_attack_main_button(fighter: &mut L2CFighterCommon, button: L2CValue) -> L2CValue {
    let is_stop = fighter.global_table[IS_STOP].get_bool();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let combo = ComboModule::count(fighter.module_accessor) as i32;
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let attack_100_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_INT_100_COUNT);
    let mini_jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    let reserve_log_attack_kind =  WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    let attack100_type = WorkModule::get_param_int(fighter.module_accessor, hash40("attack100_type"), 0);
    let attack_combo_type = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_type"), 0);
    let attack_100_enable_cnt = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_100_enable_cnt"), 0);
    let attack_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_max"), 0);
    fighter.check_100_count_button(button.clone());
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if attack100_type != *FIGHTER_ATTACK100_TYPE_NONE {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100)
        && ControlModule::check_button_on(fighter.module_accessor, button.get_i32()) {
            if attack_combo_max <= combo && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) && situation_kind == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_100.into(), true.into());
                return 1.into();
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_100)
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100) {
            if attack_100_enable_cnt <= attack_100_count && situation_kind == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_100.into(), true.into());
                return 1.into();
            }
        }
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if 0 < mini_jump_attack_frame && !StopModule::is_stop(fighter.module_accessor) && fighter.sub_check_button_jump().get_bool() {
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new_raw(motion_kind), -1);
        WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        fighter.change_status_jump_mini_attack(true.into());
        return 1.into();
    }
    if 1 == mini_jump_attack_frame && !is_stop {
        if 0 < reserve_log_attack_kind {
            FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, reserve_log_attack_kind);
            WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    if attack_combo_type != *FIGHTER_COMBO_TYPE_NONE {
        if attack_combo_type == *FIGHTER_COMBO_TYPE_HIT && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART) {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
            return 1.into();
        }
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART) {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
            return 1.into();
        }
    }
    if littlemac_can_cancel_into_dash(fighter).get_bool() {
        fighter.change_status(FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn littlemac_attack_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT {
        EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_RESET_KO_GAUGE);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_KO_GAGE_MAX_KEEP_FRAME);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_REQUEST_KO_GAUGE_MAX_EFFECT);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_HELD_TIMER);
    }
    if status_kind != *FIGHTER_STATUS_KIND_ATTACK || motion_kind == hash40("attack_13") {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_CAN_INPUT_DREAMLAND_EXPRESS);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DREAMLAND_EXPRESS);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SUCCESSFUL_DREAMLAND_EXPRESS_INPUTS);
    }
    if 0 < attack_kind {
        FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, attack_kind);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    0.into()
}

pub fn install() {
    Agent::new("littlemac")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK, littlemac_attack_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK, littlemac_attack_end_status)
    .install()
    ;
}