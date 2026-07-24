use super::*;

unsafe extern "C" fn edge_special_n_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        PLAY_SE(fighter, Hash40::new("se_edge_special_n01_01"));
        fighter.sub_change_motion_by_situation(Hash40::new("special_n_start_wing").into(), Hash40::new("special_air_n_start_wing").into(), false.into());
    }
    else {
        fighter.sub_change_motion_by_situation(Hash40::new("special_n_start").into(), Hash40::new("special_air_n_start").into(), false.into());
    }
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_n").into());
    edge_special_kinetic_handler(fighter, true.into());
    WorkModule::set_int(boma, *FIGHTER_EDGE_SPECIAL_N_NONE, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    WorkModule::off_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_REQUEST_SHOOT);
    edge_enable_cancel_terms(boma);
    ControlModule::set_add_jump_mini_button_life(boma, 8);
    fighter.sub_shift_status_main(L2CValue::Ptr(edge_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn edge_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let cmd_cat2 = fighter.global_table[CMD_CAT2].get_i32();
    let boma = fighter.module_accessor;
    let mut get_sum_speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let air_invoke_speed_y_limit = WorkModule::get_param_float(boma, hash40("param_special_n"), hash40("air_invoke_speed_y_limit"));
    let cancel_start_frame = WorkModule::get_param_int(boma, hash40("param_special_n"), hash40("cancel_start_frame"));
    if !StatusModule::is_changing(boma) {
        if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
            fighter.sub_change_motion_by_situation(Hash40::new("special_n_start_wing").into(), Hash40::new("special_air_n_start_wing").into(), true.into());
        }
        else {
            fighter.sub_change_motion_by_situation(Hash40::new("special_n_start").into(), Hash40::new("special_air_n_start").into(), true.into());
        }
        fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_n").into());
        edge_special_kinetic_handler(fighter, false.into());
    }
    if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_REQUEST_SHOOT);
    }
    if WorkModule::is_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_REQUEST_SHOOT) {
        if WorkModule::get_int(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND) != *FIGHTER_EDGE_SPECIAL_N_NONE {
            fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
            return 0.into();
        }
    }
    if cancel_start_frame <= current_frame {
        if fighter.sub_check_command_guard().get_bool() {
            if situation_kind == *SITUATION_KIND_AIR {
                if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
                    WorkModule::set_int(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                    fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL.into(), false.into());
                    return 1.into();
                }
            }
            else {
                WorkModule::set_int(boma, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL.into(), false.into());
                return 1.into();
            }
        }
        else {
            if situation_kind != *SITUATION_KIND_GROUND {
                if fighter.sub_check_jump_in_charging().get_bool() {
                    WorkModule::set_int(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                    fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
                    return 0.into();
                }
            }
            else {
                if !fighter.sub_check_jump_in_charging().get_bool() {
                    if cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE != 0 {
                        WorkModule::set_int(boma, *FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                        fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
                        return 0.into();
                    }
                    if cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_F != 0 {
                        WorkModule::set_int(boma, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                        fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
                        return 0.into();
                    }
                    if cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_B != 0 {
                        WorkModule::set_int(boma, *FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                        fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
                        return 0.into();
                    }
                }
                else {
                    WorkModule::set_int(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                    fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
                    return 0.into();
                }
            }
        }
    }
    else {
        if situation_kind == *SITUATION_KIND_AIR {
            if air_invoke_speed_y_limit < get_sum_speed_y {
                get_sum_speed_y = air_invoke_speed_y_limit;
            }
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, get_sum_speed_y);
        }
    }
    if MotionModule::is_end(boma) {
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 0.into();
        }
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 0.into();
        }
    }
    0.into()
}

unsafe extern "C" fn edge_special_n_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let is_wing = WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED);
    let charge_kind = WorkModule::get_int(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    if is_wing {
        if charge_kind == *FIGHTER_EDGE_SPECIAL_N_XL {
            fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
        }
    }
    else {
        if charge_kind == *FIGHTER_EDGE_SPECIAL_N_L {
            fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
        }
    }
    (if is_wing {charge_kind == *FIGHTER_EDGE_SPECIAL_N_XL} else {charge_kind == *FIGHTER_EDGE_SPECIAL_N_L}).into()
}

unsafe extern "C" fn edge_special_n_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let charge_kind = WorkModule::get_int(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    let screen_effect_fade_frame = WorkModule::get_param_int(boma, hash40("param_special_n"), hash40("screen_effect_fade_frame"));
    if status_kind != *FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_SHOOT {
        EffectModule::remove_screen(boma, Hash40::new("edge_fire3_screen1"), screen_effect_fade_frame);
        MotionAnimcmdModule::enable_skip_delay_update(boma);
    }
    if charge_kind != *FIGHTER_EDGE_SPECIAL_N_XL {
        STOP_SE(fighter, Hash40::new("se_edge_special_n01_01"));
    }
    0.into()
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, edge_special_n_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, edge_special_n_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, edge_special_n_end_status)
    .install()
    ;
}