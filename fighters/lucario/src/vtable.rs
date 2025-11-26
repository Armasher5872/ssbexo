use super::*;

const LUCARIO_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xc5b730; //Lucario only
const LUCARIO_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xc5b8f0; //Lucario only
const LUCARIO_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xc5bb60; //Lucario only

//Lucario Startup Initialization
#[skyline::hook(offset = LUCARIO_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucario_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Lucario Reset Initialization
#[skyline::hook(offset = LUCARIO_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucario_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Lucario Death Initialization
#[skyline::hook(offset = LUCARIO_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucario_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        lucario_start_initialization,
        lucario_reset_initialization,
        lucario_death_initialization
    );
}