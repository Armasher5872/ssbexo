use super::*;

//Down Special Start ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_start_acmd(_agent: &mut L2CAgentBase) {}

//Down Special Start Effect
unsafe extern "C" fn ssbexo_luigi_down_special_start_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Down Special Start Sound
unsafe extern "C" fn ssbexo_luigi_down_special_start_sound(_agent: &mut L2CAgentBase) {}

//Down Special Start Expression
unsafe extern "C" fn ssbexo_luigi_down_special_start_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallwstart", ssbexo_luigi_down_special_start_acmd, Low)
    .game_acmd("game_specialairlwstart", ssbexo_luigi_down_special_start_acmd, Low)
    .effect_acmd("effect_speciallwstart", ssbexo_luigi_down_special_start_effect, Low)
    .effect_acmd("effect_specialairlwstart", ssbexo_luigi_down_special_start_effect, Low)
    .sound_acmd("sound_speciallwstart", ssbexo_luigi_down_special_start_sound, Low)
    .sound_acmd("sound_specialairlwstart", ssbexo_luigi_down_special_start_sound, Low)
    .expression_acmd("expression_speciallwstart", ssbexo_luigi_down_special_start_expression, Low)
    .expression_acmd("expression_specialairlwstart", ssbexo_luigi_down_special_start_expression, Low)
    .install()
    ;
}