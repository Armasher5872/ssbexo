use super::*;

unsafe extern "C" fn cloud_jumpsquat_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let motion = if WorkModule::is_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {"punish_jump_squat"} else {"jump_squat"};
    fighter.sub_jump_squat_uniq_process_init_param(hash40(motion).into());
    0.into()
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Init, *FIGHTER_STATUS_KIND_JUMP_SQUAT, cloud_jumpsquat_init_status)
    .install()
    ;
}