use super::*;

pub unsafe extern "C" fn kirby_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INPUT);
    WorkModule::set_int(boma, -1, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_TEAM_NO);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
}