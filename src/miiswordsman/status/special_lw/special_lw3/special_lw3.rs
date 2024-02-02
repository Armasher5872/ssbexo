use super::*;

pub unsafe extern "C" fn miiswordsman_special_lw3_start_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_FALL, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_lw3_start_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_SP_BRAKE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_CLIFF_CHECK);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_WALL_CHECK);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_FALL_ONOFF);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_LANDING_FALL_SPECIAL);
    let lw3_hit_reduct_count = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("lw3_hit_reduct_count"));
    WorkModule::set_int(fighter.module_accessor, lw3_hit_reduct_count, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_INT_JET_STUB_REDUCTION_LEFT);
    WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLOAT_JET_STUB_SPEED_COEFFICIENT);
    if situation_kind != *SITUATION_KIND_GROUND {
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_INT_JET_STUB_START_SITUATION);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_INT_JET_STUB_START_SITUATION);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_12 - 1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_12 - 1);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw3"), L2CValue::Hash40s("special_air_lw3"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(miiswordsman_special_lw3_start_main_loop as *const () as _))
}

unsafe extern "C" fn miiswordsman_special_lw3_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let start_situation = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_INT_JET_STUB_START_SITUATION);
    if CancelModule::is_enable_cancel(fighter.module_accessor) && (!fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool()) {
        return 1.into();
    }
    if start_situation == *SITUATION_KIND_GROUND {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_CLIFF_CHECK) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_CLIFF_CHECK);
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW3_END.into(), false.into());
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_lw3"), -1.0, 1.0, 0.0);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW3_END.into(), false.into());
    }
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_lw3_start_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}