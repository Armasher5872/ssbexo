use super::*;

pub unsafe extern "C" fn pikachu_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_PIKACHU_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ENABLE_LANDING);
    WorkModule::off_flag(boma, *FIGHTER_PIKACHU_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_WEAK);
}