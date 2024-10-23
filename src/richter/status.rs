use super::*;

unsafe extern "C" fn richter_landing_attack_air_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_LandingAttackAirSub();
    fighter.sub_shift_status_main(L2CValue::Ptr(richter_landing_attack_air_main_loop as *const () as _))
}

unsafe extern "C" fn richter_landing_attack_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("landing_air_lw")
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW3.into(), true.into());
        return 1.into();
    }
    fighter.status_LandingAttackAir_Main()
}

unsafe extern "C" fn richter_special_n_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn richter_special_n_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
    }
    0.into()
}

unsafe extern "C" fn richter_special_n_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s1"), L2CValue::Hash40s("special_air_s1"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(richter_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn richter_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if prev_situation_kind == *SITUATION_KIND_GROUND && situation_kind == *SITUATION_KIND_AIR {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s1"), -1.0, 1.0, 0.0, false, false);
        }
        if prev_situation_kind == *SITUATION_KIND_AIR && situation_kind == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s1"), -1.0, 1.0, 0.0, false, false);
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

unsafe extern "C" fn richter_special_n_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn richter_special_n_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn richter_special_n_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn richter_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn richter_special_s_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    0.into()
}

unsafe extern "C" fn richter_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s"), L2CValue::Hash40s("special_air_s"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(richter_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn richter_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if prev_situation_kind == *SITUATION_KIND_GROUND && situation_kind == *SITUATION_KIND_AIR {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s"), -1.0, 1.0, 0.0, false, false);
        }
        if prev_situation_kind == *SITUATION_KIND_AIR && situation_kind == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s"), -1.0, 1.0, 0.0, false, false);
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

unsafe extern "C" fn richter_special_s_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn richter_special_s_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn richter_special_s_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn richter_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_SOUND);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

pub fn install() {
    Agent::new("richter")
    .status(Main, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, richter_landing_attack_air_main_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, richter_special_n_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, richter_special_n_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, richter_special_n_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, richter_special_n_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, richter_special_n_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_N, richter_special_n_exit_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, richter_special_s_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, richter_special_s_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, richter_special_s_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, richter_special_s_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, richter_special_s_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, richter_special_s_exit_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, richter_special_lw_pre_status)
    .install()
    ;
}