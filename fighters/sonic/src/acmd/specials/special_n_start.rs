use super::*;

//Neutral Special Start ACMD
unsafe extern "C" fn ssbexo_sonic_neutral_special_start_acmd(_agent: &mut L2CAgentBase) {}

//Neutral Special Start Effect
unsafe extern "C" fn ssbexo_sonic_neutral_special_start_effect(_agent: &mut L2CAgentBase) {}

//Neutral Special Start Sound
unsafe extern "C" fn ssbexo_sonic_neutral_special_start_sound(_agent: &mut L2CAgentBase) {}

//Neutral Special Start Expression
unsafe extern "C" fn ssbexo_sonic_neutral_special_start_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
}

pub fn install() {
    Agent::new("sonic")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialnstart", ssbexo_sonic_neutral_special_start_acmd, Low)
    .effect_acmd("effect_specialnstart", ssbexo_sonic_neutral_special_start_effect, Low)
    .sound_acmd("sound_specialnstart", ssbexo_sonic_neutral_special_start_sound, Low)
    .expression_acmd("expression_specialnstart", ssbexo_sonic_neutral_special_start_expression, Low)
    .game_acmd("game_specialairnstart", ssbexo_sonic_neutral_special_start_acmd, Low)
    .effect_acmd("effect_specialairnstart", ssbexo_sonic_neutral_special_start_effect, Low)
    .sound_acmd("sound_specialairnstart", ssbexo_sonic_neutral_special_start_sound, Low)
    .expression_acmd("expression_specialairnstart", ssbexo_sonic_neutral_special_start_expression, Low)
    .install()
    ;
}