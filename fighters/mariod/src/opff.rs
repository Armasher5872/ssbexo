use super::*;

unsafe extern "C" fn mariod_on_start(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    common_initialization_variable_reset(&mut *boma);
    UiManager::set_mariod_meter_info(entry_id, 0);
    WorkModule::set_int(boma, UiManager::get_mariod_pill_id(entry_id), *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
}

pub fn install() {
    Agent::new("mariod")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_start(mariod_on_start)
    .install()
    ;
}