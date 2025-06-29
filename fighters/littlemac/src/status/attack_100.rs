use super::*;

unsafe extern "C" fn littlemac_attack_100_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_Attack100_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(littlemac_attack_100_main_loop as *const () as _))
}

unsafe extern "C" fn littlemac_attack_100_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    littlemac_status_attack100_main_uniq_func(fighter)
}

unsafe extern "C" fn littlemac_status_attack100_main_uniq_func(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_stop = fighter.global_table[IS_STOP].get_bool();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let status_attack = fighter.status_attack();
    let info = status_attack[0x10f40d7b92u64].get_i64();
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_INT_100_STEP);
    let jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    let reserve_log_attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    if 0 < jump_attack_frame {
        if !StopModule::is_stop(fighter.module_accessor) && fighter.sub_check_button_jump().get_bool() {
            MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new_raw(motion_kind), -1);
            WorkModule::set_int64(fighter.module_accessor, info, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            fighter.change_status_jump_mini_attack(true.into());
            return 0.into();
        }
    }
    if 1 == jump_attack_frame {
        if !is_stop && WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND) > 0 {
            FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, reserve_log_attack_kind);
            WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    if littlemac_can_cancel_into_dash(fighter).get_bool() {
        fighter.change_status(FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
        return 1.into();
    }
    if step == *FIGHTER_STATUS_ATTACK_100_STEP_START {
        if !StopModule::is_stop(fighter.module_accessor) {
            fighter.attack_100_start_uniq_chk(false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_attack_100_start_uniq_chk as *const () as _));
        if motion_kind == hash40("attack_100_start") && !MotionModule::is_end(fighter.module_accessor) {
            return 0.into();
        }
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_INPUT);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_100"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_100_STEP_LOOP, *FIGHTER_STATUS_ATTACK_WORK_INT_100_STEP);
    }
    else if step == *FIGHTER_STATUS_ATTACK_100_STEP_LOOP {
        if !StopModule::is_stop(fighter.module_accessor) {
            fighter.sub_attack_100_uniq_check(false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_attack_100_uniq_check as *const () as _));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE) {
            return 0.into();
        }
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_100_end"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ATTACK_WORK_INT_100_HIT_NEAR_COUNT);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_100_STEP_END, *FIGHTER_STATUS_ATTACK_WORK_INT_100_STEP);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_LOG_ATTACK_SUB_KIND_UNIQ, *FIGHTER_INSTANCE_WORK_ID_INT_TRICK_SUB);
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn littlemac_attack_100_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT {
        EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_RESET_KO_GAUGE);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_KO_GAGE_MAX_KEEP_FRAME);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_REQUEST_KO_GAUGE_MAX_EFFECT);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_HELD_TIMER);
    }
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_LOG_ATTACK_SUB_KIND_NONE, *FIGHTER_INSTANCE_WORK_ID_INT_TRICK_SUB);
    0.into()
}

pub fn install() {
    Agent::new("littlemac")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_100, littlemac_attack_100_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_100, littlemac_attack_100_end_status)
    .install()
    ;
}