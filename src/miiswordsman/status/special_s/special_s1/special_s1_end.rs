use super::*;

pub unsafe extern "C" fn miiswordsman_special_s1_end_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, *FS_SUCCEEDS_KEEP_SOUND);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_s1_end_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let get_sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let s1_air_end_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi") as u64, hash40("s1_air_end_speed_x_mul") as u64);
    let s1_air_end_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi") as u64, hash40("s1_air_end_brake_x") as u64);
    let s1_air_end_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi") as u64, hash40("s1_air_end_accel_y") as u64);
    let s1_control_limit_mul_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi") as u64, hash40("s1_control_limit_mul_x") as u64);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, s1_air_end_brake_x, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -s1_air_end_accel_y);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, *ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(mul_x_speed_max, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, s1_control_limit_mul_x);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    else {
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, s1_air_end_speed_x_mul*get_sum_speed_x, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_s1_end_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s1_end"), L2CValue::Hash40s("special_air_s1_end"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(miiswordsman_special_s1_end_main_loop as *const () as _))
}

unsafe extern "C" fn miiswordsman_special_s1_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s1_end"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.set_situation(SITUATION_KIND_AIR.into());
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_HENSOKU_SLASH_WORK_FLAG_END_LANDING)
    && prev_situation_kind != *SITUATION_KIND_GROUND
    && situation_kind == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 0.into();
    }
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_s1_end_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT);
    }
    0.into()
}