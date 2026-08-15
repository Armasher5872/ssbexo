use super::*;

//Punisher Dash ACMD
unsafe extern "C" fn ssbexo_cloud_punisher_dash_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 13.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

//Punisher Dash Effect
unsafe extern "C" fn ssbexo_cloud_punisher_dash_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 5.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Punisher Dash Sound
unsafe extern "C" fn ssbexo_cloud_punisher_dash_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_dash_start"));
        SET_PLAY_INHIVIT(agent, Hash40::new("se_cloud_dash_start"), 20);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_cloud_step_left_l"));
    }
    frame(lua_state, 55.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_cloud_step_left_r"));
    }
    frame(lua_state, 76.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_cloud_step_left_l"));
    }
}

//Punisher Dash Expression
unsafe extern "C" fn ssbexo_cloud_punisher_dash_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_punishdash", ssbexo_cloud_punisher_dash_acmd, Low)
    .acmd("effect_punishdash", ssbexo_cloud_punisher_dash_effect, Low)
    .acmd("sound_punishdash", ssbexo_cloud_punisher_dash_sound, Low)
    .acmd("expression_punishdash", ssbexo_cloud_punisher_dash_expression, Low)
    .install()
    ;
}