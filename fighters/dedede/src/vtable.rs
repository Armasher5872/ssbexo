use super::*;

//Dedede Reset Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_DEDEDE, 4, false, false))]
unsafe extern "C" fn dedede_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Dedede Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_DEDEDE, 7, false, false))]
unsafe extern "C" fn dedede_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        dedede_reset_initialization,
        dedede_death_initialization
    );
}