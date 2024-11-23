use super::*;

const ROBOT_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x105bce0; //R.O.B only
const ROBOT_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x105bf20; //R.O.B only
const ROBOT_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x105bfa0; //R.O.B only
const ROBOT_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x105c7f0; //R.O.B only

unsafe extern "C" fn robot_var(boma: &mut BattleObjectModuleAccessor) {
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    WorkModule::set_float(boma, 0.0, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_SNAKE_SPEED_VALUE);
    WorkModule::off_flag(boma, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_POWER_BOOST);
    WorkModule::off_flag(boma, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_SNAKE);
    WorkModule::off_flag(boma, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST);
    WorkModule::off_flag(boma, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_EARLY_CANCEL);
    UiManager::set_robot_meter_info(entry_id, 160.0, 160.0, 80.0);
}

//R.O.B Startup Initialization
#[skyline::hook(offset = ROBOT_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn robot_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    robot_var(&mut *boma);
    original!()(vtable, fighter)
}

//R.O.B Reset Initialization
#[skyline::hook(offset = ROBOT_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn robot_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    robot_var(&mut *boma);
    original!()(vtable, fighter)
}

//R.O.B Death Initialization
#[skyline::hook(offset = ROBOT_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn robot_death_initialization(vtable: u64, fighter: &mut Fighter, param_3: u32) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    robot_var(&mut *boma);
    original!()(vtable, fighter, param_3)
}

//R.O.B Once Per Fighter Frame
#[skyline::hook(offset = ROBOT_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn robot_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    let burner_energy_value = WorkModule::get_float(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
    UiManager::set_robot_meter_info(entry_id, burner_energy_value, 160.0, 80.0);
    if burner_energy_value > 108.0 {
        UiManager::change_robot_meter_color_blue(entry_id);
    }
    else if burner_energy_value < 54.0 {
        UiManager::change_robot_meter_color_red(entry_id);
    }
    else {
        UiManager::change_robot_meter_color_yellow(entry_id);
    }
    UiManager::set_robot_meter_enable(entry_id, true);
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        robot_start_initialization,
        robot_reset_initialization,
        robot_death_initialization,
        robot_opff
    );
}