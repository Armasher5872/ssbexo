use super::*;

//Punisher Squat Rv Sound
unsafe extern "C" fn ssbexo_cloud_punisher_squat_rv_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_rise"));
    }
}

//Punisher Squat Rv Expression
unsafe extern "C" fn ssbexo_cloud_punisher_squat_rv_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_r") as i64);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("sound_punishsquatrv", ssbexo_cloud_punisher_squat_rv_sound, Low)
    .acmd("expression_punishsquatrv", ssbexo_cloud_punisher_squat_rv_expression, Low)
    .install()
    ;
}