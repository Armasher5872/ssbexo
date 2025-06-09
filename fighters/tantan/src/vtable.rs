use super::*;

const TANTAN_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x1223e20; //Min-Min only
const TANTAN_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x12241c0; //Min-Min only
const TANTAN_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x12247d0; //Min-Min only

//Min-Min Startup Initialization
#[skyline::hook(offset = TANTAN_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn tantan_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Min-Min Reset Initialization
#[skyline::hook(offset = TANTAN_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn tantan_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma); 
    original!()(vtable, fighter)
}

//Min-Min Death Initialization
#[skyline::hook(offset = TANTAN_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn tantan_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma); 
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        tantan_start_initialization,
        tantan_reset_initialization,
        tantan_death_initialization
    );
}