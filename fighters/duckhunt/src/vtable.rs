use super::*;

//Duck Hunt Reset Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_DUCKHUNT, 4, false, false))]
unsafe extern "C" fn duckhunt_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Duck Hunt Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_DUCKHUNT, 4, false, false))]
unsafe extern "C" fn duckhunt_death_initialization(vtable: u64, fighter: &mut Fighter, param_3: u32) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter, param_3)
}

pub fn install() {
	skyline::install_hooks!(
        duckhunt_reset_initialization,
        duckhunt_death_initialization
    );
}