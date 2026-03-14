use super::*;

//Down Special ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_acmd(_agent: &mut L2CAgentBase) {}

//Down Special Effect
unsafe extern "C" fn ssbexo_luigi_down_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Down Special Sound
unsafe extern "C" fn ssbexo_luigi_down_special_sound(_agent: &mut L2CAgentBase) {}

//Down Special Expression
unsafe extern "C" fn ssbexo_luigi_down_special_expression(agent: &mut L2CAgentBase) {
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
    .game_acmd("game_speciallw", ssbexo_luigi_down_special_acmd, Low)
    .game_acmd("game_specialairlw", ssbexo_luigi_down_special_acmd, Low)
    .effect_acmd("effect_speciallw", ssbexo_luigi_down_special_effect, Low)
    .effect_acmd("effect_specialairlw", ssbexo_luigi_down_special_effect, Low)
    .sound_acmd("sound_speciallw", ssbexo_luigi_down_special_sound, Low)
    .sound_acmd("sound_specialairlw", ssbexo_luigi_down_special_sound, Low)
    .expression_acmd("expression_speciallw", ssbexo_luigi_down_special_expression, Low)
    .expression_acmd("expression_specialairlw", ssbexo_luigi_down_special_expression, Low)
    .install()
    ;
}