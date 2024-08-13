use super::*;

unsafe extern "C" fn gaogaen_attack_lw4_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLOAT, (*FS_SUCCEEDS_KEEP_HIT | *FS_SUCCEEDS_KEEP_VISIBILITY | *FS_SUCCEEDS_KEEP_NO_REACTION));
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_LW4 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_HAJIKI) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_4 as u32, 0);
    0.into()
}

unsafe extern "C" fn gaogaen_attack_lw4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let restart_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
    ControlModule::reset_trigger(fighter.module_accessor);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    ComboModule::set(fighter.module_accessor, *FIGHTER_COMBO_KIND_S4);
    MotionModule::change_motion_force_inherit_frame(fighter.module_accessor, Hash40::new("attack_lw4"), restart_frame, 1.0, 0.0);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_THROW_KIRBY_GROUND);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.status_ThrowKirby_Uniq(L2CValue::Bool(false));
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(smash::lua2cpp::L2CFighterCommon_status_ThrowKirby_Uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(gaogaen_attack_lw4_main_loop as *const () as _))
}

unsafe extern "C" fn gaogaen_attack_lw4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor)
    && !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_THROW_KIRBY_GROUND)
    && !MotionModule::is_end(fighter.module_accessor) {
        fighter.sub_status_uniq_process_ThrowKirby_execFixPos();
        return 0.into()
    }
    fighter.status_AttackLw4_Main_param(FIGHTER_STATUS_KIND_WAIT.into());
    0.into()
}

unsafe extern "C" fn gaogaen_attack_lw4_map_correction_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let pass_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_stick_y"));
    let frame = MotionModule::frame(fighter.module_accessor);
    let prev_frame = MotionModule::prev_frame(fighter.module_accessor);
    let start_air_frame = 2.0;
    let fall_start_frame = 16.0;
    let fall_stop_frame = 17.0;
    let landing_frame = 19.0;
    if frame <= fall_start_frame {
        return 0.into()
    }
    if prev_frame < start_air_frame 
    && frame >= start_air_frame {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_THROW_FLAG_START_AIR);
    }
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        if prev_frame < fall_stop_frame && frame >= fall_stop_frame {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -11.0);
            sv_kinetic_energy!(set_accel_x_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
            sv_kinetic_energy!(set_accel_x_add, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
            MotionModule::set_frame(fighter.module_accessor, fall_stop_frame, true);
            MotionModule::set_rate(fighter.module_accessor, 0.0);
        }
    }
    else {
        if frame < landing_frame {
            if stick_y <= pass_stick_y {
                GroundModule::set_passable_check(fighter.module_accessor, true);
            }
            else {
                GroundModule::set_passable_check(fighter.module_accessor, false);
            }
            if GroundModule::is_passable_check(fighter.module_accessor)
            && GroundModule::is_passable_ground(fighter.module_accessor) {
                fighter.set_situation(SITUATION_KIND_AIR.into());
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            }
            else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_THROW_KIRBY_GROUND);
                MotionModule::set_frame(fighter.module_accessor, landing_frame, true);
                MotionModule::set_rate(fighter.module_accessor, 1.0);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn gaogaen_catch_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    fighter.sub_status_Catch();
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_CAN_ANGLE_CATCH);
    fighter.sub_shift_status_main(L2CValue::Ptr(gaogaen_catch_main_loop as *const () as _))
}

unsafe extern "C" fn gaogaen_catch_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() && fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_CAN_ANGLE_CATCH)
    && MotionModule::motion_kind(fighter.module_accessor) == hash40("catch") {
        if stick_y >= 0.7 {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("catch_hi"), -1.0, 1.0, 0.0, false, false);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_CAN_ANGLE_CATCH);
        }
        else if stick_y <= -0.7 {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("catch_lw"), -1.0, 1.0, 0.0, false, false);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_CAN_ANGLE_CATCH);
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
        if MotionModule::is_end(fighter.module_accessor) {
            if situation_kind != *SITUATION_KIND_GROUND {
                return 0.into();
            }
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }
    }
    0.into()
}

