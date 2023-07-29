use super::*;

/*   NEUTRAL SPECIAL STATUS SCRIPTS   */

#[status_script(agent = "littlemac", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn littlemac_special_n_main_status(fighter: &mut L2CFighterCommon) {
    fighter.change_status(FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2.into(), false.into())
}

pub fn install() {
    install_status_scripts!(
        littlemac_special_n_main_status
    );
}