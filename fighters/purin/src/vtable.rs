use super::*;

const PURIN_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const PURIN_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xfdf980; //Jigglypuff only

//Jigglypuff Reset Initialization
#[skyline::hook(offset = PURIN_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn purin_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_PURIN as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Jigglypuff Death Initialization
#[skyline::hook(offset = PURIN_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn purin_death_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
}

pub fn install() {
	skyline::install_hooks!(
        purin_reset_initialization,
        purin_death_initialization
    );
}