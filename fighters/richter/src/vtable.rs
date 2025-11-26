use super::*;

const RICHTER_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x1194120; //Shared
const RICHTER_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x1194130; //Shared
const RICHTER_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x11944e0; //Shared

//Richter Startup Initialization
#[skyline::hook(offset = RICHTER_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn richter_start_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_RICHTER as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Richter Reset Initialization
#[skyline::hook(offset = RICHTER_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn richter_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_RICHTER as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Richter Death Initialization
#[skyline::hook(offset = RICHTER_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn richter_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_RICHTER as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        richter_start_initialization,
        richter_reset_initialization,
        richter_death_initialization
    );
}