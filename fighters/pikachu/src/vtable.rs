use super::*;

const PIKACHU_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xf2a520; //Shared
const PIKACHU_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const PIKACHU_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xf2a530; //Shared
const PIKACHU_VTABLE_ON_ATTACK_OFFSET: usize = 0xf2ae00; //Shared

unsafe extern "C" fn pikachu_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || is_damaged(fighter.module_accessor) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    }
    0.into()
}

unsafe extern "C" fn pikachu_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_PIKACHU_INSTANCE_WORK_ID_FLAG_ATTACK_11_DASH);
    WorkModule::set_int(boma, 0, *FIGHTER_PIKACHU_INSTANCE_WORK_ID_INT_ATTACK_11_COUNT);
}

//Pikachu Startup Initialization
#[skyline::hook(offset = PIKACHU_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pikachu_start_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_PIKACHU as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        pikachu_var(&mut *boma);
        agent.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(pikachu_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Pikachu Reset Initialization
#[skyline::hook(offset = PIKACHU_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pikachu_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_PIKACHU as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        pikachu_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Pikachu Death Initialization
#[skyline::hook(offset = PIKACHU_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pikachu_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_PIKACHU as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
        pikachu_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Pikachu On Attack
#[skyline::hook(offset = PIKACHU_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn pikachu_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let collision_log = log as *mut CollisionLogScuffed;
    let collision_kind = (*collision_log).collision_kind;
    let opponent_object_id = (*collision_log).opponent_object_id;
    let opponent_object = get_battle_object_from_id(opponent_object_id);
    let opponent_battle_object_id = (*opponent_object).battle_object_id;
    if [1, 2].contains(&collision_kind) {
        if opponent_battle_object_id >> 0x1C == 0 {
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK {
                WorkModule::inc_int(boma, *FIGHTER_PIKACHU_INSTANCE_WORK_ID_INT_ATTACK_11_COUNT);
            }
        }
    }
    call_original!(vtable, fighter, log)
}

pub fn install() {
    skyline::install_hooks!(
        pikachu_start_initialization,
        pikachu_reset_initialization,
        pikachu_death_initialization,
        pikachu_on_attack
    );
}