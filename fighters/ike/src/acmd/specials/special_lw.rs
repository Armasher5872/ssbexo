use super::*;

//Down Special ACMD
unsafe extern "C" fn ssbexo_ike_down_special_acmd(_agent: &mut L2CAgentBase) {}

//Down Special Effect
unsafe extern "C" fn ssbexo_ike_down_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ike_volcano_hold"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.2);
    }
}

//Down Special Sound
unsafe extern "C" fn ssbexo_ike_down_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ike_special_n06"));
    }
}

//Down Special Expression
unsafe extern "C" fn ssbexo_ike_down_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0, 2, 130, 4, 0.4, 0, 12, 30, 30, 40);
        sv_animcmd::AREA_WIND_2ND_arg10(agent.lua_state_agent);
    }
}

pub fn install() {
    Agent::new("ike")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallw", ssbexo_ike_down_special_acmd, Low)
    .game_acmd("game_specialairlw", ssbexo_ike_down_special_acmd, Low)
    .effect_acmd("effect_speciallw", ssbexo_ike_down_special_effect, Low)
    .effect_acmd("effect_specialairlw", ssbexo_ike_down_special_effect, Low)
    .sound_acmd("sound_speciallw", ssbexo_ike_down_special_sound, Low)
    .sound_acmd("sound_specialairlw", ssbexo_ike_down_special_sound, Low)
    .expression_acmd("expression_speciallw", ssbexo_ike_down_special_expression, Low)
    .expression_acmd("expression_specialairlw", ssbexo_ike_down_special_expression, Low)
    .install()
    ;
}