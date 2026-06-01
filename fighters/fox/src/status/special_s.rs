use super::*;

unsafe extern "C" fn fox_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let illusion_rush_speed_mul = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_rush_speed_mul"));
    let illusion_rush_speed_mul_power_up = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_rush_speed_mul_power_up"));
    WorkModule::off_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE);
    WorkModule::set_int(boma, -1, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    WorkModule::set_int(boma, *FIGHTER_FOX_ILLUSION_STEP_START, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
    WorkModule::set_int(boma, -1, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP_PREV);
    PostureModule::set_stick_lr(boma, 0.0);
    PostureModule::update_rot_y_lr(boma);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fox_special_s_air_mot(fighter);
    }
    else {
        fox_special_s_ground_mot(fighter);
    }
    WorkModule::set_float(boma, illusion_rush_speed_mul*illusion_rush_speed_mul_power_up, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);
    if !StopModule::is_stop(boma) {
        WorkModule::dec_int(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    }
    fighter.global_table[PREV_SUB_STATUS].assign(&L2CValue::Ptr(fox_special_s_substatus as *const () as _));
	fighter.sub_shift_status_main(L2CValue::Ptr(fox_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn fox_special_s_air_mot(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    if WorkModule::is_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE) {
        MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_air_s_start"), -1.0, 1.0, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(boma, Hash40::new("special_air_s_start"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE);
    }
}

unsafe extern "C" fn fox_special_s_ground_mot(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    if WorkModule::is_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE) {
        MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_s_start"), -1.0, 1.0, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(boma, Hash40::new("special_s_start"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE);
    }
}

unsafe extern "C" fn fox_special_s_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::dec_int(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    0.into()
}

unsafe extern "C" fn fox_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() 
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    let step = WorkModule::get_int(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    let mut cont = false;
    if !StatusModule::is_changing(boma) {
        let is_end = MotionModule::is_end(boma);
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            if step == *FIGHTER_FOX_ILLUSION_STEP_FORCE_END {
                WorkModule::set_int(boma, *FIGHTER_FOX_ILLUSION_STEP_END, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
                cont = true;
            }
            else if step == *FIGHTER_FOX_ILLUSION_STEP_END {
                if is_end && situation == *SITUATION_KIND_AIR {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
                    return 0.into();
                }
                if situation == *SITUATION_KIND_GROUND {
                    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                    GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                    MotionModule::change_motion(boma, Hash40::new("special_s_landing"), 0.0, 1.0, false, 0.0, false, false);
                    return 0.into();
                }
            }
            else if step == *FIGHTER_FOX_ILLUSION_STEP_RUSH || step == *FIGHTER_FOX_ILLUSION_STEP_START {
                if situation == *SITUATION_KIND_GROUND || is_end {
                    cont = true;
                }
            }
        }
        else {
            if step == *FIGHTER_FOX_ILLUSION_STEP_FORCE_END {
                WorkModule::set_int(boma, *FIGHTER_FOX_ILLUSION_STEP_END, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
                cont = true;
            }
            else if step == *FIGHTER_FOX_ILLUSION_STEP_END {
                if situation == *SITUATION_KIND_AIR {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
                    return 0.into();
                }
                if is_end && situation == *SITUATION_KIND_GROUND {
                    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
                    return 0.into();
                }
            }
            else if step == *FIGHTER_FOX_ILLUSION_STEP_RUSH || step == *FIGHTER_FOX_ILLUSION_STEP_START {
                if situation == *SITUATION_KIND_AIR || is_end {
                    cont = true;
                }
            }
        }
    }
    if cont {
        if !StatusModule::is_changing(boma) {
            if !WorkModule::is_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_RUSH_FORCE_END) && !WorkModule::is_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_HIT_SHIELD_TO_END) {
                if MotionModule::is_end(boma) {
                    WorkModule::inc_int(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
                    WorkModule::off_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE);
                }
            }
        }
        let step = WorkModule::get_int(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
        if situation != *SITUATION_KIND_GROUND {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if step == *FIGHTER_FOX_ILLUSION_STEP_END {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                fox_special_s_change_mot(fighter, Hash40::new("special_air_s_end"));
                fox_special_s_air_control(fighter);
                fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into());
            }
            else if step == *FIGHTER_FOX_ILLUSION_STEP_RUSH {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                GroundModule::select_cliff_hangdata(boma, *FIGHTER_FOX_CLIFF_HANG_DATA_SPECIAL_S as u32);
                fox_special_s_change_mot(fighter, Hash40::new("special_air_s"));
                fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP.into());
            }
            else if step == *FIGHTER_FOX_ILLUSION_STEP_START {
                fox_special_s_air_mot(fighter);
            }
        }
        else {
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
            if step == *FIGHTER_FOX_ILLUSION_STEP_END {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                fox_special_s_change_mot(fighter, Hash40::new("special_s_end"));
                fox_special_s_air_control(fighter);
            }
            else if step == *FIGHTER_FOX_ILLUSION_STEP_RUSH {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
                sv_kinetic_energy!(friction_off, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                let motion_mul = WorkModule::get_float(boma, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);
                sv_kinetic_energy!(set_speed_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, motion_mul);
                fox_special_s_change_mot(fighter, Hash40::new("special_s"));
            }
            else if step == *FIGHTER_FOX_ILLUSION_STEP_START {
                fox_special_s_ground_mot(fighter);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn fox_special_s_change_mot(fighter: &mut L2CFighterCommon, motion_kind: Hash40) {
    let boma = fighter.module_accessor;
    if !WorkModule::is_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE) {
        MotionModule::change_motion(boma, motion_kind, 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE);
    }
    else {
        MotionModule::change_motion_inherit_frame(boma, motion_kind, -1.0, 1.0, 0.0, false, false);
    }
}

unsafe extern "C" fn fox_special_s_air_control(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let air_accel_x_mul = WorkModule::get_param_float(boma, hash40("air_accel_x_mul"), 0);
    let air_accel_x_add = WorkModule::get_param_float(boma, hash40("air_accel_x_add"), 0);
    let air_speed_x_stable = WorkModule::get_param_float(boma, hash40("air_speed_x_stable"), 0);
    let illusion_end_control_air_speed_x_mul = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_end_control_air_speed_x_mul"));
    let illusion_end_control_air_speed_x_stable = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_end_control_air_speed_x_stable"));
    if WorkModule::is_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_AIR_CONTROL) {
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, *ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        sv_kinetic_energy!(controller_set_accel_x_mul, fighter, air_accel_x_mul*illusion_end_control_air_speed_x_mul);
        sv_kinetic_energy!(controller_set_accel_x_add, fighter, air_accel_x_add*illusion_end_control_air_speed_x_mul);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable*illusion_end_control_air_speed_x_stable, 0.0);
        WorkModule::off_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_AIR_CONTROL);
    }
}

unsafe extern "C" fn fox_special_s_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
	let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let fox_illusion_step = WorkModule::get_int(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
    let fox_illusion_prev_step = WorkModule::get_int(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP_PREV);
    let stop_y_frame = WorkModule::get_int(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    let illusion_end_air_brake_x = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_end_air_brake_x"));
    let illusion_end_air_accel_y = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_end_air_accel_y"));
    let illusion_accel_y = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_accel_y"));
    let illusion_stop_y_frame = WorkModule::get_param_int(boma, hash40("param_special_s"), hash40("illusion_stop_y_frame"));
    let get_brake_x = {fighter.clear_lua_stack(); lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP); sv_kinetic_energy::get_brake_x(fighter.lua_state_agent)};
    if situation_kind != *SITUATION_KIND_AIR {
        WorkModule::off_flag(boma, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_ILLUSION_LANDING);
    }
    else {
        WorkModule::on_flag(boma, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_ILLUSION_LANDING);
    };
    if fox_illusion_step != fox_illusion_prev_step {
        fox_special_s_handle_step(fighter);
    }
    if fox_illusion_step == *FIGHTER_FOX_ILLUSION_STEP_START {
        if situation_kind == *SITUATION_KIND_AIR {
            if stop_y_frame == 0 {
                if illusion_stop_y_frame != 0 {
                    sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0, 0.0, 0.0, 0.0, 0.0, 0.0);
                    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -illusion_accel_y);
                    KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                }
            }
        }
        if !WorkModule::is_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_RUSH_FORCE_END) {
            return 0.into();
        }
        WorkModule::inc_int(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
        WorkModule::inc_int(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
        WorkModule::on_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE);
        fox_special_s_handle_step(fighter);
        WorkModule::set_int(boma, *FIGHTER_FOX_ILLUSION_STEP_FORCE_END, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
    }
    else if fox_illusion_step == *FIGHTER_FOX_ILLUSION_STEP_RUSH {
        //Illusion Shorten implementation
        if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER != 0 {
            WorkModule::on_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_RUSH_FORCE_END);
        }
        if !WorkModule::is_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_RUSH_FORCE_END)
        && !WorkModule::is_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_HIT_SHIELD) {
            return 0.into();
        }
        WorkModule::inc_int(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
        WorkModule::on_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE);
        fox_special_s_handle_step(fighter);
        WorkModule::set_int(boma, *FIGHTER_FOX_ILLUSION_STEP_FORCE_END, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
        if WorkModule::is_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_HIT_SHIELD) {
            WorkModule::on_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_HIT_SHIELD_TO_END);
        }
    }
    else if fox_illusion_step == *FIGHTER_FOX_ILLUSION_STEP_END {
        if situation_kind == *SITUATION_KIND_AIR {
            //Fix friction if the value is, for some reason, incorrect.
            if get_brake_x != illusion_end_air_brake_x {
                sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, illusion_end_air_brake_x, 0.0);
            }
            if stop_y_frame == 0 {
                sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0, 0.0, 0.0, 0.0, 0.0, 0.0);
                sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -illusion_end_air_accel_y);
                KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }
        fox_special_s_air_control(fighter);
    }
    0.into()
}

