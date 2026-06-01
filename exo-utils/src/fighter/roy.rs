use super::*;

pub unsafe extern "C" fn roy_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_ROY_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HIT);
}