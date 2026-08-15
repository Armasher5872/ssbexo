use super::*;

//Retaliation Stance Taunt ACMD
unsafe extern "C" fn ssbexo_edge_retaliation_stance_appeal_acmd(_agent: &mut L2CAgentBase) {}

//Retaliation Stance Taunt Effect
unsafe extern "C" fn ssbexo_edge_retaliation_stance_appeal_effect(_agent: &mut L2CAgentBase) {}

//Retaliation Stance Taunt Sound
unsafe extern "C" fn ssbexo_edge_retaliation_stance_appeal_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_cloth01"));
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_edge_appeal01"));
    }
}

//Retaliation Stance Taunt Expression
unsafe extern "C" fn ssbexo_edge_retaliation_stance_appeal_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_speciallwappeal", ssbexo_edge_retaliation_stance_appeal_acmd, Low)
    .acmd("game_specialairlwappeal", ssbexo_edge_retaliation_stance_appeal_acmd, Low)
    .acmd("effect_speciallwappeal", ssbexo_edge_retaliation_stance_appeal_effect, Low)
    .acmd("effect_specialairlwappeal", ssbexo_edge_retaliation_stance_appeal_effect, Low)
    .acmd("sound_speciallwappeal", ssbexo_edge_retaliation_stance_appeal_sound, Low)
    .acmd("sound_specialairlwappeal", ssbexo_edge_retaliation_stance_appeal_sound, Low)
    .acmd("expression_speciallwappeal", ssbexo_edge_retaliation_stance_appeal_expression, Low)
    .acmd("expression_specialairlwappeal", ssbexo_edge_retaliation_stance_appeal_expression, Low)
    .install()
    ;
}