unsafe extern "C" fn fox_special_s_handle_step(fighter: &mut L2CFighterCommon) {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let get_sum_speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = PostureModule::lr(boma);
    let fox_illusion_step = WorkModule::get_int(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
    let illusion_shield_hit_end_speed_x = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_shield_hit_end_speed_x"));
    let illusion_end_speed_x = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_end_speed_x"));
    let illusion_shield_hit_end_air_speed_x = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_shield_hit_end_air_speed_x"));
    let illusion_end_air_speed_x = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_end_air_speed_x"));
    let illusion_accel_x = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_accel_x"));
    let illusion_start_x_mul = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_start_x_mul"));
    let illusion_end_brake_x = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_end_brake_x"));
    let illusion_end_air_brake_x = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("illusion_end_air_brake_x"));
    let illusion_end_air_stop_y_frame = WorkModule::get_param_int(boma, hash40("param_special_s"), hash40("illusion_end_air_stop_y_frame"));
    let illusion_stop_y_frame = WorkModule::get_param_int(boma, hash40("param_special_s"), hash40("illusion_stop_y_frame"));
    WorkModule::set_int(boma, fox_illusion_step, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP_PREV);
    if fox_illusion_step == *FIGHTER_FOX_ILLUSION_STEP_START {
        WorkModule::set_int(boma, illusion_stop_y_frame, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
        if situation_kind != *SITUATION_KIND_AIR {
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, get_sum_speed_x*illusion_start_x_mul, 0.0);
        }
        else {
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 6, get_sum_speed_x*illusion_start_x_mul, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, illusion_accel_x, 0.0);
            KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            if illusion_stop_y_frame != 0 {
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
                sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
                KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, boma);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, boma);
    }
    else if fox_illusion_step == *FIGHTER_FOX_ILLUSION_STEP_END {
        WorkModule::set_int(boma, illusion_end_air_stop_y_frame, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
        if situation_kind != *SITUATION_KIND_AIR {
            sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, illusion_end_brake_x, 0.0);
            if WorkModule::is_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_HIT_SHIELD) {
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, illusion_shield_hit_end_speed_x*lr, 0.0);
            }
            else {
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, illusion_end_speed_x*lr, 0.0);
            }
            KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
        else {
            sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, illusion_end_air_brake_x, 0.0);
            if WorkModule::is_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_HIT_SHIELD) {
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, illusion_shield_hit_end_air_speed_x*lr, 0.0);
            }
            else {
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, illusion_end_air_speed_x*lr, 0.0);
            }
            KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        }
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 10.0, 10.0);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, boma);
    }
}

pub fn install() {
    Agent::new("fox")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, fox_special_s_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, fox_special_s_exec_status)
    .install()
    ;
}