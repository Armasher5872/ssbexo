use super::*;

pub unsafe extern "C" fn koopajr_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_UNIQ_FLOAT);
    WorkModule::set_int(boma, 0, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_INT_FLOAT_TIME);
}