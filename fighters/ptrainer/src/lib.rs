use {
    exo_utils::fighter_common::*,
    smash::app::Fighter
};

const PTRAINER_VTABLE_RESPAWN_INITIALIZATION_OFFSET: usize = 0xf96330; //Shared

//Pokemon Trainer Respawn Initialization
#[skyline::hook(offset = PTRAINER_VTABLE_RESPAWN_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ptrainer_respawn_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
}

pub fn install() {
    skyline::install_hook!(ptrainer_respawn_initialization);
}