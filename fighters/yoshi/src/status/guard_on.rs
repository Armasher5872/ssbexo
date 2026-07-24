use super::*;

unsafe extern "C" fn yoshi_guard_on_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let shield_radius = WorkModule::get_param_float(boma, hash40("shield_radius"), 0);
    let throw_scale = Vector3f{x: shield_radius, y: shield_radius, z: shield_radius};
    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION);
    fighter.sub_status_guard_on_common();
    WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION);
    ModelModule::set_joint_scale(boma, Hash40::new("throw"), &throw_scale);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2dc1210b69));
    fighter.sub_shift_status_main(L2CValue::Ptr(yoshi_guard_on_main_loop as *const () as _))
}

unsafe extern "C" fn yoshi_guard_on_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_GuardOn_Main()
}

pub fn install() {
    Agent::new("yoshi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_GUARD_ON, yoshi_guard_on_main_status)
    .install()
    ;
}