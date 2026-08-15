use super::*;

const ROSETTA_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x10a88e0; //Rosalina & Luma only

//Rosalina & Luma Reset Initialization
unsafe extern "C" fn rosetta_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
}

//Rosalina & Luma Death Initialization
#[skyline::hook(offset = ROSETTA_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn rosetta_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x502f2a8).data(rosetta_reset_initialization as *const () as u64);
	skyline::install_hook!(rosetta_death_initialization);
}