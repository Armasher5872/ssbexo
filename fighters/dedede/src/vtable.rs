use super::*;

const DEDEDE_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x904520; //Dedede only
const DEDEDE_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x904cf0; //Dedede only

//Dedede Reset Initialization
#[skyline::hook(offset = DEDEDE_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn dedede_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Dedede Death Initialization
#[skyline::hook(offset = DEDEDE_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn dedede_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        dedede_reset_initialization,
        dedede_death_initialization
    );
}