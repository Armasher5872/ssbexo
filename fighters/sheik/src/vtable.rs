use super::*;

const SHEIK_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x1120c60; //Sheik only
const SHEIK_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x1120910; //Sheik only
const SHEIK_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x1120c70; //Sheik only
const SHEIK_VTABLE_ON_ATTACK_OFFSET: usize = 0x1121600; //Sheik only

unsafe extern "C" fn sheik_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_SPECIAL_S_OBJECT_ID);
}

unsafe extern "C" fn sheik_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || is_damaged(fighter.module_accessor) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_LW);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_GLIDE_TIMER);
    }
    0.into()
}

unsafe extern "C" fn sheik_should_use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_LW) {
        return 0.into();
    }
    1.into()
}

//Sheik Startup Initialization
#[skyline::hook(offset = SHEIK_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn sheik_start_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    sheik_var(&mut *boma);
    agent.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(sheik_should_use_special_lw_callback as *const () as _));
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(sheik_end_control as *const () as _));
}

//Sheik Reset Initialization
#[skyline::hook(offset = SHEIK_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn sheik_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    sheik_var(&mut *boma);
    original!()(vtable, fighter)
}

//Sheik Death Initialization
#[skyline::hook(offset = SHEIK_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn sheik_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    sheik_var(&mut *boma);
    original!()(vtable, fighter)
}

//Sheik On Attack
#[skyline::hook(offset = SHEIK_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn sheik_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let get_attack_air_kind = ControlModule::get_attack_air_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    let needle_count = WorkModule::get_int(boma, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_NEEDLE_COUNT);
    if needle_count == 6 {
        if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_HI4].contains(&status_kind) {
            sheik_remove_needles(boma);
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            if get_attack_air_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_F || motion_kind == hash40("attack_air_f") {
                sheik_remove_needles(boma);
            }
        }
    }
    call_original!(vtable, fighter, log)
}

pub fn install() {
	skyline::install_hooks!(
        sheik_start_initialization,
        sheik_reset_initialization,
        sheik_death_initialization,
        sheik_on_attack
    );
}