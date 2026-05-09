use super::*;

const SAMUSD_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x10f3630; //Shared
const SAMUSD_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x10f3650; //Shared

//Dark Samus Reset Initialization
#[skyline::hook(offset = SAMUSD_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn samusd_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_SAMUSD as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        WorkModule::set_int(boma, 0, *FIGHTER_SAMUSD_INSTANCE_WORK_ID_INT_FLOAT_TIME);
    }
    original!()(vtable, fighter)
}

//Dark Samus Death Initialization
#[skyline::hook(offset = SAMUSD_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn samusd_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_SAMUSD as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
        WorkModule::set_int(boma, 0, *FIGHTER_SAMUSD_INSTANCE_WORK_ID_INT_FLOAT_TIME);
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        samusd_reset_initialization,
        samusd_death_initialization
    );
}