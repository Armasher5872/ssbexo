use super::*;

const PACMAN_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xe29310; //Pac-Man only
const PACMAN_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xe29320; //Pac-Man only
const PACMAN_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xe29340; //Pac-Man only

//Pac-Man Startup Initialization
#[skyline::hook(offset = PACMAN_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pacman_start_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
}

//Pac-Man Reset Initialization
#[skyline::hook(offset = PACMAN_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pacman_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Pac-Man Death Initialization
#[skyline::hook(offset = PACMAN_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pacman_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        pacman_start_initialization,
        pacman_reset_initialization,
        pacman_death_initialization
    );
}