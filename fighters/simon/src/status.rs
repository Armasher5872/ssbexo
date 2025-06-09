use super::*;

unsafe extern "C" fn simon_attack_air_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_uniq_process_exec()
}

unsafe extern "C" fn simon_attack_air_check_attack_status(_fighter: &mut L2CFighterCommon, _param_2: &L2CValue, _param_3: &L2CValue) -> L2CValue {
    0.into()
}

unsafe extern "C" fn simon_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_SOUND);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

pub fn install() {
    Agent::new("simon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Exec, *FIGHTER_STATUS_KIND_ATTACK_AIR, simon_attack_air_exec_status)
    .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_AIR, simon_attack_air_check_attack_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, simon_special_lw_pre_status)
    .install()
    ;
}