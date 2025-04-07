use super::*;

unsafe extern "C" fn cloud_escape_b_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE);
    fighter.sub_status_end_EscaleFB();
    0.into()
}

pub fn install() {
    Agent::new("cloud")
    .status(End, *FIGHTER_STATUS_KIND_ESCAPE_B, cloud_escape_b_end_status)
    .install()
    ;
}