use super::*;

const WIIFIT_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x12aab40; //Wii Fit Trainer only
const WIIFIT_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const WIIFIT_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x12aab50; //Wii Fit Trainer only

//Wii Fit Trainer Startup Initialization
#[skyline::hook(offset = WIIFIT_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn wiifit_start_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
}

//Wii Fit Trainer Reset Initialization
#[skyline::hook(offset = WIIFIT_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn wiifit_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_WIIFIT as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Wii Fit Trainer Death Initialization
#[skyline::hook(offset = WIIFIT_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn wiifit_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        wiifit_start_initialization,
        wiifit_reset_initialization,
        wiifit_death_initialization
    );
}