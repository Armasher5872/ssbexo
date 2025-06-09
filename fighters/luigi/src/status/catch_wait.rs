use super::*;

unsafe extern "C" fn luigi_catch_wait_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH) {
        fighter.status_CatchWait_common(hash40("special_lw_catch_pull").into())
    }
    else {
        fighter.status_CatchWait_common(hash40("catch_pull").into())
    }
}

unsafe extern "C" fn luigi_catch_wait_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchWait()
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_CATCH_WAIT, luigi_catch_wait_main_status)
    .status(End, *FIGHTER_STATUS_KIND_CATCH_WAIT, luigi_catch_wait_end_status)
    .install()
    ;
}