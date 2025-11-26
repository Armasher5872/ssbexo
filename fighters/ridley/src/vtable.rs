use super::*;

const RIDLEY_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x68d5a0; //Shared
const RIDLEY_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x1043a20; //Ridley only
const RIDLEY_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x1043d20; //Ridley only

//Ridley Startup Initialization
#[skyline::hook(offset = RIDLEY_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ridley_start_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_RIDLEY as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Ridley Reset Initialization
#[skyline::hook(offset = RIDLEY_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ridley_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Ridley Death Initialization
#[skyline::hook(offset = RIDLEY_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ridley_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        ridley_start_initialization,
        ridley_reset_initialization,
        ridley_death_initialization
    );
}