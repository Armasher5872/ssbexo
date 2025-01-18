use super::*;

const MARIO_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xcb9620; //Mario only
const MARIO_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const MARIO_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xcb9730; //Mario only

//Mario Startup Initialization
#[skyline::hook(offset = MARIO_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn mario_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Mario Reset Initialization
#[skyline::hook(offset = MARIO_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn mario_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_MARIO as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Mario Death Initialization
#[skyline::hook(offset = MARIO_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn mario_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        mario_start_initialization,
        mario_reset_initialization,
        mario_death_initialization
    );
}