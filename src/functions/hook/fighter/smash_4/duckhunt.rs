use super::*;

const DUCKHUNT_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x9a0cf0; //Duck Hunt only
const DUCKHUNT_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x9a0d00; //Duck Hunt only
const DUCKHUNT_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x9a13f0; //Duck Hunt only

//Duck Hunt Startup Initialization
#[skyline::hook(offset = DUCKHUNT_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn duckhunt_start_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
}

//Duck Hunt Reset Initialization
#[skyline::hook(offset = DUCKHUNT_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn duckhunt_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Duck Hunt Death Initialization
#[skyline::hook(offset = DUCKHUNT_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn duckhunt_death_initialization(vtable: u64, fighter: &mut Fighter, param_3: u32) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter, param_3)
}

pub fn install() {
	skyline::install_hooks!(
        duckhunt_start_initialization,
        duckhunt_reset_initialization,
        duckhunt_death_initialization
    );
}