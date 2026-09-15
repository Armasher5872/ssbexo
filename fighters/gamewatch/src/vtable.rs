use super::*;

//Mr. Game & Watch Reset Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_GAMEWATCH, 4, false, false))]
unsafe extern "C" fn gamewatch_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Mr. Game & Watch Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_GAMEWATCH, 7, false, false))]
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