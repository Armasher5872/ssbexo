use super::*;

unsafe extern "C" fn yoshi_guard_off_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_GuardOff()
}

pub fn install() {
    Agent::new("yoshi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_GUARD_OFF, yoshi_guard_off_main_status)
    .install()
    ;
}