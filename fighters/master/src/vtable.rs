use super::*;

const MASTER_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xce92a0; //Byleth only
const MASTER_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xce9340; //Byleth only
const MASTER_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xce96d0; //Byleth only

//Byleth Startup Initialization
#[skyline::hook(offset = MASTER_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn master_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Byleth Reset Initialization
#[skyline::hook(offset = MASTER_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn master_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Byleth Death Initialization
#[skyline::hook(offset = MASTER_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn master_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        master_start_initialization,
        master_reset_initialization,
        master_death_initialization
    );
}