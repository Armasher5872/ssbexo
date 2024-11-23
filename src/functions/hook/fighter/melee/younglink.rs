use super::*;

const YOUNGLINK_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xc280f0; //Shared
const YOUNGLINK_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xc28280; //Shared
const YOUNGLINK_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xc28860; //Shared

//Young Link Startup Initialization
#[skyline::hook(offset = YOUNGLINK_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn younglink_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_YOUNGLINK as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_initialization_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Young Link Reset Initialization
#[skyline::hook(offset = YOUNGLINK_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn younglink_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_YOUNGLINK as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Young Link Death Initialization
#[skyline::hook(offset = YOUNGLINK_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn younglink_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_YOUNGLINK as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        younglink_start_initialization,
        younglink_reset_initialization,
        younglink_death_initialization
    );
}