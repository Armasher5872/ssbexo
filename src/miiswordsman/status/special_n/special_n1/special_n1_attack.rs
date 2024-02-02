use super::*;

pub unsafe extern "C" fn miiswordsman_special_n1_attack_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0 as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_n1_attack_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let power = WorkModule::get_float(fighter.module_accessor, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLOAT_GUARD_BREAKER_POWER);
    match stick_y {
        _ if stick_y >= 0.5 => {
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 1.5);
        }
        _ if stick_y <= -0.5 => {
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -1.5);
        }
        _ => {}
    }
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, (3.0*power)*lr);
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_n1_attack_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    match stick_y {
        _ if stick_y >= 0.5 => {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n1_attack_hi"), L2CValue::Hash40s("special_air_n1_attack_hi"), false.into());
            WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_DIRECTION);
        }
        _ if stick_y <= -0.5 => {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n1_attack_lw"), L2CValue::Hash40s("special_air_n1_attack_lw"), false.into());
            WorkModule::set_int(fighter.module_accessor, 2, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_DIRECTION);
        }
        _ => {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n1_attack"), L2CValue::Hash40s("special_air_n1_attack"), false.into());
            WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_DIRECTION);
        }
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(miiswordsman_special_n1_attack_main_loop as *const () as _))
}

unsafe extern "C" fn miiswordsman_special_n1_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let guard_breaker_direction = WorkModule::get_int(fighter.module_accessor, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_DIRECTION);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        match guard_breaker_direction {
            0 => {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n1_attack"), -1.0, 1.0, 0.0, false, false);
            }
            1 => {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n1_attack_hi"), -1.0, 1.0, 0.0, false, false);
            }
            2 => {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n1_attack_lw"), -1.0, 1.0, 0.0, false, false);
            }
            _ => {}
        }
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        match guard_breaker_direction {
            0 => {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n1_attack"), -1.0, 1.0, 0.0, false, false);
            }
            1 => {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n1_attack_hi"), -1.0, 1.0, 0.0, false, false);
            }
            2 => {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n1_attack_lw"), -1.0, 1.0, 0.0, false, false);
            }
            _ => {}
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_n1_attack_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_n1_attack_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 1.0, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLOAT_GUARD_BREAKER_POWER);
    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_DIRECTION);
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_n1_attack_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}