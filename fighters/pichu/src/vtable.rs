use super::*;

const PICHU_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const PICHU_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xf2a530; //Shared

//Pichu Reset Initialization
#[skyline::hook(offset = PICHU_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pichu_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_PICHU as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Pichu Death Initialization
#[skyline::hook(offset = PICHU_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pichu_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_PICHU as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        pichu_reset_initialization,
        pichu_death_initialization
    );
}