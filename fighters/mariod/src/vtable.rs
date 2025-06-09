use super::*;

const MARIOD_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xcc8e10; //Dr. Mario only
const MARIOD_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const MARIOD_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xcc8f20; //Dr. Mario only
const MARIOD_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xcc96c0; //Dr. Mario only

//Dr. Mario Startup Initialization
#[skyline::hook(offset = MARIOD_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn mariod_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    common_initialization_variable_reset(&mut *boma);
    WorkModule::set_int(boma, UiManager::get_mariod_pill_id(entry_id), *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    UiManager::set_mariod_meter_info(entry_id, 0);
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
        WorkModule::set_int(boma, UiManager::get_mariod_pill_id(entry_id), *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
        UiManager::set_mariod_meter_info(entry_id, 0);
    }
    original!()(vtable, fighter)
}

//Dr. Mario Death Initialization
#[skyline::hook(offset = MARIOD_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn mariod_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    common_death_variable_reset(&mut *boma);
    WorkModule::set_int(boma, UiManager::get_mariod_pill_id(entry_id), *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    UiManager::set_mariod_meter_info(entry_id, 0);
    original!()(vtable, fighter)
}

//Dr. Mario Once Per Fighter Frame
#[skyline::hook(offset = MARIOD_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn mariod_opff(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    WorkModule::off_flag(boma, *FIGHTER_MARIOD_INSTANCE_WORK_ID_FLAG_SPECIAL_S_REFLECTOR);
    WorkModule::off_flag(boma, *FIGHTER_MARIOD_INSTANCE_WORK_ID_FLAG_SEPCIAL_S_REFLECTOR_BREAK);
    UiManager::set_mariod_meter_enable(entry_id, true);
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x3441a18).nop(); //The following removes a horizontal speed initialization so it can be assigned dynamically in a weapon init status
    let _ = skyline::patching::Patch::in_text(0x3441a3c).nop(); //The following removes the initial gravity acceleration so it can be assigned dynamically in a weapon init status
    skyline::install_hooks!(
        mariod_start_initialization,
        mariod_reset_initialization,
        mariod_death_initialization,
        mariod_opff
    );
}