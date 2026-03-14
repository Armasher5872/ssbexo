use super::*;

unsafe extern "C" fn luigi_catch_pull_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_CatchPull()
}

unsafe extern "C" fn luigi_catch_pull_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_CatchPull_common(hash40("catch_wait").into());
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_CatchPull_Main as *const () as _))
}

unsafe extern "C" fn luigi_catch_pull_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchPull()
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_CATCH_PULL, luigi_catch_pull_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_CATCH_PULL, luigi_catch_pull_main_status)
    .status(End, *FIGHTER_STATUS_KIND_CATCH_PULL, luigi_catch_pull_end_status)
    .install()
    ;
}