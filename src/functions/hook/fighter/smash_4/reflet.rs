use super::*;

const REFLET_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x10058d0; //Robin only
const REFLET_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const REFLET_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x1005b20; //Robin only

unsafe extern "C" fn reflet_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_AIR_FLIP);
    }
    0.into()
}

unsafe extern "C" fn reflet_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_ENHANCED_MAGIC_ACTIVE);
    WorkModule::set_int(boma, 0, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_ENHANCED_MAGIC_TIMER);
}

//Robin Startup Initialization
#[skyline::hook(offset = REFLET_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn reflet_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    reflet_var(&mut *boma);
    agent.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(should_use_special_hi_callback as *const () as _));
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(reflet_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Robin Reset Initialization
#[skyline::hook(offset = REFLET_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn reflet_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_REFLET as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        reflet_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Robin Death Initialization
#[skyline::hook(offset = REFLET_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn reflet_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    reflet_var(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x1005d30).nop(); //Starts the match with Levin Sword fully charged
    skyline::install_hooks!(
        reflet_start_initialization,
        reflet_reset_initialization,
        reflet_death_initialization
    );
}