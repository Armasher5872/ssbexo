use super::*;

//Ivysaur Reset Initialization
unsafe extern "C" fn pfushigisou_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
}

//Ivysaur Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_PFUSHIGISOU, 7, false, false))]
unsafe extern "C" fn pfushigisou_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(get_agent_virtual_function(*FIGHTER_KIND_PFUSHIGISOU, 4, false, true)).data(pfushigisou_reset_initialization as *const () as u64);
    skyline::install_hook!(pfushigisou_death_initialization);
}