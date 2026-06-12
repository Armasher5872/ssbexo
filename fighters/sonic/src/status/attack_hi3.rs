use super::*;

unsafe extern "C" fn sonic_attack_hi3_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let attack_kind = WorkModule::get_int64(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if 0 < attack_kind {
        FighterStatusModuleImpl::reset_log_action_info(boma, attack_kind);
        WorkModule::set_int(boma, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    WorkModule::off_flag(boma, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_PHANTOM_BOOSTED_MOTION_RATE);
    0.into()
}

pub fn install() {
    Agent::new("sonic")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_HI3, sonic_attack_hi3_end_status)
    .install()
    ;
}