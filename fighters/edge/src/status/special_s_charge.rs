use super::*;

unsafe extern "C" fn edge_special_s_charge_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(Hash40::new("special_s_hold").into(), Hash40::new("special_s_air_hold").into(), false.into());
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_EDGE_STATUS_SPECIAL_S_INT_HOLD_FRAME);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(edge_special_s_charge_main_loop as *const () as _))
}

unsafe extern "C" fn edge_special_s_charge_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let hold_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("hold_max"));
    let hold_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_S_INT_HOLD_FRAME);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        fighter.sub_change_motion_by_situation(Hash40::new("special_s_hold").into(), Hash40::new("special_s_air_hold").into(), true.into());
        fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_s").into());
        edge_special_kinetic_handler(fighter, false);
    }
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if 0.0 < current_frame {
            edge_special_s_reverse_function(fighter);
            fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_SHOOT.into(), false.into());
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        if hold_frame >= 5 {
            if fighter.sub_check_command_guard().get_bool() {
                if situation_kind == *SITUATION_KIND_GROUND {
                    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                    fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL.into(), false.into());
                    return 1.into()
                }
                else {
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
                        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                        fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL.into(), false.into());
                        return 1.into()
                    }
                }
            }
            else {
                if situation_kind == *SITUATION_KIND_AIR {
                    if fighter.sub_check_jump_in_charging().get_bool() {
                        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                        fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
                    }
                }
                else {
                    if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON != 0
                    || (ControlModule::is_enable_flick_jump(fighter.module_accessor) && (cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0) && fighter.sub_check_button_frick().get_bool()) {
                        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                        fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
                    }
                }
            }
        }
    }
    if hold_max <= hold_frame {
        edge_special_s_reverse_function(fighter);
        fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_SHOOT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn edge_special_s_reverse_function(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32()*PostureModule::lr(fighter.module_accessor);
    let turn_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_stick_x"));
    if stick_x <= turn_stick_x {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
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