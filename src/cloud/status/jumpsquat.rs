use super::*;

unsafe extern "C" fn cloud_jumpsquat_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {"punish_jump_squat"} else {"jump_squat"};
    fighter.sub_jump_squat_uniq_process_init_param(hash40(motion).into());
    0.into()
}

pub fn install() {
    Agent::new("cloud")
    .status(Init, *FIGHTER_STATUS_KIND_JUMP_SQUAT, cloud_jumpsquat_init_status)
    .install()
    ;
}