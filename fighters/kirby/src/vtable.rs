use super::*;

const KIRBY_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xb96100; //Kirby only
const KIRBY_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xb96350; //Kirby only
const KIRBY_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xb96f70; //Kirby only

unsafe extern "C" fn kirby_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INPUT);
    WorkModule::set_int(boma, -1, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_TEAM_NO);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
}

unsafe extern "C" fn kirby_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    }
    0.into()
}

//Kirby Startup Initialization
#[skyline::hook(offset = KIRBY_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn kirby_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    kirby_var(&mut *boma);
    agent.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(kirby_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Kirby Reset Initialization
#[skyline::hook(offset = KIRBY_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn kirby_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    kirby_var(&mut *boma);
    original!()(vtable, fighter)
}

//Kirby Death Initialization
#[skyline::hook(offset = KIRBY_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn kirby_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    kirby_var(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        kirby_start_initialization,
        kirby_reset_initialization,
        kirby_death_initialization
    );
}