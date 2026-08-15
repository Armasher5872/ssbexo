use super::*;

const WIIFIT_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x12aab50; //Wii Fit Trainer only

//Wii Fit Trainer Reset Initialization
unsafe extern "C" fn wiifit_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
}

//Wii Fit Trainer Death Initialization
#[skyline::hook(offset = WIIFIT_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn wiifit_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x5053a68).data(wiifit_reset_initialization as *const () as u64);
    skyline::install_hook!(wiifit_death_initialization);
}