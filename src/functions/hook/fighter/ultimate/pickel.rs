use super::*;

const PICKEL_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xf02a00; //Steve only
const PICKEL_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xf030d0; //Steve only
const PICKEL_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xf036a0; //Steve only

//Steve Startup Initialization
#[skyline::hook(offset = PICKEL_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pickel_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Steve Reset Initialization
#[skyline::hook(offset = PICKEL_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pickel_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Steve Death Initialization
#[skyline::hook(offset = PICKEL_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pickel_death_initialization(vtable: u64, fighter: &mut Fighter, param_3: i32) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter, param_3)
}

pub fn install() {
	skyline::install_hooks!(
        pickel_start_initialization,
        pickel_reset_initialization,
        pickel_death_initialization
    );
}