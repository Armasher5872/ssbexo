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
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialncatch", ssbexo_wario_neutral_special_catch_acmd, Low)
    .acmd("effect_specialncatch", ssbexo_wario_neutral_special_catch_effect, Low)
    .acmd("sound_specialncatch", ssbexo_wario_neutral_special_catch_sound, Low)
    .acmd("expression_specialncatch", ssbexo_wario_neutral_special_catch_expression, Low)
    .acmd("game_specialairncatch", ssbexo_wario_neutral_special_catch_acmd, Low)
    .acmd("effect_specialairncatch", ssbexo_wario_neutral_special_catch_effect, Low)
    .acmd("sound_specialairncatch", ssbexo_wario_neutral_special_catch_sound, Low)
    .acmd("expression_specialairncatch", ssbexo_wario_neutral_special_catch_expression, Low)
    .install()
    ;
}