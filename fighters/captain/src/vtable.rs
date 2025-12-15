use super::*;

const CAPTAIN_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x8b7ce0; //Captain Falcon only
const CAPTAIN_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x8b7610; //Captain Falcon only
const CAPTAIN_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x8b7cf0; //Captain Falcon only
const CAPTAIN_VTABLE_ON_ATTACK_OFFSET: usize = 0x8b8b90; //Captain Falcon only

unsafe extern "C" fn captain_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_JUMP);
}

//Captain Falcon Startup Initialization
#[skyline::hook(offset = CAPTAIN_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn captain_start_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    captain_var(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
}

//Captain Falcon Reset Initialization
#[skyline::hook(offset = CAPTAIN_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn captain_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    captain_var(&mut *boma);
    original!()(vtable, fighter)
}

//Captain Falcon Death Initialization
#[skyline::hook(offset = CAPTAIN_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn captain_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    captain_var(&mut *boma);
    original!()(vtable, fighter)
}

//Captain Falcon On Attack
#[skyline::hook(offset = CAPTAIN_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn captain_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let status_kind = agent.global_table[STATUS_KIND].get_i32();
    let situation_kind = agent.global_table[SITUATION_KIND].get_i32();
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW && situation_kind == *SITUATION_KIND_AIR {
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_JUMP);
        StatusModule::change_status_request(boma, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_WALL_END, false);
    }
    call_original!(vtable, fighter, log)
}

pub fn install() {
	skyline::install_hooks!(
        captain_start_initialization,
        captain_reset_initialization,
        captain_death_initialization,
        captain_on_attack
    );
}