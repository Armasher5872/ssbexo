use super::*;

const WOLF_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xa61650; //Shared
const WOLF_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xa617c0; //Shared
const WOLF_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xa62210; //Shared
const WOLF_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xa62480; //Shared

unsafe extern "C" fn wolf_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    }
    0.into()
}

//Wolf Startup Initialization
#[skyline::hook(offset = WOLF_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn wolf_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_WOLF as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        agent.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
        agent.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(should_use_special_hi_callback as *const () as _));
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(wolf_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Wolf Reset Initialization
#[skyline::hook(offset = WOLF_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn wolf_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_WOLF as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Wolf Death Initialization
#[skyline::hook(offset = WOLF_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn wolf_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_WOLF as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Wolf Once Per Fighter Frame
#[skyline::hook(offset = WOLF_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn wolf_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    if fighter.battle_object.kind == *FIGHTER_KIND_WOLF as u32 {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
            WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && LAST_ATTACK_HITBOX_ID == 0 {
                CancelModule::enable_cancel(boma);
            }
        }
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        wolf_start_initialization,
        wolf_reset_initialization,
        wolf_death_initialization,
        wolf_opff
    );
}