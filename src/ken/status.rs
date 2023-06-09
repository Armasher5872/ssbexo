use super::*;

#[status_script(agent = "ken", status = FIGHTER_RYU_STATUS_KIND_DASH_BACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ken_dash_back_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

pub fn install() {
    install_status_scripts!(
        ken_dash_back_main_status
    );
}