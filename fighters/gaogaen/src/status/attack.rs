use super::*;

unsafe extern "C" fn gaogaen_attack_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Attack();
    fighter.sub_shift_status_main(L2CValue::Ptr(gaogaen_attack_main_loop as *const () as _))
}

unsafe extern "C" fn gaogaen_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    gaogaen_status_attack_main_button(fighter, CONTROL_PAD_BUTTON_ATTACK.into())
}

unsafe extern "C" fn gaogaen_status_attack_main_button(fighter: &mut L2CFighterCommon, button: L2CValue) -> L2CValue {
    let is_stop = fighter.global_table[IS_STOP].get_bool();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let combo = ComboModule::count(boma) as i32;
    let motion_kind = MotionModule::motion_kind(boma);
    let attack_100_count = WorkModule::get_int(boma, *FIGHTER_STATUS_ATTACK_WORK_INT_100_COUNT);
    let mini_jump_attack_frame = WorkModule::get_int(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    let reserve_log_attack_kind =  WorkModule::get_int64(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    let attack100_type = WorkModule::get_param_int(boma, hash40("attack100_type"), 0);
    let attack_combo_type = WorkModule::get_param_int(boma, hash40("attack_combo_type"), 0);
    let attack_100_enable_cnt = WorkModule::get_param_int(boma, hash40("attack_100_enable_cnt"), 0);
    let attack_combo_max = WorkModule::get_param_int(boma, hash40("attack_combo_max"), 0);
    fighter.check_100_count_button(button.clone());
    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if attack100_type != *FIGHTER_ATTACK100_TYPE_NONE {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL)
        && WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100)
        && ControlModule::check_button_on(boma, button.get_i32()) {
            if attack_combo_max <= combo && WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) && situation_kind == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_100.into(), true.into());
                return 1.into();
            }
        }
        if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_100)
        && WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100) {
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
    if 0 < mini_jump_attack_frame && !StopModule::is_stop(boma) && fighter.sub_check_button_jump().get_bool() {
        MotionAnimcmdModule::call_script_single(boma, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new_raw(motion_kind), -1);
        WorkModule::set_int64(boma, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        fighter.change_status_jump_mini_attack(true.into());
        return 1.into();
    }
    if 1 == mini_jump_attack_frame && !is_stop {
        if 0 < reserve_log_attack_kind {
            FighterStatusModuleImpl::reset_log_action_info(boma, reserve_log_attack_kind);
            WorkModule::set_int64(boma, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    if attack_combo_type != *FIGHTER_COMBO_TYPE_NONE {
        if attack_combo_type == *FIGHTER_COMBO_TYPE_HIT && WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_RESTART) {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
            return 1.into();
        }
    }
    else {
        if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_RESTART) {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
            return 1.into();
        }
    }
    if WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_ENABLE_JAB_GRAB) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) {
            WorkModule::off_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_ENABLE_JAB_GRAB);
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH.into(), false.into());
            return 1.into();
        }
    }
    if ![hash40("attack_11"), hash40("attack_12")].contains(&motion_kind) {
        if WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_FOLLOW_THROUGH) {
            if WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_ATTACK_HIT) {
                WorkModule::off_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_FOLLOW_THROUGH);
                WorkModule::off_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_ATTACK_HIT);
                MotionModule::change_motion(boma, Hash40::new("attack_13_hit"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
    }
    else {
        WorkModule::off_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_ATTACK_HIT);
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }
    0.into()
}

pub fn install() {
    Agent::new("gaogaen")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK, gaogaen_attack_main_status)
    .install()
    ;
}