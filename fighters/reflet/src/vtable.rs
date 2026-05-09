use super::*;

const REFLET_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const REFLET_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x1005b20; //Robin only

//Robin Reset Initialization
#[skyline::hook(offset = REFLET_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn reflet_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_REFLET as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Robin Death Initialization
#[skyline::hook(offset = REFLET_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn reflet_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        reflet_reset_initialization,
        reflet_death_initialization
    );
}