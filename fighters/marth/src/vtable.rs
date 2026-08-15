use super::*;

//Marth & Lucina Reset Initialization
unsafe extern "C" fn marth_lucina_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
}

//Marth & Lucina Death Initialization
unsafe extern "C" fn marth_lucina_death_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x4fe6fc0).data(marth_lucina_reset_initialization as *const () as u64);
    let _ = skyline::patching::Patch::in_text(0x4fe6fd8).data(marth_lucina_death_initialization as *const () as u64);
}