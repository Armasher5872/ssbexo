use super::*;

pub unsafe extern "C" fn dedede_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_LINK_ITEM_FUSE_BACK);
    WorkModule::set_int(boma, *ITEM_KIND_NONE, *FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
}