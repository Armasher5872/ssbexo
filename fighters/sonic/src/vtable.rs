use super::*;

const SONIC_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x11d56f0; //Sonic only
const SONIC_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const SONIC_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x11d5820; //Sonic only

//Sonic Startup Initialization
#[skyline::hook(offset = SONIC_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn sonic_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Sonic Reset Initialization
#[skyline::hook(offset = SONIC_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn sonic_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_SONIC as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Sonic Death Initialization
#[skyline::hook(offset = SONIC_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn sonic_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        sonic_start_initialization,
        sonic_reset_initialization,
        sonic_death_initialization
    );
}