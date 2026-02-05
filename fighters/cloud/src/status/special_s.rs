use super::*;

unsafe extern "C" fn cloud_special_s_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
            KineticModule::clear_speed_all(fighter.module_accessor);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.4);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.4);
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    0.into()
}

unsafe extern "C" fn cloud_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
        fighter.sub_change_motion_by_situation(L2CValue::Hash40s("punish_special_s"), L2CValue::Hash40s("punish_special_air_s"), false.into());
    }
    else {
        fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s"), L2CValue::Hash40s("special_air_s"), false.into());
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(cloud_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn cloud_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let is_punisher = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE);
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("air_speed_x_stable"));
    let get_sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND 
        && prev_situation_kind == *SITUATION_KIND_AIR {
            if current_frame > 10.0 {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
                fighter.change_status(FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S_LANDING.into(), false.into());
            }
        }
        if situation_kind == *SITUATION_KIND_AIR 
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            if is_punisher {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("punish_special_air_s"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
    if current_frame == 6.0 {
        if situation_kind != *SITUATION_KIND_GROUND {
            if !is_punisher {
                sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
                sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 2.9);
            }
        }
    }
    if current_frame == 10.0 {
        if situation_kind != *SITUATION_KIND_GROUND {
            if is_punisher {
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE) {
                    sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
                    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, get_sum_speed_x*0.2);
                    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 3.0);
                    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.07);
                    sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.01);
                    sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.03);
                    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable*0.5);
                }
            }
        }
    }
    if is_punisher {
        if cloud_can_limit_break(fighter, 1).get_bool() && current_frame < 10.0 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SPECIAL);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SET_CUSTOM);
            UiManager::set_limit_type(entry_id, 1);
            fighter.change_status(FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S_LIMIT_BREAK.into(), false.into());
        }
    }
    else {
        if situation_kind != *SITUATION_KIND_GROUND {
            if cloud_can_limit_break(fighter, 1).get_bool() && current_frame < 13.0 {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SPECIAL);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SET_CUSTOM);
                UiManager::set_limit_type(entry_id, 1);
                fighter.change_status(FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S_LIMIT_BREAK.into(), false.into());
            }
        }
        else {
            if cloud_can_limit_break(fighter, 1).get_bool() && current_frame < 15.0 {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SPECIAL);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SET_CUSTOM);
                UiManager::set_limit_type(entry_id, 1);
                fighter.change_status(FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S_LIMIT_BREAK.into(), false.into());
            }
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

unsafe extern "C" fn cloud_special_s_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    }
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_INPUT_WAIT_TIMER);
    0.into()
}

unsafe extern "C" fn cloud_special_s_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind != *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S_LIMIT_BREAK {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
            cloud_set_lightweight(fighter.module_accessor, true);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE);
        }
        else {
            cloud_set_lightweight(fighter.module_accessor, false);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE);
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, cloud_special_s_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, cloud_special_s_main_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, cloud_special_s_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, cloud_special_s_exit_status)
    .install()
    ;
}