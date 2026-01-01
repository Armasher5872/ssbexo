use super::*;

unsafe extern "C" fn pikachu_attack_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_AttackCommon();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.check_attack_mtrans();
    }
    fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(L2CFighterCommon_check_attack_mtrans as *const () as _));
    fighter.sub_status_AttackComboCommon();
    fighter.sub_shift_status_main(L2CValue::Ptr(pikachu_attack_main_loop as *const () as _))
}

unsafe extern "C" fn pikachu_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    pikachu_status_attack_main_button(fighter, CONTROL_PAD_BUTTON_ATTACK.into())
}

unsafe extern "C" fn pikachu_status_attack_main_button(fighter: &mut L2CFighterCommon, button: L2CValue) -> L2CValue {
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
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKACHU_INSTANCE_WORK_ID_INT_ATTACK_11_COUNT) > 1 {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_DASH.into(), true.into());
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKACHU_INSTANCE_WORK_ID_FLAG_ATTACK_11_DASH);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn pikachu_attack_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if 0 < attack_kind {
        FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, attack_kind);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_PIKACHU_INSTANCE_WORK_ID_INT_ATTACK_11_COUNT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PIKACHU_INSTANCE_WORK_ID_FLAG_ATTACK_11_DASH);
    0.into()
}

pub fn install() {
    Agent::new("pikachu")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK, pikachu_attack_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK, pikachu_attack_end_status)
    .install()
    ;
}