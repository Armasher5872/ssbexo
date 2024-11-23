use super::*;

const GAOGAEN_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xab7cb0; //Incineroar only
const GAOGAEN_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xab7cc0; //Incineroar only
const GAOGAEN_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xab8390; //Incineroar only

unsafe extern "C" fn gaogaen_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_CAN_ANGLE_CATCH);
    WorkModule::off_flag(boma, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_THROW_CRITICAL_ZOOM);
    WorkModule::off_flag(boma, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_SPECIAL_S_CANCEL);
    WorkModule::off_flag(boma, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_SPECIAL_S_FAKE);
}

//Incineroar Startup Initialization
#[skyline::hook(offset = GAOGAEN_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn gaogaen_start_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    gaogaen_var(&mut *boma);
}

//Incineroar Reset Initialization
#[skyline::hook(offset = GAOGAEN_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn gaogaen_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    gaogaen_var(&mut *boma);
    original!()(vtable, fighter)
}

//Incineroar Death Initialization
#[skyline::hook(offset = GAOGAEN_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn gaogaen_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    gaogaen_var(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        gaogaen_start_initialization,
        gaogaen_reset_initialization,
        gaogaen_death_initialization
    );
}