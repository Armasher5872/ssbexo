use super::*;

//Jigglypuff Reset Initialization
unsafe extern "C" fn purin_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
}

//Jigglypuff Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_PURIN, 7, false, false))]
unsafe extern "C" fn purin_death_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(get_agent_virtual_function(*FIGHTER_KIND_PURIN, 4, false, true)).data(purin_reset_initialization as *const () as u64);
	skyline::install_hook!(purin_death_initialization);
}