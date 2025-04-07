use super::*;

unsafe extern "C" fn cloud_dash_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    cloud_status_dash_sub(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Dash_Main as *const () as _))
}

unsafe extern "C" fn cloud_status_dash_sub(fighter: &mut L2CFighterCommon) {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let value: f32 = if [*FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT].contains(&prev_status_kind) {6.0} else {0.0};
    let motion = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {"punish_dash"} else {"dash"};
    MotionModule::change_motion(fighter.module_accessor, Hash40::new(motion), 0.0, 1.0, false, value, false, false);
    fighter.status_DashCommon();
}

pub fn install() {
    Agent::new("cloud")
    .status(Main, *FIGHTER_STATUS_KIND_DASH, cloud_dash_main_status)
    .install()
    ;
}