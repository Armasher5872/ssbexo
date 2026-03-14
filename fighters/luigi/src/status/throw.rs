use super::*;

unsafe extern "C" fn luigi_throw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Throw()
}

unsafe extern "C" fn luigi_throw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Throw()
}

unsafe extern "C" fn luigi_throw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Throw()
}

unsafe extern "C" fn luigi_throw_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_throw_uniq_process_exit();
    0.into()
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_THROW, luigi_throw_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_THROW, luigi_throw_main_status)
    .status(End, *FIGHTER_STATUS_KIND_THROW, luigi_throw_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_THROW, luigi_throw_exit_status)
    .install()
    ;
}