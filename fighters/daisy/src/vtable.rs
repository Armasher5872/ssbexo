use super::*;

unsafe extern "C" fn daisy_kassar_vtable_on_despawn_event_offset(_vtable: u64, weapon: *mut smash::app::Weapon) {
    let boma = (*weapon).battle_object.module_accessor;
    LinkModule::remove_model_constraint(boma, true);
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(get_agent_virtual_function(*WEAPON_KIND_DAISY_KASSAR, 9, true, true)).data(daisy_kassar_vtable_on_despawn_event_offset as *const () as u64);
}