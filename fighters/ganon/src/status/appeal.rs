use super::*;

unsafe extern "C" fn ganon_appeal_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    STOP_SE(fighter, Hash40::new("se_ganon_appeal_s03"));
    fighter.status_end_Appeal()
}

unsafe extern "C" fn ganon_appeal_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    STOP_SE(fighter, Hash40::new("se_ganon_appeal_s03"));
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_STATUS_KIND_APPEAL, ganon_appeal_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_APPEAL, ganon_appeal_exit_status)
    .install()
    ;
}