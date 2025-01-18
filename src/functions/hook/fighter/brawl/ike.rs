use super::*;

const IKE_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xaf8010; //Ike only
const IKE_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const IKE_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xaf80b0; //Ike only

unsafe extern "C" fn ike_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_CAN_BOUND);
    WorkModule::off_flag(boma, *FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_N);
    WorkModule::set_float(boma, 0.0, *FIGHTER_IKE_INSTANCE_WORK_ID_FLOAT_BOUND_ANGLE);
    WorkModule::set_float(boma, 0.0, *FIGHTER_IKE_INSTANCE_WORK_ID_FLOAT_X_CHECK);
    WorkModule::set_float(boma, 0.0, *FIGHTER_IKE_INSTANCE_WORK_ID_FLOAT_Y_CHECK);
}

//Ike Startup Initialization
#[skyline::hook(offset = IKE_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ike_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    ike_var(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Ike Reset Initialization
#[skyline::hook(offset = IKE_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ike_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_IKE as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        ike_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Ike Death Initialization
#[skyline::hook(offset = IKE_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ike_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    ike_var(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        ike_start_initialization,
        ike_reset_initialization,
        ike_death_initialization
    );
}