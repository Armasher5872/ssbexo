use super::*;

const PEACH_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const PEACH_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xe89090; //Shared

//Peach Reset Initialization
#[skyline::hook(offset = PEACH_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn peach_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_PEACH as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Peach Death Initialization
#[skyline::hook(offset = PEACH_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn peach_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_PEACH as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        peach_reset_initialization,
        peach_death_initialization
    );
}