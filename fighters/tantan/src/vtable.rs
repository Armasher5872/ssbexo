use super::*;

//Min-Min Reset Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_TANTAN, 4, false, false))]
unsafe extern "C" fn tantan_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma); 
    original!()(vtable, fighter)
}

//Min-Min Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_TANTAN, 4, false, false))]
unsafe extern "C" fn tantan_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma); 
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        tantan_reset_initialization,
        tantan_death_initialization
    );
}