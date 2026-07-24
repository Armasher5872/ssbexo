use super::*;

unsafe extern "C" fn edge_special_n_cancel_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    fighter.sub_change_motion_by_situation(Hash40::new("special_n_cancel").into(), Hash40::new("special_air_n_cancel").into(), false.into());
    ControlModule::set_add_jump_mini_button_life(boma, 8);
    fighter.sub_shift_status_main(L2CValue::Ptr(edge_special_n_cancel_main_loop as *const () as _))
}

unsafe extern "C" fn edge_special_n_cancel_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let is_enable_cancel = CancelModule::is_enable_cancel(boma);
    let is_changing = StatusModule::is_changing(boma);
    let cancel_status = WorkModule::get_int(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
    fighter.sub_check_charge_cancel_jump_mini_attack();
    if situation_kind == *SITUATION_KIND_GROUND {
        if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            if cancel_status == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
                FighterControlModuleImpl::update_attack_air_kind(boma, true);
            }
        }
    }
    if !is_changing {
        if prev_situation_kind == *SITUATION_KIND_GROUND
        && situation_kind == *SITUATION_KIND_AIR {
            WorkModule::set_int(boma, -1, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
        }
        if prev_situation_kind == *SITUATION_KIND_AIR
        && situation_kind == *SITUATION_KIND_GROUND {
            WorkModule::set_int(boma, -1, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
        }
    }
    if cancel_status == *STATUS_KIND_NONE {
        if is_enable_cancel {
            if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                if fighter.sub_air_check_fall_common().get_bool() {
                    return 0.into();
                }
            }
        }
    }
    if cancel_status == *FIGHTER_STATUS_KIND_GUARD_ON {
        if !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            WorkModule::set_int(boma, -1, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
        }
    }
    if is_enable_cancel {
        if cancel_status != -1 {
            fighter.change_status(cancel_status.into(), false.into());
            return 0.into();
        }
    }
    if MotionModule::is_end(boma) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    if !is_changing {
        fighter.sub_change_motion_by_situation(Hash40::new("special_n_cancel").into(), Hash40::new("special_air_n_cancel").into(), true.into());
        fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_n").into());
        edge_special_kinetic_handler(fighter, false);
    }
    0.into()
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL, edge_special_n_cancel_main_status)
    .install()
    ;
}