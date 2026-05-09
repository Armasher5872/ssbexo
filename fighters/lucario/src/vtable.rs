use super::*;

const LUCARIO_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xc5b8f0; //Lucario only
const LUCARIO_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xc5bb60; //Lucario only

//Lucario Reset Initialization
#[skyline::hook(offset = LUCARIO_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucario_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Lucario Death Initialization
#[skyline::hook(offset = LUCARIO_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucario_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        lucario_reset_initialization,
        lucario_death_initialization
    );
}