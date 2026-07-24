use super::*;

pub unsafe extern "C" fn gaogaen_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_THROW_CRITICAL_ZOOM);
}