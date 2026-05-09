use super::*;

const PFUSHIGISOU_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const PFUSHIGISOU_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xea03e0; //Ivysaur only

//Ivysaur Reset Initialization
#[skyline::hook(offset = PFUSHIGISOU_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pfushigisou_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_PFUSHIGISOU as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Ivysaur Death Initialization
#[skyline::hook(offset = PFUSHIGISOU_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pfushigisou_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        pfushigisou_reset_initialization,
        pfushigisou_death_initialization
    );
}