use super::*;

pub unsafe extern "C" fn krool_var(boma: *mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP);
    WorkModule::set_float(boma, 0.0, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_CHARGE);
    WorkModule::set_float(boma, 0.5, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_FUEL);
    WorkModule::set_int(boma, 0, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_LW_ANGLE);
    WorkModule::set_int(boma, 0, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_LW_TIMER);
}

pub unsafe extern "C" fn fun_710001dba0(fighter: &mut L2CFighterCommon, ground_mot: L2CValue, air_mot: L2CValue, inherit: L2CValue) {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if inherit.get_bool() {
            MotionModule::change_motion_inherit_frame(boma, air_mot.get_hash(), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(boma, air_mot.get_hash(), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    else {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        if inherit.get_bool() {
            MotionModule::change_motion_inherit_frame(boma, ground_mot.get_hash(), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(boma, ground_mot.get_hash(), 0.0, 1.0, false, 0.0, false, false);
        }
    }
}

pub unsafe extern "C" fn fun_710001ea30(fighter: &mut L2CFighterCommon) {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let status_kind_interrupt= fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let mut get_sum_speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut get_sum_speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let special_hi_start_mul_spd_x = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_start_mul_spd_x"));
    let special_hi_start_air_mul_spd_x = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_start_air_mul_spd_x"));
    let special_hi_start_air_mul_spd_y = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_start_air_mul_spd_y"));
    let special_hi_fly_start_max_spd_y = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_fly_start_max_spd_y"));
    let special_hi_fly_acl_y = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_fly_acl_y"));
    let special_hi_fly_max_spd_y = if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE) {
        WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_fly_max_spd_y"))/4.0
    }
    else {
        WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_fly_max_spd_y"))
    };
    let special_hi_max_sum_spd_y = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_max_sum_spd_y"));
    let special_hi_fall_acl_y = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_fall_acl_y"));
    let special_hi_fall_max_spd_y = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_fall_max_spd_y"));
    let special_hi_fall_max_spd_x = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_fall_max_spd_x"));
    if status_kind_interrupt != *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_START {
        if status_kind_interrupt != *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI {
            if status_kind_interrupt != *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_AIR_END {
                smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, module_accessor);
                sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, get_sum_speed_y);
                sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -special_hi_fall_acl_y);
                sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
                sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, special_hi_fall_max_spd_y);
                KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, module_accessor);
                sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, get_sum_speed_x, 0.0);
                sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
                sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
                sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
                sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
                KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            }
            else {
                smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, module_accessor);
                if special_hi_max_sum_spd_y <= get_sum_speed_y {
                    get_sum_speed_y = special_hi_max_sum_spd_y;
                }
                sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, get_sum_speed_y);
                sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -special_hi_fall_acl_y);
                sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
                sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, special_hi_fall_max_spd_y);
                KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, module_accessor);
                let clamp = fighter.clamp(get_sum_speed_x.into(), (-special_hi_fall_max_spd_x).into(), special_hi_fall_max_spd_x.into());
                get_sum_speed_x = clamp.get_f32();
                sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, get_sum_speed_x, 0.0);
                sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
                sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
                sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
                sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
                KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            }
        }
        else {
            smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, module_accessor);
            if special_hi_fly_max_spd_y < get_sum_speed_y {
                get_sum_speed_y = special_hi_fly_max_spd_y;
            }
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, get_sum_speed_y);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -special_hi_fly_acl_y);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, special_hi_fly_max_spd_y);
            KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, module_accessor);
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
            sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
            KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
    }
    else {
        smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, module_accessor);
        get_sum_speed_y = get_sum_speed_y*special_hi_start_air_mul_spd_y;
        if get_sum_speed_y < -special_hi_fly_start_max_spd_y {
            get_sum_speed_y = -special_hi_fly_start_max_spd_y;
        }
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, get_sum_speed_y);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, special_hi_fly_start_max_spd_y);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, module_accessor);
        if situation_kind != *SITUATION_KIND_GROUND {
            get_sum_speed_x = get_sum_speed_x*special_hi_start_air_mul_spd_x;
        }
        else {
            get_sum_speed_x = get_sum_speed_x*special_hi_start_mul_spd_x;
        }
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, get_sum_speed_x, 0.0);
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
}