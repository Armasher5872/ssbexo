use super::*;

//Isabelle Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_SHIZUE, 7, false, false))]
unsafe extern "C" fn shizue_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hook!(shizue_death_initialization);
}