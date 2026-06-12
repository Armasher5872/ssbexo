use super::*;

//Up Special Start ACMD
unsafe extern "C" fn ssbexo_ganon_up_special_start_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.5);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_XLU), 0);
    }
}

//Grounded Up Special Start Effect
unsafe extern "C" fn ssbexo_ganon_grounded_up_special_start_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        BURN_COLOR_FRAME(agent, 18, 0.2, 0, 1.7, 0);
        FLASH_FRM(agent, 18, 0, 0, 0, 0);
        ColorBlendModule::set_disable_camera_depth_influence(boma, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        FLASH(agent, 0, 0, 0, 0.8);
        BURN_COLOR(agent, 0.2, 0, 1.7, 0.4);
    }
    for _ in 0..5 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        BURN_COLOR_NORMAL(agent);
        COL_NORMAL(agent);
        ColorBlendModule::set_disable_camera_depth_influence(boma, false);
    }
}

//Aerial Up Special Start Effect
unsafe extern "C" fn ssbexo_ganon_aerial_up_special_start_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        BURN_COLOR_FRAME(agent, 18, 0.2, 0, 1.7, 0);
        FLASH_FRM(agent, 18, 0, 0, 0, 0);
        ColorBlendModule::set_disable_camera_depth_influence(boma, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        FLASH(agent, 0, 0, 0, 0.8);
        BURN_COLOR(agent, 0.2, 0, 1.7, 0.4);
    }
    for _ in 0..5 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        BURN_COLOR_NORMAL(agent);
        COL_NORMAL(agent);
        ColorBlendModule::set_disable_camera_depth_influence(boma, false);
    }
}

//Up Special Start Sound
unsafe extern "C" fn ssbexo_ganon_up_special_start_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_appear01"));
    }
    frame(lua_state, 24.0);
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
    .acmd("game_specialhi", ssbexo_ganon_up_special_start_acmd, Low)
    .acmd("effect_specialhi", ssbexo_ganon_grounded_up_special_start_effect, Low)
    .acmd("sound_specialhi", ssbexo_ganon_up_special_start_sound, Low)
    .acmd("expression_specialhi", ssbexo_ganon_up_special_start_expression, Low)
    .acmd("game_specialairhi", ssbexo_ganon_up_special_start_acmd, Low)
    .acmd("effect_specialairhi", ssbexo_ganon_aerial_up_special_start_effect, Low)
    .acmd("sound_specialairhi", ssbexo_ganon_up_special_start_sound, Low)
    .acmd("expression_specialairhi", ssbexo_ganon_up_special_start_expression, Low)
    .install()
    ;
}