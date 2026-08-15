use super::*;

unsafe extern "C" fn mariod_special_n_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    if ArticleModule::is_exist(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE) {
        let article_boma = get_article_boma(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE);
        let capsule_life = WorkModule::get_int(article_boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        if capsule_life <= 0 {
            UiManager::set_mariod_meter_info(entry_id, 0);
            WorkModule::set_int(boma, UiManager::get_mariod_pill_id(entry_id), *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
        }
    }
    0.into()
}

unsafe extern "C" fn mariod_special_n_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    if ArticleModule::is_exist(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE) {
        let article_boma = get_article_boma(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE);
        let capsule_life = WorkModule::get_int(article_boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        if capsule_life <= 0 {
            UiManager::set_mariod_meter_info(entry_id, 0);
            WorkModule::set_int(boma, UiManager::get_mariod_pill_id(entry_id), *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("mariod")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, mariod_special_n_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_N, mariod_special_n_exit_status)
    .install()
    ;
}