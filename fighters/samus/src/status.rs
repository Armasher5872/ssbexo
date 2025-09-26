use super::*;

unsafe extern "C" fn samus_attack_hi3_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION_RUN_STOP, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_3 as u32, 0);
    0.into()
}

unsafe extern "C" fn samus_attack_dash_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let catch_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("catch_dash_frame"));
    let jump_mini_attack_enable_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("jump_mini_attack_enable_frame"));
    let log_attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    let log_infos = fighter.status_attack()["log_infos"]["attack_dash"].get_int();
    let mini_jump_attack = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_dash"), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
    WorkModule::set_int(fighter.module_accessor, catch_dash_frame, *FIGHTER_STATUS_ATTACK_DASH_WORK_INT_CATCH_FRAME);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    WorkModule::set_int64(fighter.module_accessor, log_infos as i64, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
        WorkModule::set_int(fighter.module_accessor, jump_mini_attack_enable_frame+1, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    }
    if mini_jump_attack != 0 {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) && log_attack_kind > 0 {
            FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log_attack_kind);
            WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_attack_dash_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_sub_attack_dash_uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(samus_attack_dash_main_loop as *const () as _))
}

unsafe extern "C" fn samus_attack_dash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let stick_x = fighter.global_table[STICK_X].get_f32()*PostureModule::lr(boma);
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let status_attack = fighter.status_attack();
    let status_attack_info = status_attack[0x10f40d7b92u64].get_i64();
    let motion_kind = MotionModule::motion_kind(boma);
    let reserve_log = WorkModule::get_int64(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    let mini_jump_attack_frame = WorkModule::get_int(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    let turn_run_stick_x = WorkModule::get_param_float(boma, hash40("common"), hash40("turn_run_stick_x"));
    if CancelModule::is_enable_cancel(boma) && fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool() {
        return 0.into();
    }
    if 0 < mini_jump_attack_frame {
        if !StopModule::is_stop(boma) && fighter.sub_check_button_jump().get_bool() {
            MotionAnimcmdModule::call_script_single(boma, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new_raw(motion_kind), -1);
            WorkModule::set_int64(boma, status_attack_info, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            fighter.change_status_jump_mini_attack(true.into());
            return 1.into();
        }
    }
    if 1 == mini_jump_attack_frame {
        if !fighter.global_table[IS_STOP].get_bool() && reserve_log > 0 {
            FighterStatusModuleImpl::reset_log_action_info(boma, reserve_log);
            WorkModule::set_int64(boma, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    /* START OF NEW ADDITIONS */
    //Gatlings (DACUS/DACDS)
    if attack_hi4_cancel(fighter) {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI3.into(), true.into());
        return 1.into();
    }
    if attack_lw4_cancel(fighter) {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW4_START.into(), true.into());
        return 1.into();
    }
    /* END OF NEW ADDITIONS */
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN)
    && stick_x <= turn_run_stick_x
    && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD)
    && !ItemModule::is_have_item(boma, 0) {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_TURN.into(), true.into());
        return 0.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH) 
    && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) 
    && !ItemModule::is_have_item(boma, 0) {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), true.into());
        return 0.into();
    }
    if MotionModule::is_end(boma) {
        let status = if situation_kind != *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_FALL
        }
        else {
            FIGHTER_STATUS_KIND_WAIT
        };
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn samus_special_s1a_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn samus_special_s2a_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT | *FIGHTER_LOG_MASK_FLAG_HAJIKI) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

pub fn install() {
    Agent::new("samus")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_DASH, samus_attack_dash_main_status)
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_HI3, samus_attack_hi3_pre_status)
    .status(Pre, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, samus_special_s1a_pre_status)
    .status(Pre, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, samus_special_s2a_pre_status)
    .install()
    ;
}