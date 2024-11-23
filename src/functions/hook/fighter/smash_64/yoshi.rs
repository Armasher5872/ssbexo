use super::*;

const YOSHI_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x12dbb90; //Yoshi only
const YOSHI_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x12db8f0; //Yoshi only
const YOSHI_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x12dbc20; //Yoshi only

//Yoshi Startup Initialization
#[skyline::hook(offset = YOSHI_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn yoshi_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Yoshi Reset Initialization
#[skyline::hook(offset = YOSHI_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn yoshi_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Yoshi Death Initialization
#[skyline::hook(offset = YOSHI_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn yoshi_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        yoshi_start_initialization,
        yoshi_reset_initialization,
        yoshi_death_initialization
    );
}