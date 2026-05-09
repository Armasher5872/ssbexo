use super::*;

const GAMEWATCH_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xa80fd0; //Game & Watch only
const GAMEWATCH_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xa81240; //Game & Watch only

//Mr. Game & Watch Reset Initialization
#[skyline::hook(offset = GAMEWATCH_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn gamewatch_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Mr. Game & Watch Death Initialization
#[skyline::hook(offset = GAMEWATCH_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn gamewatch_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        gamewatch_reset_initialization,
        gamewatch_death_initialization
    );
}