unsafe extern "C" fn gaogaen_catch_dash_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    fighter.sub_status_CatchDash();
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_CAN_ANGLE_CATCH);
    fighter.sub_shift_status_main(L2CValue::Ptr(gaogaen_catch_dash_main_loop as *const () as _))
}

unsafe extern "C" fn gaogaen_catch_dash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let turn_run_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_run_stick_x"));
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() && fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN) {
        if stick_x*lr <= turn_run_stick_x {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_TURN.into(), true.into());
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_CAN_ANGLE_CATCH)
    && MotionModule::motion_kind(fighter.module_accessor) == hash40("catch_dash") {
        if stick_y >= 0.7 {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("catch_dash_hi"), -1.0, 1.0, 0.0, false, false);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_CAN_ANGLE_CATCH);
        }
        else if stick_y <= -0.7 {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("catch_dash_lw"), -1.0, 1.0, 0.0, false, false);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_CAN_ANGLE_CATCH);
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
        if MotionModule::is_end(fighter.module_accessor) {
            if situation_kind != *SITUATION_KIND_GROUND {
                return 0.into();
            }
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn gaogaen_catch_turn_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    fighter.sub_status_CatchTurn();
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_CAN_ANGLE_CATCH);
    fighter.sub_shift_status_main(L2CValue::Ptr(gaogaen_catch_turn_main_loop as *const () as _))
}

unsafe extern "C" fn gaogaen_catch_turn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() && fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_CAN_ANGLE_CATCH)
    && MotionModule::motion_kind(fighter.module_accessor) == hash40("catch") {
        if stick_y >= 0.7 {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("catch_hi"), -1.0, 1.0, 0.0, false, false);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_CAN_ANGLE_CATCH);
        }
        else if stick_y <= -0.7 {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("catch_lw"), -1.0, 1.0, 0.0, false, false);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_CAN_ANGLE_CATCH);
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
        if MotionModule::is_end(fighter.module_accessor) {
            if situation_kind != *SITUATION_KIND_GROUND {
                return 0.into();
            }
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }
    }
    0.into()
}

