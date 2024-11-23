use super::*;

const TRAIL_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x1266a40; //Sora only
const TRAIL_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x1266c60; //Sora only
const TRAIL_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x1267300; //Sora only

//Sora Startup Initialization
#[skyline::hook(offset = TRAIL_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn trail_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma); 
    original!()(vtable, fighter)
}

//Sora Reset Initialization
#[skyline::hook(offset = TRAIL_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn trail_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma); 
    original!()(vtable, fighter)
}

//Sora Death Initialization
#[skyline::hook(offset = TRAIL_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn trail_death_initialization(vtable: u64, fighter: &mut Fighter, param_3: u32) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma); 
    original!()(vtable, fighter, param_3)
}

pub fn install() {
    skyline::install_hooks!(
        trail_start_initialization,
        trail_reset_initialization,
        trail_death_initialization
    );
}