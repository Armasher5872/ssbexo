use super::*;

//Neutral Special Catch Turn ACMD
unsafe extern "C" fn ssbexo_wario_neutral_special_catch_turn_acmd(_agent: &mut L2CAgentBase) {}

//Neutral Special Catch Turn Effect
unsafe extern "C" fn ssbexo_wario_neutral_special_catch_turn_effect(_agent: &mut L2CAgentBase) {}

//Neutral Special Catch Turn Sound
unsafe extern "C" fn ssbexo_wario_neutral_special_catch_turn_sound(_agent: &mut L2CAgentBase) {}

//Neutral Special Catch Turn Expression
unsafe extern "C" fn ssbexo_wario_neutral_special_catch_turn_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialncatchturn", ssbexo_wario_neutral_special_catch_turn_acmd, Low)
    .effect_acmd("effect_specialncatchturn", ssbexo_wario_neutral_special_catch_turn_effect, Low)
    .sound_acmd("sound_specialncatchturn", ssbexo_wario_neutral_special_catch_turn_sound, Low)
    .expression_acmd("expression_specialncatchturn", ssbexo_wario_neutral_special_catch_turn_expression, Low)
    .install()
    ;
}