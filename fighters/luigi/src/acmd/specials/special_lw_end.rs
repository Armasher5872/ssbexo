use super::*;

//Down Special End ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_end_acmd(_agent: &mut L2CAgentBase) {}

//Down Special End Effect
unsafe extern "C" fn ssbexo_luigi_down_special_end_effect(_agent: &mut L2CAgentBase) {}

//Down Special End Sound
unsafe extern "C" fn ssbexo_luigi_down_special_end_sound(_agent: &mut L2CAgentBase) {}

//Down Special End Expression
unsafe extern "C" fn ssbexo_luigi_down_special_end_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallwend", ssbexo_luigi_down_special_end_acmd, Low)
    .game_acmd("game_specialairlwend", ssbexo_luigi_down_special_end_acmd, Low)
    .effect_acmd("effect_speciallwend", ssbexo_luigi_down_special_end_effect, Low)
    .effect_acmd("effect_specialairlwend", ssbexo_luigi_down_special_end_effect, Low)
    .sound_acmd("sound_speciallwend", ssbexo_luigi_down_special_end_sound, Low)
    .sound_acmd("sound_specialairlwend", ssbexo_luigi_down_special_end_sound, Low)
    .expression_acmd("expression_speciallwend", ssbexo_luigi_down_special_end_expression, Low)
    .expression_acmd("expression_specialairlwend", ssbexo_luigi_down_special_end_expression, Low)
    .install()
    ;
}