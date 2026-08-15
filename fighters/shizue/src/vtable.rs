use super::*;

const SHIZUE_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x114c130; //Isabelle Only

//Isabelle Death Initialization
#[skyline::hook(offset = SHIZUE_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn shizue_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hook!(shizue_death_initialization);
}