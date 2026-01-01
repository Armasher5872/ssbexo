use super::*;

const MARIO_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xcb9620; //Mario only
const MARIO_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const MARIO_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xcb9730; //Mario only
const MARIO_VTABLE_ON_ATTACK_OFFSET: usize = 0x68d7e0; //Shared

unsafe extern "C" fn mario_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || is_damaged(fighter.module_accessor) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    }
    0.into()
}

unsafe extern "C" fn mario_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ATTACK_ACTIVE);
    WorkModule::off_flag(boma, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HIT);
    WorkModule::off_flag(boma, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP);
    WorkModule::set_float(boma, 0.0, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_ATTACK_FRAME);
    WorkModule::set_float(boma, 0.0, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_DEGREE);
    WorkModule::set_int(boma, 0, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_N_HELD_FRAME);
}

//Mario Startup Initialization
#[skyline::hook(offset = MARIO_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn mario_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    mario_var(&mut *boma);
    agent.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    agent.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(should_use_special_lw_callback as *const () as _));
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(mario_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Mario Reset Initialization
#[skyline::hook(offset = MARIO_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn mario_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_MARIO as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        mario_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Mario Death Initialization
#[skyline::hook(offset = MARIO_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn mario_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    mario_var(&mut *boma);
    original!()(vtable, fighter)
}

//Mario On Attack
#[skyline::hook(offset = MARIO_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn mario_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_MARIO as u32 {
        let boma = fighter.battle_object.module_accessor;
        let status_kind = StatusModule::status_kind(boma);
        if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_S_LOOP].contains(&status_kind) {
            WorkModule::on_flag(boma, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HIT);
        }
    }
    call_original!(vtable, fighter, log)
}

pub fn install() {
	skyline::install_hooks!(
        mario_start_initialization,
        mario_reset_initialization,
        mario_death_initialization,
        mario_on_attack
    );
}