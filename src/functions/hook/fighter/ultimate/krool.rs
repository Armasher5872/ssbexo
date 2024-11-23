use super::*;

const KROOL_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xc02400; //King K Rool only
const KROOL_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xc026a0; //King K Rool only
const KROOL_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xc04290; //King K Rool only

//King K Rool Startup Initialization
#[skyline::hook(offset = KROOL_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn krool_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//King K Rool Reset Initialization
#[skyline::hook(offset = KROOL_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn krool_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//King K Rool Death Initialization
#[skyline::hook(offset = KROOL_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn krool_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        krool_start_initialization,
        krool_reset_initialization,
        krool_death_initialization
    );
}