use super::*;

unsafe extern "C" fn dolly_dash_back_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

pub fn install() {
    Agent::new("dolly")
    .status(Main, *FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, dolly_dash_back_main_status)
    .install()
    ;
}