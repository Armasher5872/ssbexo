use super::*;

const PFUSHIGISOU_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xea03e0; //Ivysaur only

//Ivysaur Reset Initialization
unsafe extern "C" fn pfushigisou_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
}

//Ivysaur Death Initialization
#[skyline::hook(offset = PFUSHIGISOU_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pfushigisou_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x500ace8).data(pfushigisou_reset_initialization as *const () as u64);
    skyline::install_hook!(pfushigisou_death_initialization);
}