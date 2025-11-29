use super::*;

const NESS_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xdefdf0; //Ness only
const NESS_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared

//Ness Startup Initialization
#[skyline::hook(offset = NESS_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ness_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Ness Reset Initialization
#[skyline::hook(offset = NESS_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ness_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_NESS as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        ness_start_initialization,
        ness_reset_initialization
    );
}