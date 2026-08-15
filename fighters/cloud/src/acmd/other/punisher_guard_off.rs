use super::*;

//Punisher Guard Off Sound
unsafe extern "C" fn ssbexo_cloud_punisher_guard_off_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_guardoff"));
    }
}

//Punisher Guard Off Expression
unsafe extern "C" fn ssbexo_cloud_punisher_guard_off_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_r") as i64);
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("sound_punishguardoff", ssbexo_cloud_punisher_guard_off_sound, Low)
    .acmd("expression_punishguardoff", ssbexo_cloud_punisher_guard_off_expression, Low)
    .install()
    ;
}