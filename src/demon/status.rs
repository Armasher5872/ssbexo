use super::*;

unsafe extern "C" fn demon_dash_back_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

pub fn install() {
    Agent::new("demon")
    .status(Main, *FIGHTER_DEMON_STATUS_KIND_DASH_BACK, demon_dash_back_main_status)
    .install()
    ;
}