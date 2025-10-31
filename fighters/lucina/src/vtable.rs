use super::*;

const LUCINA_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xcd98a0; //Shared
const LUCINA_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xc732a0; //Shared
const LUCINA_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xcd99a0; //Shared

//Lucina Startup Initialization
#[skyline::hook(offset = LUCINA_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucina_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_LUCINA as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Lucina Reset Initialization
#[skyline::hook(offset = LUCINA_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucina_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_LUCINA as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Lucina Death Initialization
#[skyline::hook(offset = LUCINA_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucina_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_LUCINA as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        lucina_start_initialization,
        lucina_reset_initialization,
        lucina_death_initialization
    );
}