use super::*;

//Down Taunt Effect
unsafe extern "C" fn ssbexo_ness_down_taunt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        macros::FILL_SCREEN_MODEL_COLOR(agent, 1, 4, 0, 0, 0, 0, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::CHAR, *EFFECT_SCREEN_PRIO_FINAL);
        macros::FILL_SCREEN_MODEL_COLOR(agent, 2, 4, 0, 0, 0, 1, 1, 1, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, *EFFECT_SCREEN_PRIO_FINAL);
    }
    frame(agent.lua_state_agent, 56.0);
    if macros::is_excute(agent) {
        macros::CANCEL_FILL_SCREEN(agent, 1, 4);
        macros::CANCEL_FILL_SCREEN(agent, 2, 4);
    }
}

//Down Taunt Sound
unsafe extern "C" fn ssbexo_ness_down_taunt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("vc_ness_appeal03"));
    }
}

//Down Taunt Expression
unsafe extern "C" fn ssbexo_ness_down_taunt_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 110.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

pub fn install() {
    Agent::new("ness")
    .effect_acmd("effect_appeallwr", ssbexo_ness_down_taunt_effect, Priority::Low)
    .effect_acmd("effect_appeallwl", ssbexo_ness_down_taunt_effect, Priority::Low)
    .sound_acmd("sound_appeallwr", ssbexo_ness_down_taunt_sound, Priority::Low)
    .sound_acmd("sound_appeallwl", ssbexo_ness_down_taunt_sound, Priority::Low)
    .expression_acmd("expression_appeallwr", ssbexo_ness_down_taunt_expression, Priority::Low)
    .expression_acmd("expression_appeallwl", ssbexo_ness_down_taunt_expression, Priority::Low)
    .install()
    ;
}