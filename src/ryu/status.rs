use super::*;

unsafe extern "C" fn ryu_dash_back_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

pub fn install() {
    Agent::new("ryu")
    .status(Main, *FIGHTER_RYU_STATUS_KIND_DASH_BACK, ryu_dash_back_main_status)
    .install()
    ;
}