use super::*;

//Incineroar Reset Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_GAOGAEN, 4, false, false))]
unsafe extern "C" fn gaogaen_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    gaogaen_var(&mut *boma);
    original!()(vtable, fighter)
}

//Incineroar Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_GAOGAEN, 7, false, false))]
unsafe extern "C" fn gaogaen_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    gaogaen_var(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        gaogaen_reset_initialization,
        gaogaen_death_initialization
    );
}