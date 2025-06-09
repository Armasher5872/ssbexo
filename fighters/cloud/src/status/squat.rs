use super::*;

unsafe extern "C" fn cloud_squat_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    cloud_status_squat_sub(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Squat_Main as *const () as _))
}

unsafe extern "C" fn cloud_status_squat_sub(fighter: &mut L2CFighterCommon) {
    let motion = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {"punish_squat"} else {"squat"};
    fighter.status_Squat_sub_param(hash40(motion).into(), FIGHTER_STATUS_KIND_SQUAT.into(), FIGHTER_STATUS_KIND_SQUAT_WAIT.into());
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_SQUAT, cloud_squat_main_status)
    .install()
    ;
}