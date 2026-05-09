use super::*;

pub unsafe extern "C" fn shizue_var(boma: &mut BattleObjectModuleAccessor) {
    let team_no = TeamModule::team_no(boma) as i32;
    WorkModule::set_int(boma, team_no, *FIGHTER_MURABTIO_INSTANCE_WORK_ID_INT_TEAM_NO);
    WorkModule::set_int(boma, *ITEM_KIND_NONE, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
}