use super::*;

unsafe extern "C" fn demon_attack_lw3_cancel_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    MotionModule::change_motion(boma, Hash40::new("attack_lw3_cancel"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(demon_attack_lw3_cancel_main_loop as *const () as _))
}

unsafe extern "C" fn demon_attack_lw3_cancel_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let cancel_status = WorkModule::get_int(boma, *FIGHTER_DEMON_STATUS_ATTACK_LW_3_WORK_INT_CANCEL_STATUS);
    if cancel_status == *STATUS_KIND_NONE {
        if CancelModule::is_enable_cancel(boma) {
            if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                if fighter.sub_air_check_fall_common().get_bool() {
                    return 0.into();
                }
            }
        }
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if cancel_status == *FIGHTER_STATUS_KIND_GUARD_ON {
        if !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            WorkModule::set_int(boma, -1, *FIGHTER_DEMON_STATUS_ATTACK_LW_3_WORK_INT_CANCEL_STATUS);
        }
    }
    if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK) {
        if WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_LW_3_FLAG_CHECK_STEP) {
            WorkModule::off_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_LW_3_FLAG_CHECK_STEP);
            fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_LW3_CANCEL_ATTACK.into(), true.into());
            return 1.into();
        }
    }
    if CancelModule::is_enable_cancel(boma) {
        if cancel_status != -1 {
            if cancel_status == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
                fighter.change_status(cancel_status.into(), false.into());
                return 0.into();
            }
            if cancel_status != *FIGHTER_STATUS_KIND_WAIT {
                fighter.change_status(cancel_status.into(), true.into());
                return 0.into();
            }
        }
    }
    if MotionModule::is_end(boma) {
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_DEMON_STATUS_KIND_ATTACK_LW3_CANCEL, demon_attack_lw3_cancel_main_status)
    .install()
    ;
}