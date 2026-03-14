use super::*;

//Side Special Jump ACMD
unsafe extern "C" fn ssbexo_mario_side_special_jump_acmd(_agent: &mut L2CAgentBase) {}

//Side Special Jump Effect
unsafe extern "C" fn ssbexo_mario_side_special_jump_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3, 0.5, 0, 0, -90, 0.6, true, 0.7);
        LAST_EFFECT_SET_RATE(agent, 1.4);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3, 0.5, 0, 90, -90, 0.8, false, 0.5);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
}

//Side Special Jump Sound
unsafe extern "C" fn ssbexo_mario_side_special_jump_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mario_jump01"));
    }
}

//Side Special Jump Expression
unsafe extern "C" fn ssbexo_mario_side_special_jump_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

pub fn install() {
    Agent::new("mario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialsjump", ssbexo_mario_side_special_jump_acmd, Low)
    .effect_acmd("effect_specialsjump", ssbexo_mario_side_special_jump_effect, Low)
    .sound_acmd("sound_specialsjump", ssbexo_mario_side_special_jump_sound, Low)
    .expression_acmd("expression_specialsjump", ssbexo_mario_side_special_jump_expression, Low)
    .install()
    ;
}