use super::*;

unsafe extern "C" fn littlemac_attack_dash_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    MotionModule::change_motion(boma, Hash40::new("attack_dash"), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
    let catch_dash_frame = WorkModule::get_param_int(boma, hash40("common"), hash40("catch_dash_frame"));
    WorkModule::set_int(boma, catch_dash_frame, *FIGHTER_STATUS_ATTACK_DASH_WORK_INT_CATCH_FRAME);
    WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_DASH_FLAG_ATTACK_HI4_DISABLE);
    if !WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
        let jump_mini_attack_enable_frame = WorkModule::get_param_int(boma, hash40("common"), hash40("jump_mini_attack_enable_frame"));
        WorkModule::set_int(boma, jump_mini_attack_enable_frame + 1, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    }
    let log = fighter.status_attack()["log_infos"]["attack_dash"].get_int();
    WorkModule::set_int64(boma, log as i64, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    let mini_jump_attack = WorkModule::get_int(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    if mini_jump_attack != 0 {
        if WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK)
        && WorkModule::get_int64(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND) > 0 {
            let log = WorkModule::get_int64(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            FighterStatusModuleImpl::reset_log_action_info(boma, log);
            WorkModule::set_int64(boma, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    if !StopModule::is_stop(boma) {
        fighter.sub_attack_dash_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_sub_attack_dash_uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(littlemac_attack_dash_main_loop as *const () as _))
}

unsafe extern "C" fn littlemac_attack_dash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if littlemac_can_cancel_into_dash(fighter).get_bool() {
        fighter.change_status(FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
        return 1.into();
    }
    fighter.status_AttackDash_Main()
}

unsafe extern "C" fn littlemac_attack_dash_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    let attack_kind = WorkModule::get_int64(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT {
        EffectModule::remove_common(boma, Hash40::new("charge_max"));
        WorkModule::on_flag(boma, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_RESET_KO_GAUGE);
        WorkModule::set_float(boma, 0.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
        WorkModule::set_int(boma, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_KO_GAGE_MAX_KEEP_FRAME);
        WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_REQUEST_KO_GAUGE_MAX_EFFECT);
        WorkModule::set_int(boma, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
        WorkModule::set_int(boma, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_HELD_TIMER);
    }
    if 0 < attack_kind {
        FighterStatusModuleImpl::reset_log_action_info(boma, attack_kind);
        WorkModule::set_int(boma, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    0.into()
}

pub fn install() {
    Agent::new("littlemac")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_DASH, littlemac_attack_dash_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_DASH, littlemac_attack_dash_end_status)
    .install()
    ;
}