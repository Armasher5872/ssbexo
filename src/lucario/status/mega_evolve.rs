use super::*;

unsafe extern "C" fn lucario_mega_evolve_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_RESET, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_FINAL | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, (*FIGHTER_STATUS_ATTR_DISABLE_ITEM_INTERRUPT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE | *FIGHTER_STATUS_ATTR_FINAL) as u32, *FIGHTER_POWER_UP_ATTACK_BIT_FINAL as u32, 0);
    0.into()
}

unsafe extern "C" fn lucario_mega_evolve_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    0.into()
}

unsafe extern "C" fn lucario_mega_evolve_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_LUCARIOM, false, -1);
    ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_LUCARIOM, false, ArticleOperationTarget(0));
    ArticleModule::change_status(fighter.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_LUCARIOM, *WEAPON_LUCARIO_LUCARIOM_STATUS_KIND_MEGA_EVOLVE, ArticleOperationTarget(0));
    if situation_kind == *SITUATION_KIND_AIR {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_LUCARIOM, Hash40::new("mega_evolve_air"), false, -1.0);
    }
    else {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_LUCARIOM, Hash40::new("mega_evolve"), false, -1.0);
    }
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("mega_evolve"), L2CValue::Hash40s("mega_evolve_air"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(lucario_mega_evolve_main_loop as *const () as _))
}

unsafe extern "C" fn lucario_mega_evolve_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR && prev_situation_kind == *SITUATION_KIND_GROUND {
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("mega_evolve_air"), -1.0, 1.0, 0.0);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        if situation_kind == *SITUATION_KIND_GROUND && prev_situation_kind == *SITUATION_KIND_AIR {
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("mega_evolve"), -1.0, 1.0, 0.0);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }   
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn lucario_mega_evolve_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucario_mega_evolve_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    macros::CANCEL_FILL_SCREEN(fighter, 0, 10);
    0.into()
}

unsafe extern "C" fn lucario_mega_evolve_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    macros::CANCEL_FILL_SCREEN(fighter, 0, 10);
    0.into()
}

pub fn install() {
    Agent::new("lucario")
    .status(Pre, *FIGHTER_LUCARIO_STATUS_KIND_MEGA_EVOLVE, lucario_mega_evolve_pre_status)
    .status(Init, *FIGHTER_LUCARIO_STATUS_KIND_MEGA_EVOLVE, lucario_mega_evolve_init_status)
    .status(Main, *FIGHTER_LUCARIO_STATUS_KIND_MEGA_EVOLVE, lucario_mega_evolve_main_status)
    .status(Exec, *FIGHTER_LUCARIO_STATUS_KIND_MEGA_EVOLVE, lucario_mega_evolve_exec_status)
    .status(End, *FIGHTER_LUCARIO_STATUS_KIND_MEGA_EVOLVE, lucario_mega_evolve_end_status)
    .status(Exit, *FIGHTER_LUCARIO_STATUS_KIND_MEGA_EVOLVE, lucario_mega_evolve_exit_status)
    .install()
    ;
}