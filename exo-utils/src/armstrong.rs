use super::*;

pub unsafe extern "C" fn is_armstrong_slots(boma: *mut BattleObjectModuleAccessor) -> bool {
    let color = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    (8..=15).contains(&color)
}