use super::*;

//Neutral Special Start ACMD
unsafe extern "C" fn ssbexo_sonic_neutral_special_start_acmd(_agent: &mut L2CAgentBase) {}

//Neutral Special Start Effect
unsafe extern "C" fn ssbexo_sonic_neutral_special_start_effect(_agent: &mut L2CAgentBase) {}

//Neutral Special Start Sound
unsafe extern "C" fn ssbexo_sonic_neutral_special_start_sound(_agent: &mut L2CAgentBase) {}

//Neutral Special Start Expression
unsafe extern "C" fn ssbexo_sonic_neutral_special_start_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
}

pub fn install() {
    Agent::new("sonic")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialnstart", ssbexo_sonic_neutral_special_start_acmd, Low)
    .acmd("effect_specialnstart", ssbexo_sonic_neutral_special_start_effect, Low)
    .acmd("sound_specialnstart", ssbexo_sonic_neutral_special_start_sound, Low)
    .acmd("expression_specialnstart", ssbexo_sonic_neutral_special_start_expression, Low)
    .acmd("game_specialairnstart", ssbexo_sonic_neutral_special_start_acmd, Low)
    .acmd("effect_specialairnstart", ssbexo_sonic_neutral_special_start_effect, Low)
    .acmd("sound_specialairnstart", ssbexo_sonic_neutral_special_start_sound, Low)
    .acmd("expression_specialairnstart", ssbexo_sonic_neutral_special_start_expression, Low)
    .install()
    ;
}