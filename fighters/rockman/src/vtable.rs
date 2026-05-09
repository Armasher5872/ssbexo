use super::*;

const ROCKMAN_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x107db60; //Mega-Man only
const ROCKMAN_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x107e890; //Mega-Man only

//Megaman Reset Initialization
#[skyline::hook(offset = ROCKMAN_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn rockman_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Mega-Man Death Initialization
#[skyline::hook(offset = ROCKMAN_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn rockman_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        rockman_reset_initialization,
        rockman_death_initialization
    );
}