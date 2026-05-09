use super::*;

const PZENIGAME_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const PZENIGAME_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xfef060; //Squirtle only

//Squirtle Reset Initialization
#[skyline::hook(offset = PZENIGAME_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pzenigame_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_PZENIGAME as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Squirtle Death Initialization
#[skyline::hook(offset = PZENIGAME_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pzenigame_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        pzenigame_reset_initialization,
        pzenigame_death_initialization
    );
}