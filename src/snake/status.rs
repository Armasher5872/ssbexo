use super::*;

unsafe extern "C" fn snake_forward_smash_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    original_status(Main, fighter, *FIGHTER_STATUS_KIND_ATTACK_S4)(fighter)
}

unsafe extern "C" fn snake_forward_smash_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE[entry_id] = false;
    SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[entry_id] = false;
    SNAKE_INT_ATTACK_S4_COMBO_COUNT[entry_id] = 0;
    original_status(End, fighter, *FIGHTER_STATUS_KIND_ATTACK_S4)(fighter)
}

unsafe extern "C" fn snake_special_hi_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn snake_special_hi_hang_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

pub fn install() {
    Agent::new("snake")
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4, snake_forward_smash_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_S4, snake_forward_smash_end_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, snake_special_hi_pre_status)
    .status(Pre, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_HI_HANG, snake_special_hi_hang_pre_status)
    .install()
    ;
}