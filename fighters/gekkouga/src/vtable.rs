use super::*;

const GEKKOUGA_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const GEKKOUGA_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xadaf50; //Greninja only

//Greninja Reset Initialization
#[skyline::hook(offset = GEKKOUGA_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn gekkouga_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_GEKKOUGA as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        gekkouga_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Greninja Death Initialization
#[skyline::hook(offset = GEKKOUGA_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn gekkouga_death_initialization(vtable: u64, fighter: &mut Fighter, param_3: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    gekkouga_var(&mut *boma);
    original!()(vtable, fighter, param_3)
}

pub fn install() {
    skyline::install_hooks!(
        gekkouga_reset_initialization,
        gekkouga_death_initialization
    );
}