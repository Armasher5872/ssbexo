use super::*;

//Corrin Reset Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_KAMUI, 4, false, false))]
unsafe extern "C" fn kamui_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Corrin Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_KAMUI, 7, false, false))]
unsafe extern "C" fn kamui_death_initialization(vtable: u64, fighter: &mut Fighter, param_3: u32) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter, param_3)
}

pub fn install() {
    skyline::install_hooks!(
        kamui_reset_initialization,
        kamui_death_initialization
    );
}