unsafe extern "C" fn gaogaen_catch_pull_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if fighter.sub_is_throw_status_kind(status_kind.into()).get_bool() {
        let last_strans = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
        if last_strans != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_F {
            if last_strans != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_B {
                if last_strans != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI {
                    if last_strans != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_LW {
                        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
                            WorkModule::set_int64(fighter.module_accessor, hash40("throw_f_revenge") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                        }
                        else {
                            WorkModule::set_int64(fighter.module_accessor, hash40("throw_f") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                        }
                    }
                    else {
                        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
                            WorkModule::set_int64(fighter.module_accessor, hash40("throw_lw_revenge") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                        }
                        else {
                            WorkModule::set_int64(fighter.module_accessor, hash40("throw_lw") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                        }
                    }
                }
                else {
                    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
                        WorkModule::set_int64(fighter.module_accessor, hash40("throw_hi_revenge") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                    }
                    else {
                        WorkModule::set_int64(fighter.module_accessor, hash40("throw_hi") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);   
                    }
                }
            }
            else {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
                    WorkModule::set_int64(fighter.module_accessor, hash40("throw_b_revenge") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                }
                else {
                    WorkModule::set_int64(fighter.module_accessor, hash40("throw_b") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);   
                }
            }
        }
        else {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
                WorkModule::set_int64(fighter.module_accessor, hash40("throw_f_revenge") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
            }
            else {
                WorkModule::set_int64(fighter.module_accessor, hash40("throw_f") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);   
            }
        }
    }
    0.into()
}

unsafe extern "C" fn gaogaen_catch_dash_pull_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if fighter.sub_is_throw_status_kind(status_kind.into()).get_bool() {
        let last_strans = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
        if last_strans != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_F {
            if last_strans != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_B {
                if last_strans != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI {
                    if last_strans != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_LW {
                        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
                            WorkModule::set_int64(fighter.module_accessor, hash40("throw_f_revenge") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                        }
                        else {
                            WorkModule::set_int64(fighter.module_accessor, hash40("throw_f") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                        }
                    }
                    else {
                        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
                            WorkModule::set_int64(fighter.module_accessor, hash40("throw_lw_revenge") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                        }
                        else {
                            WorkModule::set_int64(fighter.module_accessor, hash40("throw_lw") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                        }
                    }
                }
                else {
                    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
                        WorkModule::set_int64(fighter.module_accessor, hash40("throw_hi_revenge") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                    }
                    else {
                        WorkModule::set_int64(fighter.module_accessor, hash40("throw_hi") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);   
                    }
                }
            }
            else {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
                    WorkModule::set_int64(fighter.module_accessor, hash40("throw_b_revenge") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                }
                else {
                    WorkModule::set_int64(fighter.module_accessor, hash40("throw_b") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);   
                }
            }
        }
        else {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
                WorkModule::set_int64(fighter.module_accessor, hash40("throw_f_revenge") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
            }
            else {
                WorkModule::set_int64(fighter.module_accessor, hash40("throw_f") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);   
            }
        }
    }
    0.into()
}

unsafe extern "C" fn gaogaen_catch_wait_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if fighter.sub_is_throw_status_kind(status_kind.into()).get_bool() {
        let last_strans = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
        if last_strans != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_F {
            if last_strans != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_B {
                if last_strans != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI {
                    if last_strans != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_LW {
                        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
                            WorkModule::set_int64(fighter.module_accessor, hash40("throw_f_revenge") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                        }
                        else {
                            WorkModule::set_int64(fighter.module_accessor, hash40("throw_f") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                        }
                    }
                    else {
                        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
                            WorkModule::set_int64(fighter.module_accessor, hash40("throw_lw_revenge") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                        }
                        else {
                            WorkModule::set_int64(fighter.module_accessor, hash40("throw_lw") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                        }
                    }
                }
                else {
                    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
                        WorkModule::set_int64(fighter.module_accessor, hash40("throw_hi_revenge") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                    }
                    else {
                        WorkModule::set_int64(fighter.module_accessor, hash40("throw_hi") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);   
                    }
                }
            }
            else {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
                    WorkModule::set_int64(fighter.module_accessor, hash40("throw_b_revenge") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                }
                else {
                    WorkModule::set_int64(fighter.module_accessor, hash40("throw_b") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);   
                }
            }
        }
        else {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
                WorkModule::set_int64(fighter.module_accessor, hash40("throw_f_revenge") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
            }
            else {
                WorkModule::set_int64(fighter.module_accessor, hash40("throw_f") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);   
            }
        }
    }
    0.into()
}

unsafe extern "C" fn gaogaen_catch_attack_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if fighter.sub_is_throw_status_kind(status_kind.into()).get_bool() {
        let last_strans = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
        if last_strans != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_F {
            if last_strans != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_B {
                if last_strans != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI {
                    if last_strans != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_LW {
                        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
                            WorkModule::set_int64(fighter.module_accessor, hash40("throw_f_revenge") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                        }
                        else {
                            WorkModule::set_int64(fighter.module_accessor, hash40("throw_f") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                        }
                    }
                    else {
                        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
                            WorkModule::set_int64(fighter.module_accessor, hash40("throw_lw_revenge") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                        }
                        else {
                            WorkModule::set_int64(fighter.module_accessor, hash40("throw_lw") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                        }
                    }
                }
                else {
                    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
                        WorkModule::set_int64(fighter.module_accessor, hash40("throw_hi_revenge") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                    }
                    else {
                        WorkModule::set_int64(fighter.module_accessor, hash40("throw_hi") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);   
                    }
                }
            }
            else {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
                    WorkModule::set_int64(fighter.module_accessor, hash40("throw_b_revenge") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                }
                else {
                    WorkModule::set_int64(fighter.module_accessor, hash40("throw_b") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);   
                }
            }
        }
        else {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
                WorkModule::set_int64(fighter.module_accessor, hash40("throw_f_revenge") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
            }
            else {
                WorkModule::set_int64(fighter.module_accessor, hash40("throw_f") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);   
            }
        }
    }
    0.into()
}

unsafe extern "C" fn gaogaen_throw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
    let motion_rate = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_FLOAT_MOTION_RATE);
    ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion_kind), 0.0, motion_rate, false, 0.0, false, false);
    gaogaen_throw_log_common(fighter);
    if !StopModule::is_stop(fighter.module_accessor) {
        gaogaen_throw_uniq(fighter);
    }
    fighter.global_table[PREV_SUB_STATUS].assign(&L2CValue::Ptr(gaogaen_throw_uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(gaogaen_throw_main_loop as *const () as _))
}

unsafe extern "C" fn gaogaen_throw_log_common(fighter: &mut L2CFighterCommon) {
    let motion_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
    if ![hash40("throw_f"), hash40("throw_f_revenge")].contains(&motion_kind) {
        if ![hash40("throw_b"), hash40("throw_b_revenge")].contains(&motion_kind) {
            if ![hash40("throw_hi"), hash40("throw_hi_revenge")].contains(&motion_kind) {
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), *FIGHTER_LOG_ACTION_CATEGORY_ATTACK, *FIGHTER_LOG_ATTACK_KIND_THROW_LW);
                fighter.pop_lua_stack(1);
            }
            else {
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), *FIGHTER_LOG_ACTION_CATEGORY_ATTACK, *FIGHTER_LOG_ATTACK_KIND_THROW_HI);
                fighter.pop_lua_stack(1);
            }
        }
        else {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), *FIGHTER_LOG_ACTION_CATEGORY_ATTACK, *FIGHTER_LOG_ATTACK_KIND_THROW_B);
            fighter.pop_lua_stack(1);
        }
    }
    else {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), *FIGHTER_LOG_ACTION_CATEGORY_ATTACK, *FIGHTER_LOG_ATTACK_KIND_THROW_F);
        fighter.pop_lua_stack(1);
    }
}

