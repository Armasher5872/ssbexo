use super::*;

const INKLING_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xb0ac70; //Inkling only
const INKLING_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xb0ac80; //Inkling only
const INKLING_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xb0ad10; //Inkling only

//Inkling Startup Initialization
#[skyline::hook(offset = INKLING_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn inkling_start_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
}

//Inkling Reset Initialization
#[skyline::hook(offset = INKLING_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn inkling_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Inkling Death Initialization
#[skyline::hook(offset = INKLING_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn inkling_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        inkling_start_initialization,
        inkling_reset_initialization,
        inkling_death_initialization
    );
}