use super::*;

const BAYONETTA_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x819430; //Bayonetta only
const BAYONETTA_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x819440; //Bayonetta only
const BAYONETTA_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x81a050; //Bayonetta only

//Bayonetta Startup Initialization
#[skyline::hook(offset = BAYONETTA_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn bayonetta_start_initialization(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Bayonetta Reset Initialization
#[skyline::hook(offset = BAYONETTA_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn bayonetta_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Bayonetta Death Initialization
#[skyline::hook(offset = BAYONETTA_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn bayonetta_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        bayonetta_start_initialization,
        bayonetta_reset_initialization,
        bayonetta_death_initialization
    );
}