unsafe extern "C" fn gaogaen_throw_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_THROW_FLAG_INVINCIBLE) {
        if !CatchModule::is_catch(fighter.module_accessor) {
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_THROW_FLAG_INVINCIBLE);
        }
    }
    0.into()
}

unsafe extern "C" fn gaogaen_throw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let special_zoom_gfx = WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() && fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_THROW_CRITICAL_ZOOM) {
        WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_THROW_CRITICAL_ZOOM);
    }
    if special_zoom_gfx > 0 {
        WorkModule::inc_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    }
    if special_zoom_gfx == 2 {
        SlowModule::set_whole(fighter.module_accessor, 8, 80);
        macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 2.0,/*no*/ 0.0,/*zoom*/ 1.8,/*yrot*/ 0.0,/*xrot*/ 0.0);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), Hash40::new("top"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
        macros::PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_XL);
    }
    if special_zoom_gfx >= 4 {
        SlowModule::clear_whole(fighter.module_accessor);
        CameraModule::reset_all(fighter.module_accessor);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
        macros::CAM_ZOOM_OUT(fighter);
    }
    if situation_kind != *SITUATION_KIND_AIR {
        if MotionModule::is_end(fighter.module_accessor) {
            if situation_kind == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
        }
    }
    else {
        let is_catch = {fighter.clear_lua_stack(); lua_args!(fighter, *MA_MSC_CMD_CATCH_IS_CATCH); sv_module_access::_catch(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
        if is_catch {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_JUMP.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn gaogaen_throw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_THROW_CRITICAL_ZOOM);
    0.into()
}

unsafe extern "C" fn gaogaen_special_n_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn gaogaen_special_n_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    0.into()
}

unsafe extern "C" fn gaogaen_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_INT_BATTLE_OBJECT_ID_SWING_THROWN_FIGHTER);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_INVALID_SPECIAL_AIR_S);
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    fun_710001d4b0(fighter, true.into());
    if !StopModule::is_stop(fighter.module_accessor) {
        fun_7100023730(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(fun_7100023730 as *const () as _));
    GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_GAOGAEN_CLIFF_HANG_DATA_SPECIAL_S as u32);
    fighter.sub_shift_status_main(L2CValue::Ptr(gaogaen_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn fun_710001d4b0(fighter: &mut L2CFighterCommon, bool_check: L2CValue) {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let status_kind_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let get_sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let get_sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let special_air_s_catch_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_air_s_catch_accel_y"));
    let special_s_start_speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_start_speed_mul"));
    let special_air_s_start_speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_air_s_start_speed_mul"));
    let special_s_catch_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_catch_speed_x_mul"));
    let special_s_catch_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_catch_brake_x"));
    let special_air_s_catch_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_air_s_catch_speed_x_mul"));
    let special_air_s_catch_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_air_s_catch_brake_x"));
    let special_air_s_catch_speed_y_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_air_s_catch_speed_y_max"));
    if status_kind_interrupt != *FIGHTER_STATUS_KIND_SPECIAL_S {
        if status_kind_interrupt != *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_S_THROW {
            if status_kind_interrupt != *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_S_SHOULDER {
                if status_kind_interrupt != *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_S_LARIAT {
                    if status_kind_interrupt != *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_S_FAILURE {
                        if status_kind_interrupt == *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_S_END {
                            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, module_accessor);
                            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, module_accessor);
                            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, module_accessor);
                            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, module_accessor);
                        }
                    }
                    else {
                        if bool_check.get_bool() {
                            if situation_kind != *SITUATION_KIND_GROUND {
                                sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, *ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
                            }
                            else {
                                sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, *ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
                            }
                            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
                        }
                        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, module_accessor);
                        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, module_accessor);
                        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, module_accessor);
                    }
                }
                else {
                    if situation_kind != *SITUATION_KIND_GROUND {
                        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, *ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
                    }
                    else {
                        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, *ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
                    }
                    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
                    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, module_accessor);
                    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, module_accessor);
                    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, module_accessor);
                }
            }
            else {
                if situation_kind != *SITUATION_KIND_GROUND {
                    sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, *ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
                }
                else {
                    sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, *ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
                }
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, module_accessor);
                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, module_accessor);
                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, module_accessor);
            }
        }
        else {
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, module_accessor);
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, module_accessor);
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, module_accessor);
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, module_accessor);
            if bool_check.get_bool() {
                if situation_kind != *SITUATION_KIND_AIR {
                    sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
                    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, get_sum_speed_x*special_air_s_catch_speed_x_mul, get_sum_speed_y*special_air_s_catch_speed_y_max);
                    sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_air_s_catch_brake_x, special_air_s_catch_accel_y);
                }
                else {
                    sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
                    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, get_sum_speed_x*special_s_catch_speed_x_mul, 0.0);
                    sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_s_catch_brake_x, special_air_s_catch_accel_y);
                }
            }
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_MOVE) {
            if bool_check.get_bool() {
                if situation_kind != *SITUATION_KIND_GROUND {
                    sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, *ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
                    sv_kinetic_energy!(set_speed_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, special_air_s_start_speed_mul);
                }
                else {
                    sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, *ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
                    sv_kinetic_energy!(set_speed_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, special_s_start_speed_mul);
                }
            }
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, module_accessor);
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, module_accessor);
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, module_accessor);
        }
        else {
            if bool_check.get_bool() {
                if situation_kind != *SITUATION_KIND_GROUND {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                    sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, get_sum_speed_x, get_sum_speed_y, 0.0, 0.0, 0.0);
                    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
                }
                else {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                }
            }
            else {
                if situation_kind != *SITUATION_KIND_GROUND {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                }
                else {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                }
            }
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, module_accessor);
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, module_accessor);
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, module_accessor);
        }
    }
}

