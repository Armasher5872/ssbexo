use super::*;

unsafe extern "C" fn ken_guard_off_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_GuardOff()
}

pub fn install() {
    Agent::new("ken")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_GUARD_OFF, ken_guard_off_main_status)
    .install()
    ;
}