use super::*;

const BUDDY_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x891260; //Banjo & Kazooie only
const BUDDY_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x891bf0; //Banjo & Kazooie only
const BUDDY_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x891270; //Banjo & Kazooie only

//Banjo & Kazooie Startup Initialization
#[skyline::hook(offset = BUDDY_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn buddy_start_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
}

//Banjo & Kazooie Reset Initialization
#[skyline::hook(offset = BUDDY_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn buddy_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Banjo & Kazooie Death Initialization
#[skyline::hook(offset = BUDDY_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn buddy_death_initialization(vtable: u64, fighter: &mut Fighter, param_3: u32) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter, param_3)
}

pub fn install() {
	skyline::install_hooks!(
        buddy_start_initialization,
        buddy_reset_initialization,
        buddy_death_initialization
    );
}