unsafe extern "C" fn fun_7100023730(fighter: &mut L2CFighterCommon, bool_check: L2CValue) -> L2CValue {
    if !bool_check.get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_MOVE_START) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_MOVE_START);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_MOVE);
            fun_710001d4b0(fighter, true.into());
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_MOVE_END) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_MOVE_END);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_MOVE);
            fun_710001d4b0(fighter, true.into());
        }
    }
    0.into()
}

unsafe extern "C" fn gaogaen_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() && fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if prev_situation_kind != *SITUATION_KIND_GROUND {
        if situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_SPECIAL_S_FAKE) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_cancel"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_start"), -1.0, 1.0, 0.0, false, false);
            }
            fun_710001d4b0(fighter, false.into());
        }
    }
    else {
        if prev_situation_kind == *SITUATION_KIND_GROUND {
            if situation_kind != *SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_SPECIAL_S_FAKE) {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_cancel"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_start"), -1.0, 1.0, 0.0, false, false);
                }
                fun_710001d4b0(fighter, false.into());
            }
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_SPECIAL_S_CANCEL) {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_SPECIAL_S_CANCEL);
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_SPECIAL_S_FAKE);
            if situation_kind == *SITUATION_KIND_AIR {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_cancel"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_cancel"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
    fun_710001ff80(fighter);
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn fun_710001ff80(fighter: &mut L2CFighterCommon) {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
    let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let special_s_attack_after_control_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_attack_after_control_speed_x_mul"));
    let special_s_attack_after_control_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_attack_after_control_speed_x_stable"));
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_WORK_ID_FLAG_AIR_CONTROL) {
        if situation_kind == *SITUATION_KIND_AIR {
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, *ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            sv_kinetic_energy!(controller_set_accel_x_mul, fighter, air_accel_x_mul*special_s_attack_after_control_speed_x_mul);
            sv_kinetic_energy!(controller_set_accel_x_add, fighter, air_accel_x_add*special_s_attack_after_control_speed_x_mul);
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable*special_s_attack_after_control_speed_x_stable, 0.0);
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_WORK_ID_FLAG_AIR_CONTROL);
    }
}

