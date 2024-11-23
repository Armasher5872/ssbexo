use super::*;

const BRAVE_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x850380; //Hero only
const BRAVE_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x850890; //Hero only
const BRAVE_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x851fe0; //Hero only

//Hero Startup Initialization
#[skyline::hook(offset = BRAVE_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn brave_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Hero Reset Initialization
#[skyline::hook(offset = BRAVE_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn brave_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Hero Death Initialization
#[skyline::hook(offset = BRAVE_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn brave_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        brave_start_initialization,
        brave_reset_initialization,
        brave_death_initialization
    );
}