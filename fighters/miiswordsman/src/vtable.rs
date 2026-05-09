use super::*;

const MIISWORDSMAN_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xd99d30; //Mii Swordfighter only
const MIISWORDSMAN_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xd99d50; //Mii Swordfighter only

//Mii Swordfighter Reset Initialization
#[skyline::hook(offset = MIISWORDSMAN_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn miiswordsman_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Mii Swordfighter Death Initialization
#[skyline::hook(offset = MIISWORDSMAN_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn miiswordsman_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        miiswordsman_reset_initialization,
        miiswordsman_death_initialization
    );
}