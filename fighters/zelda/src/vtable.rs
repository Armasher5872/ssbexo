use super::*;

//Zelda Reset Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_ZELDA, 4, false, false))]
unsafe extern "C" fn zelda_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Zelda Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_ZELDA, 7, false, false))]
unsafe extern "C" fn zelda_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        zelda_reset_initialization,
        zelda_death_initialization
    );
}