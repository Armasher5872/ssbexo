use super::*;

unsafe extern "C" fn demon_special_n_ground_start_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_n").into());
    MotionModule::change_motion(boma, Hash40::new("special_n_start"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(demon_special_n_ground_start_main_loop as *const () as _))
}

unsafe extern "C" fn demon_special_n_ground_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let special_n_timer = WorkModule::get_int(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_INT_SPECIAL_N_HOLD_FRAME);
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) && special_n_timer < 14 {
        WorkModule::inc_int(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_INT_SPECIAL_N_HOLD_FRAME);
    }
    fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_n").into());
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_DEMON_STATUS_KIND_SPECIAL_N_GROUND_SHOOT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn demon_special_n_ground_start_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    if status_kind != *FIGHTER_DEMON_STATUS_KIND_SPECIAL_N_GROUND_SHOOT {
        WorkModule::set_int(boma, 0, *FIGHTER_DEMON_INSTANCE_WORK_ID_INT_SPECIAL_N_HOLD_FRAME);
    }
    0.into()
}

unsafe extern "C" fn demon_special_n_ground_start_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    if status_kind != *FIGHTER_DEMON_STATUS_KIND_SPECIAL_N_GROUND_SHOOT {
        WorkModule::set_int(boma, 0, *FIGHTER_DEMON_INSTANCE_WORK_ID_INT_SPECIAL_N_HOLD_FRAME);
    }
    0.into()
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_DEMON_STATUS_KIND_SPECIAL_N_GROUND_START, demon_special_n_ground_start_main_status)
    .status(End, *FIGHTER_DEMON_STATUS_KIND_SPECIAL_N_GROUND_START, demon_special_n_ground_start_end_status)
    .status(Exit, *FIGHTER_DEMON_STATUS_KIND_SPECIAL_N_GROUND_START, demon_special_n_ground_start_exit_status)
    .install()
    ;
}