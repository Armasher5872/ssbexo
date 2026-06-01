use super::*;

pub unsafe extern "C" fn wario_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_N_THROW);
    WorkModule::off_flag(boma, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_STRONG);
    WorkModule::off_flag(boma, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP);
    WorkModule::set_float(boma, 1.0, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_PILEDRIVER_MULTIPLIER);
    WorkModule::set_float(boma, 0.0, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_CHARGE);
    WorkModule::set_int(boma, 0, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_SPECIAL_S_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_TIMER);
}