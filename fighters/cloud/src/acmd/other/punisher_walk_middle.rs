use super::*;

//Punisher Middle Walk Effect
unsafe extern "C" fn ssbexo_cloud_punisher_middle_walk_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 35.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footl"), 4, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
    }
    wait(lua_state, 27.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footr"), 4, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
    }
}

//Punisher Middle Walk Sound
unsafe extern "C" fn ssbexo_cloud_punisher_middle_walk_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 34.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_cloud_step_left_s"));
    }
    wait(lua_state, 26.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_cloud_step_right_s"));
    }
}

//Punisher Middle Walk Expression
unsafe extern "C" fn ssbexo_cloud_punisher_middle_walk_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(lua_state, 20.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("effect_punishwalkmiddle", ssbexo_cloud_punisher_middle_walk_effect, Low)
    .acmd("sound_punishwalkmiddle", ssbexo_cloud_punisher_middle_walk_sound, Low)
    .acmd("expression_punishwalkmiddle", ssbexo_cloud_punisher_middle_walk_expression, Low)
    .install()
    ;
}