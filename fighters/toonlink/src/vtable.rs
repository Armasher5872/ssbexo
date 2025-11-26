use super::*;

const TOONLINK_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xc280f0; //Shared
const TOONLINK_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xc28280; //Shared
const TOONLINK_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xc28860; //Shared

//Toon Link Startup Initialization
#[skyline::hook(offset = TOONLINK_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn toonlink_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_TOONLINK as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Toon Link Reset Initialization
#[skyline::hook(offset = TOONLINK_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn toonlink_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_TOONLINK as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Toon Link Death Initialization
#[skyline::hook(offset = TOONLINK_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn toonlink_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_TOONLINK as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        toonlink_start_initialization,
        toonlink_reset_initialization,
        toonlink_death_initialization
    );
}