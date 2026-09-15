use super::*;

//Megaman Reset Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_ROCKMAN, 4, false, false))]
unsafe extern "C" fn rockman_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Mega-Man Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_ROCKMAN, 7, false, false))]
unsafe extern "C" fn rockman_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        rockman_reset_initialization,
        rockman_death_initialization
    );
}