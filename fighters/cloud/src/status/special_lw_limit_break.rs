use super::*;

unsafe extern "C" fn cloud_special_lw_lb_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, (*FIGHTER_STATUS_ATTR_START_TURN | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE) as u32, 0);
    0.into()
}

unsafe extern "C" fn cloud_special_lw_lb_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let lw2_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("lw2_accel_y"));
    let lw2_speed_max_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("lw2_speed_max_y"));
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -lw2_accel_y);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, lw2_speed_max_y);
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.05);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    0.into()
}

unsafe extern "C" fn cloud_special_lw_lb_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw"), L2CValue::Hash40s("special_air_lw"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(cloud_special_lw_lb_main_loop as *const () as _))
}

unsafe extern "C" fn cloud_special_lw_lb_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
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
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0, false, false);
        }
        if situation_kind == *SITUATION_KIND_AIR 
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0, false, false);
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

unsafe extern "C" fn cloud_special_lw_lb_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn cloud_special_lw_lb_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SPECIAL);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SET_CUSTOM);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_INPUT_WAIT_TIMER);
    display_final_window(false);
    0.into()
}

unsafe extern "C" fn cloud_special_lw_lb_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let limit_level = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_LEVEL);
    WorkModule::set_int(fighter.module_accessor, limit_level-3, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_LEVEL);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SPECIAL);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SET_CUSTOM);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_INPUT_WAIT_TIMER);
    display_final_window(false);
    0.into()
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_LIMIT_BREAK, cloud_special_lw_lb_pre_status)
    .status(Init, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_LIMIT_BREAK, cloud_special_lw_lb_init_status)
    .status(Main, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_LIMIT_BREAK, cloud_special_lw_lb_main_status)
    .status(Exec, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_LIMIT_BREAK, cloud_special_lw_lb_exec_status)
    .status(End, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_LIMIT_BREAK, cloud_special_lw_lb_end_status)
    .status(Exit, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_LIMIT_BREAK, cloud_special_lw_lb_exit_status)
    .install()
    ;
}