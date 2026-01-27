use super::*;

unsafe extern "C" fn gaogaen_special_n_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn gaogaen_special_n_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Vanilla Status aside from the grounded versions correct kind
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_GRAVITY_DEFAULT);
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
    }
    fun_7100023ef0(fighter, true.into());
    if !StopModule::is_stop(fighter.module_accessor) {
        fun_71000253c0(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(fun_71000253c0 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(gaogaen_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn fun_7100023ef0(fighter: &mut L2CFighterCommon, bool_check: L2CValue) {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let get_sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let get_sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let special_air_n_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("special_air_n_speed_y"));
    let special_air_n_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("special_air_n_accel_y"));
    let special_air_n_speed_y_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("special_air_n_speed_y_max"));
    let special_n_speed_x_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_speed_x_max"));
    let special_air_n_speed_x_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("special_air_n_speed_x_max"));
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_ROTATION) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_UNIQ);
            smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            if situation_kind != *SITUATION_KIND_GROUND {
                sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
                if bool_check.get_bool() {
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_INVALID_SPECIAL_AIR_N_SPPED_Y) {
                        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, special_air_n_speed_y);
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_INVALID_SPECIAL_AIR_N_SPPED_Y);
                    }
                }
                else {
                    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, get_sum_speed_y);
                }
                sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -special_air_n_accel_y);
                sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
                sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, special_air_n_speed_y_max);
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
            else {
                smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fighter.module_accessor);
            }
            smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_FREE, 0.0, 0.0, 0.0, 0.0, 0.0);
            if situation_kind != *SITUATION_KIND_GROUND {
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, get_sum_speed_x, 0.0);
                sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
                sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
                sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_air_n_speed_x_max, 0.0);
                sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
            }
            else {
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, get_sum_speed_x, 0.0);
                sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
                sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
                sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_n_speed_x_max, 0.0);
                sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
            }
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
        else {
            if bool_check.get_bool() {
                if situation_kind != *SITUATION_KIND_GROUND {
                    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_GRAVITY_DEFAULT) {
                        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                    }
                    sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, get_sum_speed_x, 0.0, 0.0, 0.0, 0.0);
                    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
                }
                else {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                }
            }
            else {
                if situation_kind == *SITUATION_KIND_GROUND {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                }
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_GRAVITY_DEFAULT) {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                }
            }
            smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
        }
    }
}

unsafe extern "C" fn fun_71000253c0(fighter: &mut L2CFighterCommon, bool_check: L2CValue) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32();
    if !bool_check.get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_START_ROTATION) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_START_ROTATION);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_ROTATION);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_GRAVITY_DEFAULT);
            fun_7100023ef0(fighter, true.into());
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_END_ROTATION) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_END_ROTATION);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_ROTATION);
            fun_7100023ef0(fighter, true.into());
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_REQUEST_GRAVITY_DEFAULT) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_REQUEST_GRAVITY_DEFAULT);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_GRAVITY_DEFAULT);
            if situation_kind != *SITUATION_KIND_GROUND {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_ROTATION) {
            let special_n_stick_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_stick_accel_x"));
            let special_air_n_stick_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("special_air_n_stick_accel_x"));
            let special_n_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_brake_x"));
            let special_air_n_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("special_air_n_brake_x"));
            if -0.1 > stick_x {
                let mut x_accel = special_n_stick_accel_x;
                if situation_kind != *SITUATION_KIND_GROUND {
                    x_accel = special_air_n_stick_accel_x;
                }
                if stick_x < -0.1 {
                    x_accel = -x_accel;
                }
                sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, x_accel, 0.0);
                sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
            }
            else {
                if stick_x <= 0.1 {
                    let x_brake;
                    if situation_kind != *SITUATION_KIND_GROUND {
                        x_brake = special_air_n_brake_x;
                    }
                    else {
                        x_brake = special_n_brake_x;
                    }
                    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
                    sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, x_brake, 0.0);
                }
            }
        }
    }
    0.into()
}

unsafe extern "C" fn gaogaen_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Vanilla Status aside from the grounded versions correct kind
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if prev_situation_kind != *SITUATION_KIND_GROUND {
        if situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n"), -1.0, 1.0, 0.0, false, false);
            fun_7100023ef0(fighter, false.into());
        }
    }
    if prev_situation_kind == *SITUATION_KIND_GROUND {
        if situation_kind != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n"), -1.0, 1.0, 0.0, false, false);
            fun_7100023ef0(fighter, false.into());
        }
    }
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

pub fn install() {
    Agent::new("gaogaen")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, gaogaen_special_n_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, gaogaen_special_n_main_status)
    .install()
    ;
}