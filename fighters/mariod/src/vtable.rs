use super::*;

const MARIOD_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xcc8e10; //Dr. Mario only
const MARIOD_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const MARIOD_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xcc8f20; //Dr. Mario only
const MARIOD_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xcc96c0; //Dr. Mario only
const MARIOD_VTABLE_ON_ATTACK_OFFSET: usize = 0x68d7e0; //Shared

//Dr. Mario Startup Initialization
#[skyline::hook(offset = MARIOD_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn mariod_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    common_initialization_variable_reset(&mut *boma);
    UiManager::set_mariod_meter_info(entry_id, 0);
    WorkModule::set_int(boma, UiManager::get_mariod_pill_id(entry_id), *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Dr. Mario Reset Initialization
#[skyline::hook(offset = MARIOD_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn mariod_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_MARIOD as u32 {
        let boma = fighter.battle_object.module_accessor;
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
        common_reset_variable_reset(&mut *boma);
        UiManager::set_mariod_meter_info(entry_id, 0);
        WorkModule::set_int(boma, UiManager::get_mariod_pill_id(entry_id), *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    }
    original!()(vtable, fighter)
}

//Dr. Mario Death Initialization
#[skyline::hook(offset = MARIOD_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn mariod_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    common_death_variable_reset(&mut *boma);
    UiManager::set_mariod_meter_info(entry_id, 0);
    WorkModule::set_int(boma, UiManager::get_mariod_pill_id(entry_id), *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    original!()(vtable, fighter)
}

//Dr. Mario Once Per Fighter Frame
#[skyline::hook(offset = MARIOD_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn mariod_opff(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    UiManager::set_mariod_meter_enable(entry_id, true);
}

//Dr. Mario On Attack
#[skyline::hook(offset = MARIOD_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn mariod_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_MARIOD as u32 {
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
    original!()(vtable, fighter, log)
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x34417a8).nop(); //The following removes a horizontal speed initialization so it can be assigned dynamically in a weapon init status
    let _ = skyline::patching::Patch::in_text(0x34417cc).nop(); //The following removes the initial gravity acceleration so it can be assigned dynamically in a weapon init status
    skyline::install_hooks!(
        mariod_start_initialization,
        mariod_reset_initialization,
        mariod_death_initialization,
        mariod_opff,
        mariod_on_attack
    );
}