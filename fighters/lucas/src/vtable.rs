use super::*;

const LUCAS_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xc73040; //Lucas only
const LUCAS_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xc732a0; //Shared
const LUCAS_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xc732c0; //Lucas only

//Lucas Startup Initialization
#[skyline::hook(offset = LUCAS_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucas_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Lucas Reset Initialization
#[skyline::hook(offset = LUCAS_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucas_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_LUCAS as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Lucas Death Initialization
#[skyline::hook(offset = LUCAS_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucas_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        lucas_start_initialization,
        lucas_reset_initialization,
        lucas_death_initialization
    );
}