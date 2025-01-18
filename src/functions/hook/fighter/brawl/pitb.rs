use super::*;

const PITB_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xf6d5c0; //Shared
const PITB_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xf6d1c0; //Shared
const PITB_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xf6e050; //Shared

//Dark Pit Startup Initialization
#[skyline::hook(offset = PITB_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pitb_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_PITB as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Dark Pit Reset Initialization
#[skyline::hook(offset = PITB_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pitb_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_PITB as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Dark Pit Death Initialization
#[skyline::hook(offset = PITB_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pitb_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_PITB as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        pitb_start_initialization,
        pitb_reset_initialization,
        pitb_death_initialization
    );
}