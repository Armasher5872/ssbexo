use super::*;

unsafe extern "C" fn shizue_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    fun_71000233d0(fighter);
    0.into()
}

unsafe extern "C" fn fun_71000233d0(fighter: &mut L2CFighterCommon) {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_FLAG_SHOOT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_FLAG_HIT);
    WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_INT_TARGET_OBJECT_CATEGORY);
    WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_INT_TARGET_OBJECT_ID);
}

unsafe extern "C" fn shizue_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

pub fn install() {
    Agent::new("shizue")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, shizue_special_s_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, shizue_special_lw_pre_status)
    .install()
    ;
}