use super::*;

const PZENIGAME_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xfef060; //Squirtle only

//Squirtle Reset Initialization
unsafe extern "C" fn pzenigame_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
}

//Squirtle Death Initialization
#[skyline::hook(offset = PZENIGAME_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pzenigame_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x5020e78).data(pzenigame_reset_initialization as *const () as u64);
    skyline::install_hook!(pzenigame_death_initialization);
}