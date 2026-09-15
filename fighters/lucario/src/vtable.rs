use super::*;

//Lucario Reset Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_LUCARIO, 4, false, false))]
unsafe extern "C" fn lucario_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Lucario Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_LUCARIO, 7, false, false))]
unsafe extern "C" fn lucario_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        lucario_reset_initialization,
        lucario_death_initialization
    );
}