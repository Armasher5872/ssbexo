use super::*;

//Dr. Mario Reset Initialization
unsafe extern "C" fn mariod_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    common_reset_variable_reset(&mut *boma);
    UiManager::set_mariod_meter_info(entry_id, 0);
    WorkModule::set_int(boma, UiManager::get_mariod_pill_id(entry_id), *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
}

//Dr. Mario Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_MARIOD, 7, false, false))]
unsafe extern "C" fn mariod_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    common_death_variable_reset(&mut *boma);
    UiManager::set_mariod_meter_info(entry_id, 0);
    WorkModule::set_int(boma, UiManager::get_mariod_pill_id(entry_id), *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    original!()(vtable, fighter)
}

//Dr. Mario Once Per Fighter Frame
unsafe extern "C" fn mariod_opff(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    UiManager::set_mariod_meter_enable(entry_id, true);
}

//Dr. Mario On Attack
unsafe extern "C" fn mariod_on_attack(_vtable: u64, fighter: &mut Fighter, _log: u64) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    let status_kind = StatusModule::status_kind(boma);
    if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_DASH].contains(&status_kind) {
        UiManager::set_mariod_meter_info(entry_id, 1);
        WorkModule::set_int(boma, UiManager::get_mariod_pill_id(entry_id), *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    }
    if [*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) {
        UiManager::set_mariod_meter_info(entry_id, 2);
        WorkModule::set_int(boma, UiManager::get_mariod_pill_id(entry_id), *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    }
    if [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_CATCH_ATTACK, *FIGHTER_STATUS_KIND_THROW].contains(&status_kind) {
        UiManager::set_mariod_meter_info(entry_id, 3);
        WorkModule::set_int(boma, UiManager::get_mariod_pill_id(entry_id), *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    }
    if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
        UiManager::set_mariod_meter_info(entry_id, 0);
        WorkModule::set_int(boma, UiManager::get_mariod_pill_id(entry_id), *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    }
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x34414dc).nop(); //The following removes the life initialization so it can be assigned dynamically in a weapon init status
    let _ = skyline::patching::Patch::in_text(0x34417a8).nop(); //The following removes the horizontal speed initialization so it can be assigned dynamically in a weapon init status
    let _ = skyline::patching::Patch::in_text(0x34417cc).nop(); //The following removes the initial gravity acceleration so it can be assigned dynamically in a weapon init status
    let _ = skyline::patching::Patch::in_text(get_agent_virtual_function(*FIGHTER_KIND_MARIOD, 4, false, true)).data(mariod_reset_initialization as *const () as u64);
    let _ = skyline::patching::Patch::in_text(get_agent_virtual_function(*FIGHTER_KIND_MARIOD, 13, false, true)).data(mariod_opff as *const () as u64);
    let _ = skyline::patching::Patch::in_text(get_agent_virtual_function(*FIGHTER_KIND_MARIOD, 36, false, true)).data(mariod_on_attack as *const () as u64);
    skyline::install_hook!(mariod_death_initialization);
}