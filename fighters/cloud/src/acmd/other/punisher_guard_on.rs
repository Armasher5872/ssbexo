use super::*;

//Punisher Guard On Sound
unsafe extern "C" fn ssbexo_cloud_punisher_guard_on_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_guardon"));
    }
}

//Punisher Guard On Expression
unsafe extern "C" fn ssbexo_cloud_punisher_guard_on_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_r") as i64);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("sound_punishguardon", ssbexo_cloud_punisher_guard_on_sound, Low)
    .acmd("expression_punishguardon", ssbexo_cloud_punisher_guard_on_expression, Low)
    .install()
    ;
}