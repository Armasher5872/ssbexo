use super::*;

const METAKNIGHT_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x68d5a0; //Shared
const METAKNIGHT_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const METAKNIGHT_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xd12b90; //Meta Knight only

//Meta Knight Startup Initialization
#[skyline::hook(offset = METAKNIGHT_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn metaknight_start_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_METAKNIGHT as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Meta Knight Reset Initialization
#[skyline::hook(offset = METAKNIGHT_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn metaknight_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_METAKNIGHT as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Meta Knight Death Initialization
#[skyline::hook(offset = METAKNIGHT_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn metaknight_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        metaknight_start_initialization,
        metaknight_reset_initialization,
        metaknight_death_initialization
    );
}