use super::*;

pub unsafe extern "C" fn captain_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_STORED);
    WorkModule::set_int(boma, 0, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_SPECIAL_N_EFFECT_HANDLE);
}