/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//Sub Status Attack Common, removes the combo check
#[skyline::hook(replace = L2CFighterCommon_sub_status_AttackCommon)]
unsafe extern "C" fn sub_status_attackcommon(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    if fighter.global_table[PREV_STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_ATTACK {
        WorkModule::set_int(boma, 0, *FIGHTER_STATUS_ATTACK_WORK_INT_100_HIT_NEAR_COUNT);
    }
    ComboModule::reset(boma);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_100);
    WorkModule::set_int64(boma, hash40("attack_11") as i64, *FIGHTER_STATUS_ATTACK_WORK_INT_ATTACK11_MOTION);
}

//The following five are reimplemented to make sure only Neutral Attack inputs can trigger Jab followups.
#[skyline::hook(replace = L2CFighterCommon_attack_combo_none_uniq_chk_button)]
unsafe extern "C" fn attack_combo_none_uniq_chk_button(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue, param_3: L2CValue) {
    let boma = fighter.module_accessor;
    if !param_1.get_bool() {
        if ControlModule::check_button_on(boma, param_2.get_i32()) && only_jabs(fighter) {
            if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART) {
                WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_RESTART);
                WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_RESTART_ATTACK);
            }
        }
        fighter.attack_uniq_chk_command(param_3);
    }
    else {
        if !WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
            if !WorkModule::count_down_int(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME, 0) {
                return;
            }
        }
        WorkModule::set_int(boma, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
        WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    }
}

#[skyline::hook(replace = L2CFighterCommon_attack_combo_uniq_chk_button)]
unsafe extern "C" fn attack_combo_uniq_chk_button(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue, param_3: L2CValue) {
    let boma = fighter.module_accessor;
    if !param_1.get_bool() {
        fighter.attack_uniq_chk_command(param_3.clone());
        if fighter.global_table[CMD_CAT1].get_i32() & param_3.get_i32() != 0 && only_jabs(fighter) {
            if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
                WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO);
            }
        }
        let button = param_2.get_i32();
        if !ControlModule::check_button_on(boma, button) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_RELEASE_BUTTON);
        }
        else {
            if !AttackModule::is_infliction_status(boma, 0x7f) {
                if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART) {
                    if !WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_RELEASE_BUTTON) {
                        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_RESTART);
                        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_RESTART_ATTACK);
                        ComboModule::reset(boma);
                    }
                }
                if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO) && only_jabs(fighter) {
                    WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO);
                    if ControlModule::check_button_on_trriger(boma, button) {
                        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO_TRIGGER);
                    }
                }
            }
            else if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) && only_jabs(fighter) {
                WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO);
            }
        }
        if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_RESTART_COMBO) && WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO) {
            let combo_count = ComboModule::count(boma) as i32;
            let attack_combo_max = WorkModule::get_param_int(boma, hash40("attack_combo_max"), 0);
            if combo_count != attack_combo_max {
                return;
            }
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_RESTART);
            ComboModule::reset(boma);
            if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO) {
                if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO_TRIGGER) {
                    return;
                }
                WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_RESTART_ATTACK);
            }
        }
    }
    else {
        if !WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
            if !WorkModule::count_down_int(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME, 0) {
                return;
            }
        }
        WorkModule::set_int(boma, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
        WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    }
}

#[skyline::hook(replace = L2CFighterCommon_attack_uniq_chk_command)]
unsafe extern "C" fn attack_uniq_chk_command(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let boma = fighter.module_accessor;
    if cmd_cat1 & param_1.get_i32() != 0 && only_jabs(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE);
    }
}

#[skyline::hook(replace = L2CFighterCommon_check_100_count_button)]
unsafe extern "C" fn check_100_count_button(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    let button = param_1.get_i32();
    let boma = fighter.module_accessor;
    if only_jabs(fighter) {
        if fighter.global_table[IS_STOP].get_bool() {
            if !ControlModule::check_button_on_trriger(boma, button) && !ControlModule::check_button_on_release(boma, button) {
                return;
            }
            WorkModule::inc_int(boma, *FIGHTER_STATUS_ATTACK_WORK_INT_100_COUNT);
        }
        else {
            if !ControlModule::check_button_trigger(boma, button) && !ControlModule::check_button_release(boma, button) {
                return;
            }
            WorkModule::inc_int(boma, *FIGHTER_STATUS_ATTACK_WORK_INT_100_COUNT);
        }
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_Attack_Main_button)]
unsafe extern "C" fn status_attack_main_button(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue) -> L2CValue {
    let boma = fighter.module_accessor;
    fighter.check_100_count_button(param_1.clone());
    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    let attack100_type = WorkModule::get_param_int(boma, hash40("attack100_type"), 0);
    if attack100_type != *FIGHTER_ATTACK100_TYPE_NONE {
        if AttackModule::is_infliction_status(boma, 0x7f)
        && WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100)
        && ControlModule::check_button_on(boma, param_1.get_i32())
        && only_jabs(fighter) {
            let combo = ComboModule::count(boma) as i32;
            let attack_combo_max = WorkModule::get_param_int(boma, hash40("attack_combo_max"), 0);
            if attack_combo_max <= combo && WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_100.into(), true.into());
                return 1.into();
            }
        }
        if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_100)
        && WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100) {
            let attack_100_count = WorkModule::get_int(boma, *FIGHTER_STATUS_ATTACK_WORK_INT_100_COUNT);
            let attack_100_enable_cnt = WorkModule::get_param_int(boma, hash40("attack_100_enable_cnt"), 0);
            if attack_100_enable_cnt <= attack_100_count && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_100.into(), true.into());
                return 1.into();
            }
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if 0 < WorkModule::get_int(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME) && !StopModule::is_stop(boma) && fighter.sub_check_button_jump().get_bool() {
        let mot = MotionModule::motion_kind(boma);
        MotionAnimcmdModule::call_script_single(boma, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new_raw(mot), -1);
        WorkModule::set_int64(boma, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        let callable: extern "C" fn(&mut L2CFighterCommon, L2CValue) -> L2CValue = std::mem::transmute(param_2.get_ptr());
        callable(fighter, true.into());
        return 1.into();
    }
    if 1 == WorkModule::get_int(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME) && !fighter.global_table[IS_STOP].get_bool() {
        let kind =  WorkModule::get_int64(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        if 0 < kind {
            FighterStatusModuleImpl::reset_log_action_info(boma, kind);
            WorkModule::set_int64(boma, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    let attack_combo_type = WorkModule::get_param_int(boma, hash40("attack_combo_type"), 0);
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
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_status_attackcommon,
            attack_combo_none_uniq_chk_button,
            attack_combo_uniq_chk_button,
            attack_uniq_chk_command,
            check_100_count_button,
            status_attack_main_button
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}