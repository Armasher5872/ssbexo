use super::*;

//Neutral Special Catch Wait ACMD
unsafe extern "C" fn ssbexo_wario_neutral_special_catch_wait_acmd(_agent: &mut L2CAgentBase) {}

//Neutral Special Catch Wait Effect
unsafe extern "C" fn ssbexo_wario_neutral_special_catch_wait_effect(_agent: &mut L2CAgentBase) {}

//Neutral Special Catch Wait Sound
unsafe extern "C" fn ssbexo_wario_neutral_special_catch_wait_sound(_agent: &mut L2CAgentBase) {}

//Neutral Special Catch Wait Expression
unsafe extern "C" fn ssbexo_wario_neutral_special_catch_wait_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialncatchwait", ssbexo_wario_neutral_special_catch_wait_acmd, Low)
    .effect_acmd("effect_specialncatchwait", ssbexo_wario_neutral_special_catch_wait_effect, Low)
    .sound_acmd("sound_specialncatchwait", ssbexo_wario_neutral_special_catch_wait_sound, Low)
    .expression_acmd("expression_specialncatchwait", ssbexo_wario_neutral_special_catch_wait_expression, Low)
    .game_acmd("game_specialairncatchwait", ssbexo_wario_neutral_special_catch_wait_acmd, Low)
    .effect_acmd("effect_specialairncatchwait", ssbexo_wario_neutral_special_catch_wait_effect, Low)
    .sound_acmd("sound_specialairncatchwait", ssbexo_wario_neutral_special_catch_wait_sound, Low)
    .expression_acmd("expression_specialairncatchwait", ssbexo_wario_neutral_special_catch_wait_expression, Low)
    .install()
    ;
}