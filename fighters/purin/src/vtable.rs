use super::*;

const PURIN_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xfdf980; //Jigglypuff only

//Jigglypuff Reset Initialization
unsafe extern "C" fn purin_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
}

//Jigglypuff Death Initialization
#[skyline::hook(offset = PURIN_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn purin_death_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x501f338).data(purin_reset_initialization as *const () as u64);
	skyline::install_hook!(purin_death_initialization);
}