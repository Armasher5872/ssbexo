use super::*;

const ROY_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x10bb480; //Shared
const ROY_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const ROY_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x10bb700; //Shared

unsafe extern "C" fn roy_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || is_damaged(fighter.module_accessor) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    }
    0.into()
}

//Roy Startup Initialization
#[skyline::hook(offset = ROY_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn roy_start_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_ROY as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        agent.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(roy_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Roy Reset Initialization
#[skyline::hook(offset = ROY_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn roy_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_ROY as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Roy Death Initialization
#[skyline::hook(offset = ROY_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn roy_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_ROY as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        roy_start_initialization,
        roy_reset_initialization,
        roy_death_initialization
    );
}