use super::*;

//Up Special Hold ACMD
unsafe extern "C" fn ssbexo_link_special_hi_hold_acmd(_agent: &mut L2CAgentBase) {}

//Up Special Hold Effect
unsafe extern "C" fn ssbexo_link_special_hi_hold_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 1.0);
        LAST_EFFECT_SET_RATE(agent, 0.5);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 1.0);
        LAST_EFFECT_SET_RATE(agent, 0.5);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 1.0);
        LAST_EFFECT_SET_RATE(agent, 0.5);
    }
}

//Up Special Hold Sound
unsafe extern "C" fn ssbexo_link_special_hi_hold_sound(_agent: &mut L2CAgentBase) {}

//Up Special Hold Expression
unsafe extern "C" fn ssbexo_link_special_hi_hold_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialhihold", ssbexo_link_special_hi_hold_acmd, Low)
    .game_acmd("game_specialairhihold", ssbexo_link_special_hi_hold_acmd, Low)
    .effect_acmd("effect_specialhihold", ssbexo_link_special_hi_hold_effect, Low)
    .effect_acmd("effect_specialairhihold", ssbexo_link_special_hi_hold_effect, Low)
    .sound_acmd("sound_specialhihold", ssbexo_link_special_hi_hold_sound, Low)
    .sound_acmd("sound_specialairhihold", ssbexo_link_special_hi_hold_sound, Low)
    .expression_acmd("expression_specialhihold", ssbexo_link_special_hi_hold_expression, Low)
    .expression_acmd("expression_specialairhihold", ssbexo_link_special_hi_hold_expression, Low)
    .install()
    ;
}