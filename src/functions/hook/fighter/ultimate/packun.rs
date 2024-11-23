use super::*;

const PACKUN_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xe09b90; //Piranha Plant only
const PACKUN_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const PACKUN_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xe09f70; //Piranha Plant only

//Piranha Plant Startup Initialization
#[skyline::hook(offset = PACKUN_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn packun_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Piranha Plant Reset Initialization
#[skyline::hook(offset = PACKUN_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn packun_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_PACKUN as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Piranha Plant Death Initialization
#[skyline::hook(offset = PACKUN_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn packun_death_initialization(vtable: u64, fighter: &mut Fighter, param_3: i32) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter, param_3)
}

pub fn install() {
	skyline::install_hooks!(
        packun_start_initialization,
        packun_reset_initialization,
        packun_death_initialization
    );
}