use super::*;

const PIT_PITB_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xf6d1c0; //Shared
const PIT_PITB_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xf6e050; //Shared

//Pit & Dark Pit Reset Initialization
#[skyline::hook(offset = PIT_PITB_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pit_pitb_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Pit & Dark Pit Death Initialization
#[skyline::hook(offset = PIT_PITB_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pit_pitb_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        pit_pitb_reset_initialization,
        pit_pitb_death_initialization
    );
}