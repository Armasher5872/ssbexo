use super::*;

unsafe extern "C" fn cloud_guard_on_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
        fighter.change_status(FIGHTER_CLOUD_STATUS_KIND_GUARD_ON.into(), true.into());
    }
    original_status(Exec, fighter, *FIGHTER_STATUS_KIND_GUARD_ON)(fighter)
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Exec, *FIGHTER_STATUS_KIND_GUARD_ON, cloud_guard_on_exec_status)
    .install()
    ;
}