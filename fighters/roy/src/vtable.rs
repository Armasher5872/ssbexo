use super::*;

const ROY_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const ROY_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x10bb700; //Shared

//Roy Reset Initialization
#[skyline::hook(offset = ROY_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn roy_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_ROY as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        roy_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Roy Death Initialization
#[skyline::hook(offset = ROY_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn roy_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_ROY as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
        roy_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        roy_reset_initialization,
        roy_death_initialization
    );
}