use super::*;

const CLOUD_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x8dacd0; //Cloud only
const CLOUD_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x8db3b0; //Cloud only
const CLOUD_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x8db780; //Cloud only

//Cloud Startup Initialization
#[skyline::hook(offset = CLOUD_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn cloud_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Cloud Reset Initialization
#[skyline::hook(offset = CLOUD_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn cloud_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Cloud Death Initialization
#[skyline::hook(offset = CLOUD_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn cloud_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        cloud_start_initialization,
        cloud_reset_initialization,
        cloud_death_initialization
    );
}