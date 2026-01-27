use super::*;

unsafe extern "C" fn sonic_special_n_cancel_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 120, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_N_COOLDOWN_TIMER);
    0.into()
}

unsafe extern "C" fn sonic_special_n_cancel_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 120, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_N_COOLDOWN_TIMER);
    0.into()
}

pub fn install() {
    Agent::new("sonic")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_CANCEL, sonic_special_n_cancel_end_status)
    .status(Exit, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_CANCEL, sonic_special_n_cancel_exit_status)
    .install()
    ;
}