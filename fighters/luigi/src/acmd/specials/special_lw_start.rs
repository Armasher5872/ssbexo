use super::*;

//Down Special Start ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_start_acmd(_agent: &mut L2CAgentBase) {}

//Down Special Start Effect
unsafe extern "C" fn ssbexo_luigi_down_special_start_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 10.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Down Special Start Sound
unsafe extern "C" fn ssbexo_luigi_down_special_start_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_luigi_special_l01"));
        PLAY_SE(agent, Hash40::new("se_luigi_special_l02"));
    }
}

//Down Special Start Expression
unsafe extern "C" fn ssbexo_luigi_down_special_start_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_speciallwstart", ssbexo_luigi_down_special_start_acmd, Low)
    .acmd("game_specialairlwstart", ssbexo_luigi_down_special_start_acmd, Low)
    .acmd("effect_speciallwstart", ssbexo_luigi_down_special_start_effect, Low)
    .acmd("effect_specialairlwstart", ssbexo_luigi_down_special_start_effect, Low)
    .acmd("sound_speciallwstart", ssbexo_luigi_down_special_start_sound, Low)
    .acmd("sound_specialairlwstart", ssbexo_luigi_down_special_start_sound, Low)
    .acmd("expression_speciallwstart", ssbexo_luigi_down_special_start_expression, Low)
    .acmd("expression_specialairlwstart", ssbexo_luigi_down_special_start_expression, Low)
    .install()
    ;
}