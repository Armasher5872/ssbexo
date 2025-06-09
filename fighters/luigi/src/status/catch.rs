use super::*;

unsafe extern "C" fn luigi_catch_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Catch()
}

unsafe extern "C" fn luigi_catch_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Catch()
}

unsafe extern "C" fn luigi_catch_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Catch()
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_CATCH, luigi_catch_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_CATCH, luigi_catch_main_status)
    .status(End, *FIGHTER_STATUS_KIND_CATCH, luigi_catch_end_status)
    .install()
    ;
}