use super::*;

//Steve Reset Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_PICKEL, 4, false, false))]
unsafe extern "C" fn pickel_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Steve Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_PICKEL, 7, false, false))]
unsafe extern "C" fn pickel_death_initialization(vtable: u64, fighter: &mut Fighter, param_3: i32) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter, param_3)
}

pub fn install() {
	skyline::install_hooks!(
        pickel_reset_initialization,
        pickel_death_initialization
    );
}