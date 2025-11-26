use super::*;

//Neutral Special Catch ACMD
unsafe extern "C" fn ssbexo_wario_neutral_special_catch_acmd(_agent: &mut L2CAgentBase) {}

//Neutral Special Catch Effect
unsafe extern "C" fn ssbexo_wario_neutral_special_catch_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("sys_catch"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
    }
}

//Neutral Special Catch Sound
unsafe extern "C" fn ssbexo_wario_neutral_special_catch_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_wario_cliffcatch"));
    }
}

//Neutral Special Catch Expression
unsafe extern "C" fn ssbexo_wario_neutral_special_catch_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialncatch", ssbexo_wario_neutral_special_catch_acmd, Low)
    .effect_acmd("effect_specialncatch", ssbexo_wario_neutral_special_catch_effect, Low)
    .sound_acmd("sound_specialncatch", ssbexo_wario_neutral_special_catch_sound, Low)
    .expression_acmd("expression_specialncatch", ssbexo_wario_neutral_special_catch_expression, Low)
    .game_acmd("game_specialairncatch", ssbexo_wario_neutral_special_catch_acmd, Low)
    .effect_acmd("effect_specialairncatch", ssbexo_wario_neutral_special_catch_effect, Low)
    .sound_acmd("sound_specialairncatch", ssbexo_wario_neutral_special_catch_sound, Low)
    .expression_acmd("expression_specialairncatch", ssbexo_wario_neutral_special_catch_expression, Low)
    .install()
    ;
}