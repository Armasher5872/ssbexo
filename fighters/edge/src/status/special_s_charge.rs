use super::*;

unsafe extern "C" fn edge_special_s_charge_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    fighter.sub_change_motion_by_situation(Hash40::new("special_s_hold").into(), Hash40::new("special_s_air_hold").into(), false.into());
    WorkModule::set_int(boma, 0, *FIGHTER_EDGE_STATUS_SPECIAL_S_INT_HOLD_FRAME);
    fighter.sub_shift_status_main(L2CValue::Ptr(edge_special_s_charge_main_loop as *const () as _))
}

unsafe extern "C" fn edge_special_s_charge_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let boma = fighter.module_accessor;
    let hold_max = WorkModule::get_param_int(boma, hash40("param_special_s"), hash40("hold_max"));
    let hold_frame = WorkModule::get_int(boma, *FIGHTER_EDGE_STATUS_SPECIAL_S_INT_HOLD_FRAME);
    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if !StatusModule::is_changing(boma) {
        fighter.sub_change_motion_by_situation(Hash40::new("special_s_hold").into(), Hash40::new("special_s_air_hold").into(), true.into());
        fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_s").into());
        edge_special_kinetic_handler(fighter, false);
    }
    if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
        if 0.0 < current_frame {
            edge_special_s_reverse_function(fighter);
            fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_SHOOT.into(), false.into());
        }
    }
    if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        if fighter.sub_check_command_guard().get_bool() {
            fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_CANCEL.into(), false.into());
            return 1.into();
        }
    }
    if hold_max <= hold_frame {
        edge_special_s_reverse_function(fighter);
        fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_SHOOT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn edge_special_s_reverse_function(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let stick_x = fighter.global_table[STICK_X].get_f32()*PostureModule::lr(boma);
    let turn_stick_x = WorkModule::get_param_float(boma, hash40("common"), hash40("turn_stick_x"));
    if stick_x <= turn_stick_x {
        PostureModule::reverse_lr(boma);
        PostureModule::update_rot_y_lr(boma);
    }
    0.into()
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_CHARGE, edge_special_s_charge_main_status)
    .install()
    ;
}