use super::*;

//Mii Gunner Reset Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_MIIGUNNER, 4, false, false))]
unsafe extern "C" fn miigunner_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Mii Gunner Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_MIIGUNNER, 7, false, false))]
unsafe extern "C" fn miigunner_death_initialization(vtable: u64, fighter: &mut Fighter, param_3: u32) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter, param_3)
}

pub fn install() {
    skyline::install_hooks!(
        miigunner_reset_initialization,
        miigunner_death_initialization
    );
}