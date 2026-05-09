use super::*;

const DAISY_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const DAISY_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xe89090; //Shared

//Daisy Reset Initialization
#[skyline::hook(offset = DAISY_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn daisy_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_DAISY as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Daisy Death Initialization
#[skyline::hook(offset = DAISY_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn daisy_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_DAISY as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

unsafe extern "C" fn daisy_kassar_vtable_on_despawn_event_offset(_vtable: u64, weapon: *mut smash::app::Weapon) {
    let boma = (*weapon).battle_object.module_accessor;
    LinkModule::remove_model_constraint(boma, true);
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x51face0).data(daisy_kassar_vtable_on_despawn_event_offset as *const () as u64);
    skyline::install_hooks!(
        daisy_reset_initialization,
        daisy_death_initialization
    );
}