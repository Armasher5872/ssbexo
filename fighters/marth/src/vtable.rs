use super::*;

const MARTH_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xcd98a0; //Shared
const MARTH_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xc732a0; //Shared
const MARTH_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xcd99a0; //Shared

//Marth Startup Initialization
#[skyline::hook(offset = MARTH_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn marth_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_MARTH as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Marth Reset Initialization
#[skyline::hook(offset = MARTH_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn marth_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_MARTH as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Marth Death Initialization
#[skyline::hook(offset = MARTH_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn marth_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_MARTH as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        marth_start_initialization,
        marth_reset_initialization,
        marth_death_initialization
    );
}