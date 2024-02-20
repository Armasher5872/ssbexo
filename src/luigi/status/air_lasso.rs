use super::*;

unsafe extern "C" fn luigi_air_lasso_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_FALL, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, true, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_AIR_LASSO | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, (*FIGHTER_STATUS_ATTR_CLEAR_MOTION_ENERGY | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE) as u32, *FIGHTER_POWER_UP_ATTACK_BIT_CATCH as u32, 0);
    0.into()
}

unsafe extern "C" fn luigi_air_lasso_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AirLasso(ARTICLE_ID_NONE.into(), ARTICLE_ID_NONE.into(), FIGHTER_CLIFF_HANG_DATA_DEFAULT.into(), true.into());
    0.into()
}

unsafe extern "C" fn luigi_air_lasso_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    fighter.status_end_AirLasso();
    if ![*FIGHTER_STATUS_KIND_AIR_LASSO_HANG, *FIGHTER_STATUS_KIND_AIR_LASSO_REACH, *FIGHTER_STATUS_KIND_AIR_LASSO_LANDING].contains(&status_kind) {
        let global_fighter = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
        let object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_OBAKYUMU_OBJECT_ID);
        ArticleModule::remove_exist_object_id(fighter.module_accessor, object_id as u32);
        delete_plunger(global_fighter, false);
    }
    0.into()
}

pub fn install() {
    Agent::new("luigi")
    .status(Pre, *FIGHTER_STATUS_KIND_AIR_LASSO, luigi_air_lasso_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_AIR_LASSO, luigi_air_lasso_main_status)
    .status(End, *FIGHTER_STATUS_KIND_AIR_LASSO, luigi_air_lasso_end_status)
    .install()
    ;
}