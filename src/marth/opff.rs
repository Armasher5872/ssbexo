use super::*;

unsafe extern "C" fn marth_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let motion_kind = MotionModule::motion_kind(boma);
    let frame = MotionModule::frame(boma);
    let long_sword_scale = Vector3f{x: 1.0, y: 1.2, z: 1.0};
    ModelModule::set_joint_scale(boma, Hash40::new("havel"), &long_sword_scale);
    ModelModule::set_joint_scale(boma, Hash40::new("haver"), &long_sword_scale);
    if [hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind)
    && frame > 28.0 {
        WorkModule::off_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        StatusModule::change_status_request_from_script(boma, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, true);
    }
}

pub fn install() {
    Agent::new("marth")
    .on_line(Main, marth_frame)
    .install()
    ;
}