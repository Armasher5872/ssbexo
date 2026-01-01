use super::*;

const KROOL_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xc02400; //King K Rool only
const KROOL_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xc026a0; //King K Rool only
const KROOL_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xc04290; //King K Rool only

unsafe extern "C" fn krool_var(boma: *mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP);
    WorkModule::set_float(boma, 0.0, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_CHARGE);
    WorkModule::set_int(boma, 0, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_LW_ANGLE);
}

unsafe extern "C" fn krool_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || is_damaged(fighter.module_accessor) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    }
    0.into()
}

//King K Rool Startup Initialization
#[skyline::hook(offset = KROOL_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn krool_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    krool_var(boma);
    agent.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(should_use_special_lw_callback as *const () as _));
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(krool_end_control as *const () as _));
    original!()(vtable, fighter)
}

//King K Rool Reset Initialization
#[skyline::hook(offset = KROOL_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn krool_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    krool_var(boma);
    original!()(vtable, fighter)
}

//King K Rool Death Initialization
#[skyline::hook(offset = KROOL_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn krool_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    krool_var(boma);
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        krool_start_initialization,
        krool_reset_initialization,
        krool_death_initialization
    );
}