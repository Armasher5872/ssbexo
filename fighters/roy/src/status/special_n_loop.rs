use super::*;

unsafe extern "C" fn roy_special_n_loop_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn roy_special_n_loop_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("roy")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_LOOP, roy_special_n_loop_end_status)
    .status(Exit, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_LOOP, roy_special_n_loop_exit_status)
    .install()
    ;
}