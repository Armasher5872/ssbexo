use super::*;

//Up Special Ascend End ACMD
unsafe extern "C" fn ssbexo_link_special_hi_ascend_end_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 6.0);
    FT_MOTION_RATE(agent, 1.0);
}

//Up Special Ascend End Effect
unsafe extern "C" fn ssbexo_link_special_hi_ascend_end_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.25, 1.0, 0.5);
        LAST_EFFECT_SET_RATE(agent, 1.1);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
}

//Up Special Ascend End Sound
unsafe extern "C" fn ssbexo_link_special_hi_ascend_end_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent,Hash40::new("se_link_special_h04"));
    }
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent,Hash40::new("vc_link_cliffcatch"));
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_jump02"));
    }
    wait(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
    wait(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
}

//Up Special Ascend End Expression
unsafe extern "C" fn ssbexo_link_special_hi_ascend_end_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_jump"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialhiend", ssbexo_link_special_hi_ascend_end_acmd, Low)
    .effect_acmd("effect_specialhiend", ssbexo_link_special_hi_ascend_end_effect, Low)
    .sound_acmd("sound_specialhiend", ssbexo_link_special_hi_ascend_end_sound, Low)
    .expression_acmd("expression_specialhiend", ssbexo_link_special_hi_ascend_end_expression, Low)
    .install()
    ;
}