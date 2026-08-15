use super::*;

//Punisher Turn Effect
unsafe extern "C" fn ssbexo_cloud_punisher_turn_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Punisher Turn Sound
unsafe extern "C" fn ssbexo_cloud_punisher_turn_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_cloud_step_right_m"));
    }
    wait(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_cloud_step_left_m"));
    }
}

//Punisher Turn Expression
unsafe extern "C" fn ssbexo_cloud_punisher_turn_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("effect_punishturn", ssbexo_cloud_punisher_turn_effect, Low)
    .acmd("sound_punishturn", ssbexo_cloud_punisher_turn_sound, Low)
    .acmd("expression_punishturn", ssbexo_cloud_punisher_turn_expression, Low)
    .install()
    ;
}