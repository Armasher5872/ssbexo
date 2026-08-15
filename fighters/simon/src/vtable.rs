use super::*;

const SIMON_RICHTER_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x1194130; //Shared
const SIMON_RICHTER_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x11944e0; //Shared

//Simon & Richter Reset Initialization
#[skyline::hook(offset = SIMON_RICHTER_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn simon_richter_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma); 
    original!()(vtable, fighter)
}

//Simon & Richter Death Initialization
#[skyline::hook(offset = SIMON_RICHTER_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn simon_richter_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma); 
    original!()(vtable, fighter)
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x1195224).data(0x7100001F); //Credited to HDR, makes it so Simon/Richter only enter the Cross Catch animation if they are idle
	skyline::install_hooks!(
        simon_richter_reset_initialization,
        simon_richter_death_initialization
    );
}