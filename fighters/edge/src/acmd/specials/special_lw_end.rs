use super::*;

//Retaliation Stance End ACMD
unsafe extern "C" fn ssbexo_edge_retaliation_stance_end_acmd(_agent: &mut L2CAgentBase) {}

//Retaliation Stance End Effect
unsafe extern "C" fn ssbexo_edge_retaliation_stance_end_effect(_agent: &mut L2CAgentBase) {}

//Retaliation Stance End Sound
unsafe extern "C" fn ssbexo_edge_retaliation_stance_end_sound(_agent: &mut L2CAgentBase) {}

//Retaliation Stance End Expression
unsafe extern "C" fn ssbexo_edge_retaliation_stance_end_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_speciallwend", ssbexo_edge_retaliation_stance_end_acmd, Low)
    .acmd("game_specialairlwend", ssbexo_edge_retaliation_stance_end_acmd, Low)
    .acmd("effect_speciallwend", ssbexo_edge_retaliation_stance_end_effect, Low)
    .acmd("effect_specialairlwend", ssbexo_edge_retaliation_stance_end_effect, Low)
    .acmd("sound_speciallwend", ssbexo_edge_retaliation_stance_end_sound, Low)
    .acmd("sound_specialairlwend", ssbexo_edge_retaliation_stance_end_sound, Low)
    .acmd("expression_speciallwend", ssbexo_edge_retaliation_stance_end_expression, Low)
    .acmd("expression_specialairlwend", ssbexo_edge_retaliation_stance_end_expression, Low)
    .install()
    ;
}