use super::*;

unsafe extern "C" fn cloud_escape_air_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind != *FIGHTER_STATUS_KIND_LANDING {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE);
    }
    fighter.status_end_EscapeAir()
}

pub fn install() {
    Agent::new("cloud")
    .status(End, *FIGHTER_STATUS_KIND_ESCAPE_AIR, cloud_escape_air_end_status)
    .install()
    ;
}