use super::*;

pub unsafe extern "C" fn roy_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_ROY_INSTANCE_WORK_ID_FLAG_SPECIAL_S_CHARGED);
    WorkModule::off_flag(boma, *FIGHTER_ROY_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HIT);
    WorkModule::set_int(boma, 0, *FIGHTER_ROY_INSTANCE_WORK_ID_INT_SPECIAL_S_CHARGE);
}