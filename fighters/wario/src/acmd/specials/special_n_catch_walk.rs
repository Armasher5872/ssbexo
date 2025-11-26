use super::*;

//Neutral Special Catch Walk ACMD
unsafe extern "C" fn ssbexo_wario_neutral_special_catch_walk_acmd(_agent: &mut L2CAgentBase) {}

//Neutral Special Catch Walk Effect
unsafe extern "C" fn ssbexo_wario_neutral_special_catch_walk_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Neutral Special Catch Walk Sound
unsafe extern "C" fn ssbexo_wario_neutral_special_catch_walk_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_wario_step_left_m"));
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_wario_step_right_m"));
    }
}

//Neutral Special Catch Walk Expression
unsafe extern "C" fn ssbexo_wario_neutral_special_catch_walk_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialncatchwalk", ssbexo_wario_neutral_special_catch_walk_acmd, Low)
    .effect_acmd("effect_specialncatchwalk", ssbexo_wario_neutral_special_catch_walk_effect, Low)
    .sound_acmd("sound_specialncatchwalk", ssbexo_wario_neutral_special_catch_walk_sound, Low)
    .expression_acmd("expression_specialncatchwalk", ssbexo_wario_neutral_special_catch_walk_expression, Low)
    .install()
    ;
}