use super::*;

const PIKMIN_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xf51380; //Olimar only
const PIKMIN_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xf51390; //Olimar only
const PIKMIN_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xf51df0; //Olimar only

//Olimar Startup Initialization
#[skyline::hook(offset = PIKMIN_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pikmin_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Olimar Reset Initialization
#[skyline::hook(offset = PIKMIN_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pikmin_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Olimar Death Initialization
#[skyline::hook(offset = PIKMIN_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pikmin_death_initialization(vtable: u64, fighter: &mut Fighter, pikmin_count: i32) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter, pikmin_count)
}

pub fn install() {
	skyline::install_hooks!(
        pikmin_start_initialization,
        pikmin_reset_initialization,
        pikmin_death_initialization
    );
}