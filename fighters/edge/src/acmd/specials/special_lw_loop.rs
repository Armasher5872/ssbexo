use super::*;

//Retaliation Stance Loop ACMD
unsafe extern "C" fn ssbexo_edge_retaliation_stance_loop_acmd(_agent: &mut L2CAgentBase) {}

//Retaliation Stance Loop Effect
unsafe extern "C" fn ssbexo_edge_retaliation_stance_loop_effect(_agent: &mut L2CAgentBase) {}

//Retaliation Stance Loop Sound
unsafe extern "C" fn ssbexo_edge_retaliation_stance_loop_sound(_agent: &mut L2CAgentBase) {}

//Retaliation Stance Loop Expression
unsafe extern "C" fn ssbexo_edge_retaliation_stance_loop_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_speciallwloop", ssbexo_edge_retaliation_stance_loop_acmd, Low)
    .acmd("game_specialairlwloop", ssbexo_edge_retaliation_stance_loop_acmd, Low)
    .acmd("effect_speciallwloop", ssbexo_edge_retaliation_stance_loop_effect, Low)
    .acmd("effect_specialairlwloop", ssbexo_edge_retaliation_stance_loop_effect, Low)
    .acmd("sound_speciallwloop", ssbexo_edge_retaliation_stance_loop_sound, Low)
    .acmd("sound_specialairlwloop", ssbexo_edge_retaliation_stance_loop_sound, Low)
    .acmd("expression_speciallwloop", ssbexo_edge_retaliation_stance_loop_expression, Low)
    .acmd("expression_specialairlwloop", ssbexo_edge_retaliation_stance_loop_expression, Low)
    .install()
    ;
}