use super::*;

unsafe extern "C" fn luigi_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_AIR_LASSO | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn luigi_special_lw_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let global_fighter = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
    delete_plunger(global_fighter, false);
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    0.into()
}

unsafe extern "C" fn luigi_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw"), L2CValue::Hash40s("special_air_lw"), false.into());
    if lr == -1.0 {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("special_lw_l"), false, -1.0);
    }
    else {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("special_lw"), false, -1.0);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_special_lw_loop as *const () as _))
}

unsafe extern "C" fn luigi_special_lw_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
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
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0, false, false);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            fighter.change_status(FIGHTER_STATUS_KIND_AIR_LASSO.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_LOOP.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn luigi_special_lw_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_lw_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_lw_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("luigi")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, luigi_special_lw_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, luigi_special_lw_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, luigi_special_lw_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_LW, luigi_special_lw_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, luigi_special_lw_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_LW, luigi_special_lw_exit_status)
    .install()
    ;
}