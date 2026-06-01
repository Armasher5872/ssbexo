use super::*;

unsafe extern "C" fn ken_attack_dash_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let attack_kind = WorkModule::get_int64(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if 0 < attack_kind {
        FighterStatusModuleImpl::reset_log_action_info(boma, attack_kind);
        WorkModule::set_int64(boma, 0i64, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    EFFECT_OFF_KIND(fighter, Hash40::new("ken_syoryuken_fire"), true, true);
    0.into()
}

pub fn install() {
    Agent::new("ken")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_DASH, ken_attack_dash_end_status)
    .install()
    ;
}