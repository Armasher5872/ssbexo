use super::*;

pub unsafe extern "C" fn sheik_remove_needles(boma: *mut BattleObjectModuleAccessor) {
    WorkModule::dec_int(boma, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_NEEDLE_COUNT);
    WorkModule::off_flag(boma, *FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_CHARGE_MAX);
    EffectModule::remove_common(boma, Hash40::new("charge_max"));
    ArticleModule::set_visibility(boma, *FIGHTER_SHEIK_GENERATE_ARTICLE_NEEDLEHAVE, Hash40::new("set_main"), Hash40::new("group_main_5"), ArticleOperationTarget(0));
}