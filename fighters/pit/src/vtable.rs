use super::*;

const PIT_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xf6d1c0; //Shared
const PIT_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xf6e050; //Shared

//Pit Reset Initialization
#[skyline::hook(offset = PIT_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pit_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_PIT as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Pit Death Initialization
#[skyline::hook(offset = PIT_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pit_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_PIT as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        pit_reset_initialization,
        pit_death_initialization
    );
}