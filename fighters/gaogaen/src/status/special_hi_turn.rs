use super::*;

unsafe extern "C" fn gaogaen_special_hi_turn_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    if WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CANCELED) {
        MotionModule::change_motion(boma, Hash40::new("special_air_hi_cancel"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(boma, Hash40::new("special_air_hi_turn"), 0.0, 1.0, false, 0.0, false, false);
    }
    GroundModule::set_passable_check(boma, true);
    fun_7100016830(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(gaogaen_special_hi_turn_main_loop as *const () as _))
}

unsafe extern "C" fn fun_7100016830(fighter: &mut L2CFighterCommon) {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let status_kind_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let boma = fighter.module_accessor;
    let lr = PostureModule::lr(boma);
    let get_sum_speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let get_sum_speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let special_air_hi_turn_brake_x = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_air_hi_turn_brake_x"));
    let special_air_hi_turn_accel_y = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_air_hi_turn_accel_y"));
    let special_air_hi_turn_speed_y_max = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_air_hi_turn_speed_y_max"));
    //let special_air_hi_fall_loop_frame = WorkModule::get_param_int(boma, hash40("param_special_hi"), hash40("special_air_hi_fall_loop_frame")); Unused in the vanilla func
    let special_hi_bound_speed_x = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_bound_speed_x"));
    let special_hi_bound_brake_x = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_bound_brake_x"));
    let special_hi_bound_control_x_mul = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_bound_control_x_mul"));
    let special_hi_bound_speed_y = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_bound_speed_y"));
    let special_hi_bound_speed_y_max = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_bound_speed_y_max"));
    let special_air_hi_end_target_speed_y = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_air_hi_end_target_speed_y"));
    let special_air_hi_end_target_speed_brake_frame = WorkModule::get_param_int(boma, hash40("param_special_hi"), hash40("special_air_hi_end_target_speed_brake_frame"));
    let special_air_hi_fall_vector;
    let special_air_hi_fall_speed;
    let special_air_hi_fall_2_vector;
    if WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_HI_FLAG_START_AIR) {
        special_air_hi_fall_vector = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_air_hi_fall_vector_for_air"));
        special_air_hi_fall_speed = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_air_hi_fall_speed_for_air"));
        special_air_hi_fall_2_vector = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_air_hi_fall_2_vector_for_air"));
    }
    else {
        special_air_hi_fall_vector = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_air_hi_fall_vector"));
        special_air_hi_fall_speed = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_air_hi_fall_speed"));
        special_air_hi_fall_2_vector = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_air_hi_fall_2_vector"));
    }
    if status_kind_interrupt == *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_TURN {
        if WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CANCELED) {
            KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 2.02);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.04);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        }
        else {
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, module_accessor);
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, get_sum_speed_y);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -special_air_hi_turn_accel_y);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, special_air_hi_turn_speed_y_max);
            KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, module_accessor);
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_FREE, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, get_sum_speed_x, 0.0);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
            sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_air_hi_turn_brake_x, 0.0);
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
            KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
    }
    if status_kind_interrupt == *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_FALL {
        let mut vector = special_air_hi_fall_vector.to_radians();
        if WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_HI_FLAG_FALL_TYPE_2) {
            vector = special_air_hi_fall_2_vector.to_radians();
        }
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, module_accessor);
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, vector.sin());
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, module_accessor);
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_FREE, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, (vector.cos()*special_air_hi_fall_speed)*lr, 0.0);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    if status_kind_interrupt == *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_LOOP {
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, module_accessor);
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, get_sum_speed_y);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, module_accessor);
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_FREE, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, get_sum_speed_x, 0.0);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    if status_kind_interrupt == *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_END {
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, module_accessor);
        if special_air_hi_end_target_speed_brake_frame > 0 {
            if get_sum_speed_y < 0.0 {
                if get_sum_speed_y < -special_air_hi_end_target_speed_y {
                    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, get_sum_speed_y);
                    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, (get_sum_speed_y+special_air_hi_end_target_speed_y).abs()/(special_air_hi_end_target_speed_brake_frame as f32));
                }
            }
        }
        else {
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -special_air_hi_end_target_speed_y);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        }
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, get_sum_speed_x, 0.0);
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    if status_kind_interrupt == *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_BOUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, *ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, special_hi_bound_speed_y);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, special_hi_bound_speed_y_max);
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, *ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(mul_x_speed_max, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, special_hi_bound_control_x_mul);
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR_BRAKE, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_hi_bound_speed_x*lr, 0.0);
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_hi_bound_brake_x, 0.0);
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
}

unsafe extern "C" fn gaogaen_special_hi_turn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let stick_x = ControlModule::get_stick_x(boma);
    let lr = PostureModule::lr(boma);
    let fall_type_param = WorkModule::get_param_float(boma, hash40("param_special_hi"), 0x284f8d34e0);
    if CancelModule::is_enable_cancel(boma) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_HI_FLAG_FALL_TYPE_CHECK) {
        WorkModule::off_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_HI_FLAG_FALL_TYPE_CHECK);
        if 0.0 >= lr {
            if fall_type_param < stick_x {
                WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_HI_FLAG_FALL_TYPE_2);
            }
        }
        else {
            if stick_x < -fall_type_param {
                WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_HI_FLAG_FALL_TYPE_2);
            }
        }
    }
    fun_710001a8c0(fighter);
    if MotionModule::is_end(boma) {
        if WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CANCELED) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_FALL.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn gaogaen_special_hi_turn_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CAN_CANCEL);
    WorkModule::off_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CANCELED);
    fun_7100016750(fighter);
    0.into()
}

unsafe extern "C" fn fun_7100016750(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_GAOGAEN_STATUS_SPECIAL_HI_INT_FALL_HIT_OBJECT_ID);
    WorkModule::off_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_HI_FLAG_DISABLE_OPPONENT_PASSIVE);
}

pub fn install() {
    Agent::new("gaogaen")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_TURN, gaogaen_special_hi_turn_main_status)
    .status(End, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_TURN, gaogaen_special_hi_turn_end_status)
    .install()
    ;
}