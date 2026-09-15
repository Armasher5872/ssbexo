use super::*;

unsafe extern "C" fn sonic_dash_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Dash()
}

pub fn install() {
    Agent::new("sonic")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_DASH, sonic_dash_status)
    .install()
    ;
}