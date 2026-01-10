use super::*;

unsafe extern "C" fn cloud_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn cloud_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
        fighter.sub_change_motion_by_situation(L2CValue::Hash40s("punish_special_lw"), L2CValue::Hash40s("punish_special_air_lw"), false.into());
    }
    else {
        fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw_start"), L2CValue::Hash40s("special_air_lw_start"), false.into());
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(cloud_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn cloud_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
            }
            else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_start"), -1.0, 1.0, 0.0, false, false);
            }
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("punish_special_air_lw"), -1.0, 1.0, 0.0, false, false);
            }
            else {
               MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_start"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
    if cloud_can_limit_break(fighter, 3).get_bool() {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SPECIAL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SET_CUSTOM);
        fighter.change_status(FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_LIMIT_BREAK.into(), false.into());
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

unsafe extern "C" fn cloud_special_lw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_INPUT_WAIT_TIMER);
    0.into()
}

unsafe extern "C" fn cloud_special_lw_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind != *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_LIMIT_BREAK {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE);
        }
        else {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE);
        }
    }
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PARAM_CHANGE);
    0.into()
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, cloud_special_lw_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, cloud_special_lw_main_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, cloud_special_lw_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_LW, cloud_special_lw_exit_status)
    .install()
    ;
}