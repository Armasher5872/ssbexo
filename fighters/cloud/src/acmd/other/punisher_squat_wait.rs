use super::*;

//Punisher Squat Wait Sound
unsafe extern "C" fn ssbexo_cloud_punisher_squat_wait_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 3.0);
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_SQUAT_FLAG_REQUEST_SQUAT_SE) {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_cloud_squat"));
            WorkModule::off_flag(boma, *FIGHTER_STATUS_SQUAT_FLAG_REQUEST_SQUAT_SE);
        }
    }
}

//Punisher Squat Wait Expression
unsafe extern "C" fn ssbexo_cloud_punisher_squat_wait_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_r") as i64);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("sound_punishsquatwait1", ssbexo_cloud_punisher_squat_wait_sound, Low)
    .acmd("expression_punishsquatwait1", ssbexo_cloud_punisher_squat_wait_expression, Low)
    .install()
    ;
}