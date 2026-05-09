use super::*;

const RIDLEY_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x1043a20; //Ridley only
const RIDLEY_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x1043d20; //Ridley only

//Ridley Reset Initialization
#[skyline::hook(offset = RIDLEY_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ridley_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Ridley Death Initialization
#[skyline::hook(offset = RIDLEY_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ridley_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        ridley_reset_initialization,
        ridley_death_initialization
    );
}