use super::*;

const MIISWORDSMAN_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xd99a30; //Mii Swordfighter only
const MIISWORDSMAN_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xd99d30; //Mii Swordfighter only
const MIISWORDSMAN_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xd99d50; //Mii Swordfighter only

//Mii Swordfighter Startup Initialization
#[skyline::hook(offset = MIISWORDSMAN_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn miiswordsman_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

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
        miiswordsman_start_initialization,
        miiswordsman_reset_initialization,
        miiswordsman_death_initialization
    );
}