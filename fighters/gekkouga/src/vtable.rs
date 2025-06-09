use super::*;

const GEKKOUGA_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xadac40; //Greninja only
const GEKKOUGA_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const GEKKOUGA_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xadaf50; //Greninja only
const GEKKOUGA_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xadca30; //Greninja only
const GEKKOUGA_VTABLE_ON_SEARCH_EVENT_OFFSET: usize = 0x68d8a0; //Shared

unsafe extern "C" fn gekkouga_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    }
    0.into()
}

unsafe extern "C" fn gekkouga_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_DOLL_LAUNCH);
    WorkModule::off_flag(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_DOLL_LINK);
    WorkModule::off_flag(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_FOUND_DOLL);
    WorkModule::off_flag(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_VERTICAL);
    WorkModule::off_flag(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_HI);
}

//Greninja Startup Initialization
#[skyline::hook(offset = GEKKOUGA_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn gekkouga_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    gekkouga_var(&mut *boma);
    agent.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(gekkouga_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Greninja Reset Initialization
#[skyline::hook(offset = GEKKOUGA_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn gekkouga_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_GEKKOUGA as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        gekkouga_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Greninja Death Initialization
#[skyline::hook(offset = GEKKOUGA_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn gekkouga_death_initialization(vtable: u64, fighter: &mut Fighter, param_3: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    gekkouga_var(&mut *boma);
    original!()(vtable, fighter, param_3)
}

//Greninja Once Per Fighter Frame
#[skyline::hook(offset = GEKKOUGA_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn gekkouga_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE) {
        WorkModule::off_flag(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_START_HOLD);
    }
    original!()(vtable, fighter)
}

//Greninja On Search
#[skyline::hook(offset = GEKKOUGA_VTABLE_ON_SEARCH_EVENT_OFFSET)]
unsafe extern "C" fn gekkouga_on_search(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_GEKKOUGA as u32 {
        let boma = fighter.battle_object.module_accessor;
        let collision_log = *(log as *const u64).add(0x10/0x8);
        let collision_log = collision_log as *const CollisionLog;
        let status_kind = StatusModule::status_kind(boma);
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            let opponent_id = (*collision_log).opponent_battle_object_id;
            let doll_id = WorkModule::get_int(boma, 0x100000C2);
            if opponent_id == doll_id as u32 {
                WorkModule::on_flag(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_FOUND_DOLL);
            }
        }
    }
    original!()(vtable, fighter, log)
}

pub fn install() {
    skyline::install_hooks!(
        gekkouga_start_initialization,
        gekkouga_reset_initialization,
        gekkouga_death_initialization,
        gekkouga_opff,
        gekkouga_on_search
    );
}