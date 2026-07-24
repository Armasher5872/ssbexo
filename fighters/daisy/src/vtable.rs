use super::*;

//Daisy Reset Initialization
unsafe extern "C" fn daisy_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
}

//Daisy Death Initialization
unsafe extern "C" fn daisy_death_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
}

unsafe extern "C" fn daisy_kassar_vtable_on_despawn_event_offset(_vtable: u64, weapon: *mut smash::app::Weapon) {
    let boma = (*weapon).battle_object.module_accessor;
    LinkModule::remove_model_constraint(boma, true);
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x5008a38).data(daisy_reset_initialization as *const () as u64);
    let _ = skyline::patching::Patch::in_text(0x5008a50).data(daisy_death_initialization as *const () as u64);
    let _ = skyline::patching::Patch::in_text(0x51face0).data(daisy_kassar_vtable_on_despawn_event_offset as *const () as u64);
}