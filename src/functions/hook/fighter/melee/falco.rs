use super::*;

const FALCO_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xa44a90; //Falco only
const FALCO_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xa44b90; //Falco only
const FALCO_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xa45360; //Falco only

//Falco Startup Initialization
#[skyline::hook(offset = FALCO_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn falco_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Falco Reset Initialization
#[skyline::hook(offset = FALCO_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn falco_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Falco Death Initialization
#[skyline::hook(offset = FALCO_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn falco_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        falco_start_initialization,
        falco_reset_initialization,
        falco_death_initialization
    );
}