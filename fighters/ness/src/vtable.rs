use super::*;

//Ness Reset Initialization
unsafe extern "C" fn ness_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x4ffce90).data(ness_reset_initialization as *const () as u64);
}