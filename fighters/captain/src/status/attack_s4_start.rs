use super::*;

//Forward Smash Start Pre Status
unsafe extern "C" fn captain_attack_s4_start_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION_RUN_STOP, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_START_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_START_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_START_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64, 0, 0, 0);
    0.into()
}

pub fn install() {
    Agent::new("captain")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_S4_START, captain_attack_s4_start_pre_status)
    .install()
    ;
}