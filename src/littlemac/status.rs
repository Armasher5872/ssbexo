use super::*;

/*   NEUTRAL SPECIAL STATUS SCRIPTS   */

unsafe extern "C" fn littlemac_special_n_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_status(FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2.into(), false.into());
    0.into()
}

pub fn install() {
    Agent::new("littlemac")
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, littlemac_special_n_main_status)
    .install()
    ;
}