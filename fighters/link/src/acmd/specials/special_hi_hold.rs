use super::*;

//Up Special Hold ACMD
unsafe extern "C" fn ssbexo_link_special_hi_hold_acmd(_agent: &mut L2CAgentBase) {}

//Up Special Hold Effect
unsafe extern "C" fn ssbexo_link_special_hi_hold_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 28.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 65.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 75.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
}

//Up Special Hold Sound
unsafe extern "C" fn ssbexo_link_special_hi_hold_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_special_h01"));
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_special_h03"));
    }
}

//Up Special Hold Expression
unsafe extern "C" fn ssbexo_link_special_hi_hold_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(boma, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_back") as i64);
    }
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialhihold", ssbexo_link_special_hi_hold_acmd, Low)
    .acmd("game_specialairhihold", ssbexo_link_special_hi_hold_acmd, Low)
    .acmd("effect_specialhihold", ssbexo_link_special_hi_hold_effect, Low)
    .acmd("effect_specialairhihold", ssbexo_link_special_hi_hold_effect, Low)
    .acmd("sound_specialhihold", ssbexo_link_special_hi_hold_sound, Low)
    .acmd("sound_specialairhihold", ssbexo_link_special_hi_hold_sound, Low)
    .acmd("expression_specialhihold", ssbexo_link_special_hi_hold_expression, Low)
    .acmd("expression_specialairhihold", ssbexo_link_special_hi_hold_expression, Low)
    .install()
    ;
}