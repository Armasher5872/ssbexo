use super::*;

unsafe extern "C" fn marth_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let long_sword_scale = Vector3f{x: 1.0, y: 1.2, z: 1.0};
    ModelModule::set_joint_scale(boma, Hash40::new("havel"), &long_sword_scale);
    ModelModule::set_joint_scale(boma, Hash40::new("haver"), &long_sword_scale);
}

pub fn install() {
    Agent::new("marth")
    .on_line(Main, marth_frame)
    .install()
    ;
}