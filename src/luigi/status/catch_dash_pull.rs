use super::*;

unsafe extern "C" fn luigi_catch_dash_pull_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_CatchDashPull()
}

unsafe extern "C" fn luigi_catch_dash_pull_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchDashPull()
}

pub fn install() {
    Agent::new("luigi")
    .status(Main, *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, luigi_catch_dash_pull_main_status)
    .status(End, *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, luigi_catch_dash_pull_end_status)
    .install()
    ;
}