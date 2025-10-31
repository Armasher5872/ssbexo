use super::*;

const MEWTWO_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xd28a20; //Mewtwo only
const MEWTWO_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const MEWTWO_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xd294e0; //Mewtwo only

//Mewtwo Startup Initialization
#[skyline::hook(offset = MEWTWO_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn mewtwo_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Mewtwo Reset Initialization
#[skyline::hook(offset = MEWTWO_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn mewtwo_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_MEWTWO as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Mewtwo Death Initialization
#[skyline::hook(offset = MEWTWO_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn mewtwo_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        mewtwo_start_initialization,
        mewtwo_reset_initialization,
        mewtwo_death_initialization
    );
}