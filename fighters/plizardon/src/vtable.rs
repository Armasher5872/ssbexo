use super::*;

const PLIZARDON_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xf93b20; //Charizard only

//Charizard Reset Initialization
unsafe extern "C" fn plizardon_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
}

//Charizard Death Initialization
#[skyline::hook(offset = PLIZARDON_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn plizardon_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x501a4c8).data(plizardon_reset_initialization as *const () as u64);
    skyline::install_hook!(plizardon_death_initialization);
}