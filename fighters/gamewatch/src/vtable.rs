use super::*;

const GAMEWATCH_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xa80d30; //Game & Watch only
const GAMEWATCH_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xa80fd0; //Game & Watch only
const GAMEWATCH_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xa81240; //Game & Watch only

//Mr. Game & Watch Startup Initialization
#[skyline::hook(offset = GAMEWATCH_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn gamewatch_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    agent.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(should_use_special_hi_callback as *const () as _));
    original!()(vtable, fighter)
}

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
        gamewatch_start_initialization,
        gamewatch_reset_initialization,
        gamewatch_death_initialization
    );
}