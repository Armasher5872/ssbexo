use super::*;

unsafe extern "C" fn cloud_squat_rv_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {"punish_squat_rv"} else {"squat_rv"};
    fighter.status_SquatRv_param(FIGHTER_STATUS_KIND_SQUAT.into(), FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), hash40(motion).into());
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_SquatRv_Main as *const () as _))
}

pub fn install() {
    Agent::new("cloud")
    .status(Main, *FIGHTER_STATUS_KIND_SQUAT_RV, cloud_squat_rv_main_status)
    .install()
    ;
}