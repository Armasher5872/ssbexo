use super::*;

//Down Special Catch Wait ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_catch_wait_acmd(_agent: &mut L2CAgentBase) {}

//Down Special Catch Wait Effect
unsafe extern "C" fn ssbexo_luigi_down_special_catch_wait_effect(_agent: &mut L2CAgentBase) {}

//Down Special Catch Wait Sound
unsafe extern "C" fn ssbexo_luigi_down_special_catch_wait_sound(_agent: &mut L2CAgentBase) {}

//Down Special Catch Wait Expression
unsafe extern "C" fn ssbexo_luigi_down_special_catch_wait_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallwcatchwait", ssbexo_luigi_down_special_catch_wait_acmd, Low)
    .effect_acmd("effect_speciallwcatchwait", ssbexo_luigi_down_special_catch_wait_effect, Low)
    .sound_acmd("sound_speciallwcatchwait", ssbexo_luigi_down_special_catch_wait_sound, Low)
    .expression_acmd("expression_speciallwcatchwait", ssbexo_luigi_down_special_catch_wait_expression, Low)
    .install()
    ;
}