use super::*;

const MIIGUNNER_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xd735a0; //Mii Gunner only
const MIIGUNNER_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xd738c0; //Mii Gunner only
const MIIGUNNER_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xd738e0; //Mii Gunner only

//Mii Gunner Startup Initialization
#[skyline::hook(offset = MIIGUNNER_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn miigunner_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Mii Gunner Reset Initialization
#[skyline::hook(offset = MIIGUNNER_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn miigunner_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Mii Gunner Death Initialization
#[skyline::hook(offset = MIIGUNNER_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn miigunner_death_initialization(vtable: u64, fighter: &mut Fighter, param_3: u32) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter, param_3)
}

pub fn install() {
    skyline::install_hooks!(
        miigunner_start_initialization,
        miigunner_reset_initialization,
        miigunner_death_initialization
    );
}