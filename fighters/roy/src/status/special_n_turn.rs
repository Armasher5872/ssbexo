use super::*;

unsafe extern "C" fn roy_special_n_turn_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("roy")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_TURN, roy_special_n_turn_end_status)
    .install()
    ;
}