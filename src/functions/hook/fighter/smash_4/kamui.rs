use super::*;

const KAMUI_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xb5c1a0; //Corrin only
const KAMUI_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xb5c360; //Corrin only
const KAMUI_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xb5cb30; //Corrin only

//Corrin Startup Initialization
#[skyline::hook(offset = KAMUI_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn kamui_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Corrin Reset Initialization
#[skyline::hook(offset = KAMUI_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn kamui_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Corrin Death Initialization
#[skyline::hook(offset = KAMUI_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn kamui_death_initialization(vtable: u64, fighter: &mut Fighter, param_3: u32) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter, param_3)
}

pub fn install() {
    skyline::install_hooks!(
        kamui_start_initialization,
        kamui_reset_initialization,
        kamui_death_initialization
    );
}