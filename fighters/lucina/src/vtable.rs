use super::*;

const LUCINA_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xcd98a0; //Shared
const LUCINA_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xc732a0; //Shared
const LUCINA_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xcd99a0; //Shared
const LUCINA_VTABLE_ON_SEARCH_EVENT_OFFSET: usize = 0x68d8a0; //Shared

unsafe extern "C" fn lucina_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    }
    0.into()
}

unsafe extern "C" fn lucina_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_LUCINA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_VIS_OFF);
    WorkModule::off_flag(boma, *FIGHTER_LUCINA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_VIS_ON);
    WorkModule::off_flag(boma, *FIGHTER_LUCINA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_GRAVITY_ON);
    WorkModule::off_flag(boma, *FIGHTER_LUCINA_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INPUT_ON);
    WorkModule::set_float(boma, 1.0, *FIGHTER_LUCINA_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_POWER);
    WorkModule::set_int(boma, 0, *FIGHTER_LUCINA_INSTANCE_WORK_ID_INT_SPECIAL_LW_EFFECT_COUNT);
}

//Lucina Startup Initialization
#[skyline::hook(offset = LUCINA_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucina_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_LUCINA as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        lucina_var(&mut *boma);
        agent.global_table[CHECK_SPECIAL_N_UNIQ].assign(&L2CValue::Ptr(should_use_special_n_callback as *const () as _));
        agent.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
        agent.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(should_use_special_hi_callback as *const () as _));
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(lucina_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Lucina Reset Initialization
#[skyline::hook(offset = LUCINA_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucina_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_LUCINA as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        lucina_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Lucina Death Initialization
#[skyline::hook(offset = LUCINA_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn lucina_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_LUCINA as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
        lucina_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Lucina On Search
#[skyline::hook(offset = LUCINA_VTABLE_ON_SEARCH_EVENT_OFFSET)]
unsafe extern "C" fn lucina_on_search(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_LUCINA as u32 {
        let boma = fighter.battle_object.module_accessor;
        let collision_log = *(log as *const u64).add(0x10/0x8);
        let collision_log = collision_log as *const CollisionLog;
        let status_kind = StatusModule::status_kind(boma);
        let pos = *PostureModule::pos(boma);
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            let opponent_id = (*collision_log).opponent_battle_object_id;
            if opponent_id != *BATTLE_OBJECT_ID_INVALID as u32 {
                let opponent_boma = sv_battle_object::module_accessor(opponent_id as u32);
                PostureModule::set_pos(opponent_boma, &Vector3f{x: pos.x+(15.0*PostureModule::lr(boma)), y: pos.y, z: pos.z});
            }
        }
    }
    original!()(vtable, fighter, log)
}

pub fn install() {
	skyline::install_hooks!(
        lucina_start_initialization,
        lucina_reset_initialization,
        lucina_death_initialization,
        lucina_on_search
    );
}