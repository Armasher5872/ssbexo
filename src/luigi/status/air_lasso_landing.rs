use super::*;

unsafe extern "C" fn luigi_air_lasso_landing_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND), fighter.sub_pre_landing_kinetic_type().get_i32(), *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_LUIGI_STATUS_WORK_KEEP_FLAG_AIR_LASSO_LANDING_FLAG, *FIGHTER_LUIGI_STATUS_WORK_KEEP_FLAG_AIR_LASSO_LANDING_INT, *FIGHTER_LUIGI_STATUS_WORK_KEEP_FLAG_AIR_LASSO_LANDING_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE as u32, 0, 0);
    0.into()
}

unsafe extern "C" fn luigi_air_lasso_landing_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    fighter.status_LandingSub();
    fighter.status_LandingStiffness();
    if lr == -1.0 {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("special_lw_shoot_l"), false, -1.0);
    }
    else {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("special_lw_shoot"), false, -1.0);
    }
    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_end"), -1.0, 1.0, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_air_lasso_landing_main_loop as *const () as _))
}

unsafe extern "C" fn luigi_air_lasso_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_air_lasso_landing_main()
}

unsafe extern "C" fn luigi_air_lasso_landing_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let global_fighter = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
    let object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_OBAKYUMU_OBJECT_ID);
    fighter.status_end_AirLassoLanding();
    ArticleModule::remove_exist_object_id(fighter.module_accessor, object_id as u32);
    delete_plunger(global_fighter, false);
    0.into()
}

pub fn install() {
    Agent::new("luigi")
    .status(Pre, *FIGHTER_STATUS_KIND_AIR_LASSO_LANDING, luigi_air_lasso_landing_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_AIR_LASSO_LANDING, luigi_air_lasso_landing_main_status)
    .status(End, *FIGHTER_STATUS_KIND_AIR_LASSO_LANDING, luigi_air_lasso_landing_end_status)
    .install()
    ;
}