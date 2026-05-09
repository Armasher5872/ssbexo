use super::*;

const PLIZARDON_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const PLIZARDON_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xf93b20; //Charizard only

//Charizard Reset Initialization
#[skyline::hook(offset = PLIZARDON_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn plizardon_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_PLIZARDON as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Charizard Death Initialization
#[skyline::hook(offset = PLIZARDON_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn plizardon_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        plizardon_reset_initialization,
        plizardon_death_initialization
    );
}