use super::*;

//Grounded Side Special End ACMD
unsafe extern "C" fn ssbexo_armstrong_grounded_side_special_end_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
}

//Grounded Side Special End Effect
unsafe extern "C" fn ssbexo_armstrong_grounded_side_special_end_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Grounded Side Special End Sound
unsafe extern "C" fn ssbexo_armstrong_grounded_side_special_end_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_step_right_m"));
    }
    frame(agent.lua_state_agent, 38.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_step_left_m"));
    }
}

//Grounded Side Special End Expression
unsafe extern "C" fn ssbexo_armstrong_grounded_side_special_end_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

//Aerial Side Special End ACMD
unsafe extern "C" fn ssbexo_armstrong_aerial_side_special_end_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
}

//Aerial Side Special End Effect
unsafe extern "C" fn ssbexo_armstrong_aerial_side_special_end_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Side Special End Sound
unsafe extern "C" fn ssbexo_armstrong_aerial_side_special_end_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_step_right_m"));
    }
    frame(agent.lua_state_agent, 38.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_step_left_m"));
    }
}

//Aerial Side Special End Expression
unsafe extern "C" fn ssbexo_armstrong_aerial_side_special_end_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([8, 9, 10, 11, 12, 13, 14, 15].to_vec())
    .game_acmd("game_specialsend", ssbexo_armstrong_grounded_side_special_end_acmd, Low)
    .effect_acmd("effect_specialsend", ssbexo_armstrong_grounded_side_special_end_effect, Low)
    .sound_acmd("sound_specialsend", ssbexo_armstrong_grounded_side_special_end_sound, Low)
    .expression_acmd("expression_specialsend", ssbexo_armstrong_grounded_side_special_end_expression, Low)
    .game_acmd("game_specialairsend", ssbexo_armstrong_aerial_side_special_end_acmd, Low)
    .effect_acmd("effect_specialairsend", ssbexo_armstrong_aerial_side_special_end_effect, Low)
    .sound_acmd("sound_specialairsend", ssbexo_armstrong_aerial_side_special_end_sound, Low)
    .expression_acmd("expression_specialairsend", ssbexo_armstrong_aerial_side_special_end_expression, Low)
    .install()
    ;
}