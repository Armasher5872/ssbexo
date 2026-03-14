use super::*;

unsafe extern "C" fn luigi_special_lw_loop_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_CATCH as u64, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn luigi_special_lw_loop_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    0.into()
}

unsafe extern "C" fn luigi_special_lw_loop_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    PLAY_STATUS(fighter, Hash40::new("se_luigi_final02"));
    ArticleModule::change_status(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_SPECIAL_LW_LOOP, ArticleOperationTarget(0));
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_loop"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_special_lw_loop_main_loop as *const () as _))
}

unsafe extern "C" fn luigi_special_lw_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let special_lw_timer = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_LW_TIMER);
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }
    }
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_LW_TIMER);
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_PLUNGER.into(), false.into());
    }
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) || special_lw_timer > 180 {
        fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_END.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_loop"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}

unsafe extern "C" fn luigi_special_lw_loop_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if [*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_PULL, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_PLUNGER, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_END].contains(&status_kind) {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_LW_TIMER);
    }
    else {
        let object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_OBAKYUMU_OBJECT_ID);
        ArticleModule::remove_exist_object_id(fighter.module_accessor, object_id as u32);
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    0.into()
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_LOOP, luigi_special_lw_loop_pre_status)
    .status(Init, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_LOOP, luigi_special_lw_loop_init_status)
    .status(Main, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_LOOP, luigi_special_lw_loop_main_status)
    .status(End, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_LOOP, luigi_special_lw_loop_end_status)
    .install()
    ;
}