unsafe extern "C" fn gaogaen_special_s_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_SPECIAL_S_CANCEL);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_SPECIAL_S_FAKE);
    0.into()
}

unsafe extern "C" fn gaogaen_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

pub fn install() {
    Agent::new("gaogaen")
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_LW4, gaogaen_attack_lw4_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4, gaogaen_attack_lw4_main_status)
    .status(MapCorrection, *FIGHTER_STATUS_KIND_ATTACK_LW4, gaogaen_attack_lw4_map_correction_status)
    .status(Main, *FIGHTER_STATUS_KIND_CATCH, gaogaen_catch_main_status)
    .status(Main, *FIGHTER_STATUS_KIND_CATCH_DASH, gaogaen_catch_dash_main_status)
    .status(Main, *FIGHTER_STATUS_KIND_CATCH_TURN, gaogaen_catch_turn_main_status)
    .status(End, *FIGHTER_STATUS_KIND_CATCH_PULL, gaogaen_catch_pull_end_status)
    .status(End, *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, gaogaen_catch_dash_pull_end_status)
    .status(End, *FIGHTER_STATUS_KIND_CATCH_WAIT, gaogaen_catch_wait_end_status)
    .status(End, *FIGHTER_STATUS_KIND_CATCH_ATTACK, gaogaen_catch_attack_end_status)
    .status(Main, *FIGHTER_STATUS_KIND_THROW, gaogaen_throw_main_status)
    .status(End, *FIGHTER_STATUS_KIND_THROW, gaogaen_throw_end_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, gaogaen_special_n_pre_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, gaogaen_special_n_exec_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, gaogaen_special_s_main_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, gaogaen_special_s_end_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, gaogaen_special_lw_pre_status)
    .install()
    ;
}