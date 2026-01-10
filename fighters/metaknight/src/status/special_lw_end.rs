use super::*;

//Down Special End Pre Status
unsafe extern "C" fn metaknight_special_lw_end_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

//Down Special End Init Status
unsafe extern "C" fn metaknight_special_lw_end_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    0.into()
}

//Down Special End Main Status
unsafe extern "C" fn metaknight_special_lw_end_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, false, -1);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw_end"), L2CValue::Hash40s("special_air_lw_end"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_special_lw_end_main_loop as *const () as _))
}

unsafe extern "C" fn metaknight_special_lw_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) && fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_end"), -1.0, 1.0, 0.0, false, false);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_end"), -1.0, 1.0, 0.0, false, false);
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

//Down Special End Exec Status
unsafe extern "C" fn metaknight_special_lw_end_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Down Special End End Status
unsafe extern "C" fn metaknight_special_lw_end_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill(fighter.module_accessor, WorkModule::get_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_INT_SPECIAL_LW_EFFECT_HANDLE) as u32, true, true);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_POWER);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_INT_SPECIAL_LW_EFFECT_HANDLE);
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, ArticleOperationTarget(0));
    }
    0.into()
}

//Down Special End Exit Status
unsafe extern "C" fn metaknight_special_lw_end_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill(fighter.module_accessor, WorkModule::get_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_INT_SPECIAL_LW_EFFECT_HANDLE) as u32, true, true);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_POWER);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_INT_SPECIAL_LW_EFFECT_HANDLE);
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, ArticleOperationTarget(0));
    }
    0.into()
}

pub fn install() {
    Agent::new("metaknight")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_END, metaknight_special_lw_end_pre_status)
    .status(Init, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_END, metaknight_special_lw_end_init_status)
    .status(Main, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_END, metaknight_special_lw_end_main_status)
    .status(Exec, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_END, metaknight_special_lw_end_exec_status)
    .status(End, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_END, metaknight_special_lw_end_end_status)
    .status(Exit, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_END, metaknight_special_lw_end_exit_status)
    .install()
    ;
}