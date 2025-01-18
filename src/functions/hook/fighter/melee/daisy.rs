use super::*;

const DAISY_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xe88ca0; //Shared
const DAISY_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const DAISY_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xe89090; //Shared

//Daisy Startup Initialization
#[skyline::hook(offset = DAISY_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn daisy_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_DAISY as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        agent.global_table[CHECK_AIR_JUMP_UNIQ].assign(&false.into());
        agent.global_table[CHECK_AIR_JUMP_AERIAL_UNIQ].assign(&false.into());
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Daisy Reset Initialization
#[skyline::hook(offset = DAISY_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn daisy_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_DAISY as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Daisy Death Initialization
#[skyline::hook(offset = DAISY_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn daisy_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_DAISY as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        daisy_start_initialization,
        daisy_reset_initialization,
        daisy_death_initialization
    );
}