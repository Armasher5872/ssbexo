use super::*;

unsafe extern "C" fn ken_dash_back_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

pub fn install() {
    Agent::new("ken")
    .status(Main, *FIGHTER_RYU_STATUS_KIND_DASH_BACK, ken_dash_back_main_status)
    .install()
    ;
}