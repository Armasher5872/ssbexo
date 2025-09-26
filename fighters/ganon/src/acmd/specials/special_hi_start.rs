use super::*;

//Up Special Start ACMD
unsafe extern "C" fn ssbexo_ganon_up_special_start_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.5);
    }
    frame(agent.lua_state_agent, 27.0);
    if is_excute(agent) {
        HitModule::set_whole(agent.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    }
}

//Grounded Up Special Start Effect
unsafe extern "C" fn ssbexo_ganon_grounded_up_special_start_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        BURN_COLOR_FRAME(agent, 18, 0.2, 0, 1.7, 0);
        FLASH_FRM(agent, 18, 0, 0, 0, 0);
        ColorBlendModule::set_disable_camera_depth_influence(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        FLASH(agent, 0, 0, 0, 0.8);
        BURN_COLOR(agent, 0.2, 0, 1.7, 0.4);
    }
    for _ in 0..5 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        BURN_COLOR_NORMAL(agent);
        COL_NORMAL(agent);
        ColorBlendModule::set_disable_camera_depth_influence(agent.module_accessor, false);
    }
}

//Aerial Up Special Start Effect
unsafe extern "C" fn ssbexo_ganon_aerial_up_special_start_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        BURN_COLOR_FRAME(agent, 18, 0.2, 0, 1.7, 0);
        FLASH_FRM(agent, 18, 0, 0, 0, 0);
        ColorBlendModule::set_disable_camera_depth_influence(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        FLASH(agent, 0, 0, 0, 0.8);
        BURN_COLOR(agent, 0.2, 0, 1.7, 0.4);
    }
    for _ in 0..5 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        BURN_COLOR_NORMAL(agent);
        COL_NORMAL(agent);
        ColorBlendModule::set_disable_camera_depth_influence(agent.module_accessor, false);
    }
}

//Up Special Start Sound
unsafe extern "C" fn ssbexo_ganon_up_special_start_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_appear01"));
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_appeal_l01"));
    }
}

//Up Special Start Expression
unsafe extern "C" fn ssbexo_ganon_up_special_start_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialhi", ssbexo_ganon_up_special_start_acmd, Low)
    .effect_acmd("effect_specialhi", ssbexo_ganon_grounded_up_special_start_effect, Low)
    .sound_acmd("sound_specialhi", ssbexo_ganon_up_special_start_sound, Low)
    .expression_acmd("expression_specialhi", ssbexo_ganon_up_special_start_expression, Low)
    .game_acmd("game_specialairhi", ssbexo_ganon_up_special_start_acmd, Low)
    .effect_acmd("effect_specialairhi", ssbexo_ganon_aerial_up_special_start_effect, Low)
    .sound_acmd("sound_specialairhi", ssbexo_ganon_up_special_start_sound, Low)
    .expression_acmd("expression_specialairhi", ssbexo_ganon_up_special_start_expression, Low)
    .install()
    ;
}