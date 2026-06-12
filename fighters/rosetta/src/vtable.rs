use super::*;

const ROSETTA_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const ROSETTA_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x10a88e0; //Rosalina & Luma only

//Rosalina & Luma Reset Initialization
#[skyline::hook(offset = ROSETTA_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn rosetta_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_ROSETTA as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Rosalina & Luma Death Initialization
#[skyline::hook(offset = ROSETTA_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn rosetta_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        rosetta_reset_initialization,
        rosetta_death_initialization
    );
}