use super::*;

const KROOL_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xc026a0; //King K Rool only
const KROOL_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xc04290; //King K Rool only
const KROOL_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xc04c20; //King K Rool only

//King K Rool Reset Initialization
#[skyline::hook(offset = KROOL_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn krool_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    krool_var(boma);
    original!()(vtable, fighter)
}

//King K Rool Death Initialization
#[skyline::hook(offset = KROOL_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn krool_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    krool_var(boma);
    original!()(vtable, fighter)
}

//King K Rool OPFF
#[skyline::hook(offset = KROOL_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn krool_opff(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let special_lw_fuel = WorkModule::get_float(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_FUEL);
    let special_lw_timer = WorkModule::get_int(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_LW_TIMER);
    if special_lw_fuel < 1.0 {
        WorkModule::add_float(boma, 0.00416, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_FUEL);
    }
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE) {
        if special_lw_timer > 0 {
            WorkModule::dec_int(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_LW_TIMER);
        }
        if special_lw_timer <= 0 {
            WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
        }
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        krool_reset_initialization,
        krool_death_initialization,
        krool_opff
    );
}