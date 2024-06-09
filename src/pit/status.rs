use super::*;

unsafe extern "C" fn pit_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT_CHECK_ONOFF);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_CLIFF_FALL_ONOFF);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_GRAVITY_ONOFF);
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    fun_710001a140(fighter, false);
    if !StopModule::is_stop(fighter.module_accessor) {
        fun_710001a5c0(fighter, false);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(fun_710001a5c0 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(pit_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn fun_710001a140(fighter: &mut L2CFighterCommon, param_2: bool) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_INT_START_SITUATION);
        if param_2 {          
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_start"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    }
    else {
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_INT_START_SITUATION);
        if param_2 {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_start"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    0.into()
}

unsafe extern "C" fn fun_710001a5c0(fighter: &mut L2CFighterCommon, param_2: bool) -> L2CValue {
    if !param_2 {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_INT_START_SITUATION) != *SITUATION_KIND_AIR {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_CLIFF_FALL_ONOFF) {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }
            else {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            }
        }
        else {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_GRAVITY_ONOFF) {
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
            else {
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn pit_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_INT_START_SITUATION) == *SITUATION_KIND_GROUND {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_MTRANS_AIR_UNABLE) {
                fun_710001a140(fighter, true);
            }
        }
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_APPLIED_POWERUP) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_MOVE_FRONT) {
            let speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s") as u64, hash40("speed_mul") as u64);
            sv_kinetic_energy!(set_speed_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, speed_mul);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_APPLIED_POWERUP);
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT_CHECK_ONOFF) && fighter.global_table[CURRENT_FRAME].get_f32() >= 36.0 {
        fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        return 0.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT) {
        fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        return 0.into();
    }
    0.into()
}

pub fn install() {
    Agent::new("pit")
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, pit_special_s_main_status)
    .install()
    ;
}