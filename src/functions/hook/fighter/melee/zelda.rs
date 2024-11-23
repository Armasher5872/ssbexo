use super::*;

const ZELDA_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x130e3e0; //Zelda only
const ZELDA_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x130e260; //Zelda only
const ZELDA_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x130e4f0; //Zelda only

//Zelda Startup Initialization
#[skyline::hook(offset = ZELDA_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn zelda_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Zelda Reset Initialization
#[skyline::hook(offset = ZELDA_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn zelda_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Zelda Death Initialization
#[skyline::hook(offset = ZELDA_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn zelda_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        zelda_start_initialization,
        zelda_reset_initialization,
        zelda_death_initialization
    );
}