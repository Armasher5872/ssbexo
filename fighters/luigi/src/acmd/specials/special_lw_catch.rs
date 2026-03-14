use super::*;

//Down Special Catch ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_catch_acmd(_agent: &mut L2CAgentBase) {}

//Down Special Catch Effect
unsafe extern "C" fn ssbexo_luigi_down_special_catch_effect(_agent: &mut L2CAgentBase) {}

//Down Special Catch Sound
unsafe extern "C" fn ssbexo_luigi_down_special_catch_sound(_agent: &mut L2CAgentBase) {}

//Down Special Catch Pull Expression
unsafe extern "C" fn ssbexo_luigi_down_special_catch_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallwcatch", ssbexo_luigi_down_special_catch_acmd, Low)
    .effect_acmd("effect_speciallwcatch", ssbexo_luigi_down_special_catch_effect, Low)
    .sound_acmd("sound_speciallwcatch", ssbexo_luigi_down_special_catch_sound, Low)
    .expression_acmd("expression_speciallwcatch", ssbexo_luigi_down_special_catch_expression, Low)
    .install()
    ;
}