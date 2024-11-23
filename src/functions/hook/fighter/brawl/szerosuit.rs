use super::*;

const SZEROSUIT_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x11f4ba0; //Zero Suit Samus only
const SZEROSUIT_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x11f4c40; //Zero Suit Samus only
const SZEROSUIT_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x11f4c70; //Zero Suit Samus only

//Zero Suit Samus Startup Initialization
#[skyline::hook(offset = SZEROSUIT_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn szerosuit_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Zero Suit Samus Reset Initialization
#[skyline::hook(offset = SZEROSUIT_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn szerosuit_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    WorkModule::set_int(boma, 1, *FIGHTER_INSTANCE_WORK_ID_INT_AIR_LASSO_HANG_DATA);
}

//Zero Suit Samus Death Initialization
#[skyline::hook(offset = SZEROSUIT_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn szerosuit_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        szerosuit_start_initialization,
        szerosuit_reset_initialization,
        szerosuit_death_initialization
    );
}