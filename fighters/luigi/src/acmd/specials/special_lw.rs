use super::*;

//Down Special ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_acmd(_agent: &mut L2CAgentBase) {}

//Down Special Effect
unsafe extern "C" fn ssbexo_luigi_down_special_effect(_agent: &mut L2CAgentBase) {}

//Down Special Sound
unsafe extern "C" fn ssbexo_luigi_down_special_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_luigi_final01"));
    }   
}

//Down Special Expression
unsafe extern "C" fn ssbexo_luigi_down_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_speciallw", ssbexo_luigi_down_special_acmd, Low)
    .acmd("game_specialairlw", ssbexo_luigi_down_special_acmd, Low)
    .acmd("effect_speciallw", ssbexo_luigi_down_special_effect, Low)
    .acmd("effect_specialairlw", ssbexo_luigi_down_special_effect, Low)
    .acmd("sound_speciallw", ssbexo_luigi_down_special_sound, Low)
    .acmd("sound_specialairlw", ssbexo_luigi_down_special_sound, Low)
    .acmd("expression_speciallw", ssbexo_luigi_down_special_expression, Low)
    .acmd("expression_specialairlw", ssbexo_luigi_down_special_expression, Low)
    .install()
    ;
}