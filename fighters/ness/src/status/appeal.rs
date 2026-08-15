use super::*;

unsafe extern "C" fn ness_appeal_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    let log_attack_kind = WorkModule::get_int64(boma, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if status_kind != *FIGHTER_STATUS_KIND_SMASH_APPEAL {
        if 0 < log_attack_kind {
            FighterStatusModuleImpl::reset_log_action_info(boma, log_attack_kind);
            WorkModule::set_int(boma, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
        CANCEL_FILL_SCREEN(fighter, 1, 4);
        CANCEL_FILL_SCREEN(fighter, 2, 4);
    }
    0.into()
}

pub fn install() {
    Agent::new("ness")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_STATUS_KIND_APPEAL, ness_appeal_end_status)
    .install()
    ;
}