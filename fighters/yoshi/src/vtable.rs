use super::*;

//Yoshi Reset Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_YOSHI, 4, false, false))]
unsafe extern "C" fn yoshi_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Yoshi Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_YOSHI, 7, false, false))]
unsafe extern "C" fn yoshi_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        yoshi_reset_initialization,
        yoshi_death_initialization
    );
}