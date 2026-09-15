use super::*;

//Hero Reset Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_BRAVE, 4, false, false))]
unsafe extern "C" fn brave_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Hero Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_BRAVE, 7, false, false))]
unsafe extern "C" fn brave_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        brave_reset_initialization,
        brave_death_initialization
    );
}