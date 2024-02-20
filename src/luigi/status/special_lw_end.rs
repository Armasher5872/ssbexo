use super::*;

unsafe extern "C" fn luigi_special_lw_end_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn luigi_special_lw_end_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw_end"), L2CValue::Hash40s("special_air_lw_end"), false.into());
    if lr == -1.0 {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("special_lw_end_l"), false, -1.0);
    }
    else {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("special_lw_end"), false, -1.0);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_special_lw_end_loop as *const () as _))
}

unsafe extern "C" fn luigi_special_lw_end_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_end"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_end"), -1.0, 1.0, 0.0, false, false);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn luigi_special_lw_end_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let global_fighter = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
    let object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_OBAKYUMU_OBJECT_ID);
    ArticleModule::remove_exist_object_id(fighter.module_accessor, object_id as u32);
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    delete_plunger(global_fighter, false);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    0.into()
}

pub fn install() {
    Agent::new("luigi")
    .status(Pre, FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_END, luigi_special_lw_end_pre_status)
    .status(Main, FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_END, luigi_special_lw_end_main_status)
    .status(End, FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_END, luigi_special_lw_end_end_status)
    .install()
    ;
}