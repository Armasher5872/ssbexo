use super::*;

//Throw End Status
unsafe extern "C" fn mario_throw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_HAMMER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    fighter.status_end_Throw()
}

//Throw Exit Status
unsafe extern "C" fn mario_throw_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_HAMMER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    fighter.sub_throw_uniq_process_exit();
    0.into()
}

pub fn install() {
    Agent::new("mario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_STATUS_KIND_THROW, mario_throw_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_THROW, mario_throw_exit_status)
    .install()
    ;
}