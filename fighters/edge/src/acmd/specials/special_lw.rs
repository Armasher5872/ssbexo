use super::*;

//Retaliation Stance Start ACMD
unsafe extern "C" fn ssbexo_edge_retaliation_stance_start_acmd(_agent: &mut L2CAgentBase) {}

//Retaliation Stance Start Effect
unsafe extern "C" fn ssbexo_edge_retaliation_stance_start_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_muzzleflash"), Hash40::new("waist"), 0, 0, 0, 90, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}

//Retaliation Stance Start Sound
unsafe extern "C" fn ssbexo_edge_retaliation_stance_start_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_jump01"));
        PLAY_SE(agent, Hash40::new("se_edge_attackair_f01"));
        PLAY_SE(agent, Hash40::new("se_edge_appeal_l01"));
    }
}

//Retaliation Stance Start Expression
unsafe extern "C" fn ssbexo_edge_retaliation_stance_start_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_speciallw", ssbexo_edge_retaliation_stance_start_acmd, Low)
    .acmd("game_specialairlw", ssbexo_edge_retaliation_stance_start_acmd, Low)
    .acmd("effect_speciallw", ssbexo_edge_retaliation_stance_start_effect, Low)
    .acmd("effect_specialairlw", ssbexo_edge_retaliation_stance_start_effect, Low)
    .acmd("sound_speciallw", ssbexo_edge_retaliation_stance_start_sound, Low)
    .acmd("sound_specialairlw", ssbexo_edge_retaliation_stance_start_sound, Low)
    .acmd("expression_speciallw", ssbexo_edge_retaliation_stance_start_expression, Low)
    .acmd("expression_specialairlw", ssbexo_edge_retaliation_stance_start_expression, Low)
    .install()
    ;
}