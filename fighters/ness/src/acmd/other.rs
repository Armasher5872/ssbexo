use super::*;

//Airdodge ACMD
unsafe extern "C" fn ssbexo_ness_airdodge_acmd(_agent: &mut L2CAgentBase) {}

//Down Taunt Effect
unsafe extern "C" fn ssbexo_ness_down_taunt_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 50.0);
    if is_excute(agent) {
        FILL_SCREEN_MODEL_COLOR(agent, 1, 4, 0, 0, 0, 0, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::CHAR, *EFFECT_SCREEN_PRIO_FINAL);
        FILL_SCREEN_MODEL_COLOR(agent, 2, 4, 0, 0, 0, 1, 1, 1, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, *EFFECT_SCREEN_PRIO_FINAL);
    }
    frame(lua_state, 56.0);
    if is_excute(agent) {
        CANCEL_FILL_SCREEN(agent, 1, 4);
        CANCEL_FILL_SCREEN(agent, 2, 4);
    }
}

//Down Taunt Sound
unsafe extern "C" fn ssbexo_ness_down_taunt_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 35.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("vc_ness_appeal03"));
    }
}

//Down Taunt Expression
unsafe extern "C" fn ssbexo_ness_down_taunt_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 110.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

pub fn install() {
    Agent::new("ness")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_escapeair", ssbexo_ness_airdodge_acmd, Low)
    .acmd("game_escapeairslide", ssbexo_ness_airdodge_acmd, Low)
    .acmd("effect_appeallwr", ssbexo_ness_down_taunt_effect, Low)
    .acmd("effect_appeallwl", ssbexo_ness_down_taunt_effect, Low)
    .acmd("sound_appeallwr", ssbexo_ness_down_taunt_sound, Low)
    .acmd("sound_appeallwl", ssbexo_ness_down_taunt_sound, Low)
    .acmd("expression_appeallwr", ssbexo_ness_down_taunt_expression, Low)
    .acmd("expression_appeallwl", ssbexo_ness_down_taunt_expression, Low)
    .install()
    ;
}