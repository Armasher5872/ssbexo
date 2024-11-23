use super::*;

const NANA_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xfb6750; //Shared
const NANA_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xfb6a50; //Shared
const NANA_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xfb6c40; //Shared

//Nana Startup Initialization
#[skyline::hook(offset = NANA_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn nana_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_NANA as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_initialization_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Nana Reset Initialization
#[skyline::hook(offset = NANA_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn nana_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_NANA as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Nana Death Initialization
#[skyline::hook(offset = NANA_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn nana_death_initialization(vtable: u64, fighter: &mut Fighter, param_3: i32) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_NANA as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter, param_3)
}

pub fn install() {
    skyline::install_hooks!(
        nana_start_initialization,
        nana_reset_initialization,
        nana_death_initialization
    );
}