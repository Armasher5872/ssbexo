use super::*;

//Falco Reset Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_FALCO, 4, false, false))]
unsafe extern "C" fn falco_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Falco Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_FALCO, 7, false, false))]
unsafe extern "C" fn falco_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        falco_reset_initialization,
        falco_death_initialization
    );
}