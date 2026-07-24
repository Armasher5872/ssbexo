use super::*;

//Toot Kamikaze ACMD
unsafe extern "C" fn ssbexo_wario_toot_kamikaze_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(lua_state, 55.0);
    if is_excute(agent) {
        SlowModule::set_whole(boma, 20, 30);
        FT_SET_FINAL_FEAR_FACE(agent, 30);
    }
    frame(lua_state, 57.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, FIGHTER_WARIO_GENERATE_ARTICLE_KAMIKAZE, false, -1);
    }
    frame(lua_state, 58.0);
    if is_excute(agent) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DEAD, false);
    }
}

//Toot Kamikaze Effect
unsafe extern "C" fn ssbexo_wario_toot_kamikaze_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let lr_check = get_value_float(lua_state, *SO_VAR_FLOAT_LR);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("wario_kamikaze_start"), Hash40::new("top"), 0, 6, 0, 0, -60, 0, 1, false);
    }
    if lr_check < 0.0 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("wario_kamikaze"), Hash40::new("top"), 0, 7, -1, 0, 180, 0, 1, true);
            LAST_EFFECT_SET_RATE(agent, 0.8);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("wario_kamikaze"), Hash40::new("top"), 0, 7, -0.5, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_RATE(agent, 0.8);
        }
    }
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, true);
        sv_animcmd::LAST_EFFECT_SET_DISABLE_SYSTEM_SLOW(lua_state);
    }
    frame(lua_state, 5.0);
    if lr_check < 0.0 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wario_ppe_s"), Hash40::new("top"), 0, 0, 0, 0, 100, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wario_ppe_s"), Hash40::new("top"), 0, 0, 0, 0, -100, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 20.0);
    if lr_check < 0.0 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wario_ppe_s"), Hash40::new("top"), 0, 0, 0, 0, 100, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wario_ppe_s"), Hash40::new("top"), 0, 0, 0, 0, -100, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 35.0);
    if lr_check < 0.0 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wario_ppe_s"), Hash40::new("top"), 0, 0, 0, 0, 100, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wario_ppe_s"), Hash40::new("top"), 0, 0, 0, 0, -100, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        FLASH(agent, 0, 0, 0, 0);
        ColorBlendModule::set_disable_camera_depth_influence(boma, true);
        BURN_COLOR(agent, 4, 4, 1.2, 0);
        ColorBlendModule::set_disable_camera_depth_influence(boma, true);
    }
    frame(lua_state, 50.0);
    if lr_check < 0.0 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wario_ppe_s"), Hash40::new("top"), 0, 0, 0, 0, 100, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wario_ppe_s"), Hash40::new("top"), 0, 0, 0, 0, -100, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 1.0, 1.0, 1.0, 1);
        sv_animcmd::FLASH_NO_STOP(lua_state);
        FLASH_FRM(agent, 50, 1, 1, 1, 1);
        BURN_COLOR_FRAME(agent, 50, 4, 4, 4, 1);
    }
    frame(lua_state, 57.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("wario_shock"), Hash40::new("top"), 3, 0, -4, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("wario_ground_crack"), Hash40::new("top"), 3, 0, -7, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.5);
    }
}

//Toot Kamikaze Sound
unsafe extern "C" fn ssbexo_wario_toot_kamikaze_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_special_s01"));
        PLAY_SE(agent, Hash40::new("se_wario_special_s07"));
    }
}

//Toot Kamikaze Expression
unsafe extern "C" fn ssbexo_wario_toot_kamikaze_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattack"), 40, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_appealkamikaze", ssbexo_wario_toot_kamikaze_acmd, Low)
    .acmd("effect_appealkamikaze", ssbexo_wario_toot_kamikaze_effect, Low)
    .acmd("sound_appealkamikaze", ssbexo_wario_toot_kamikaze_sound, Low)
    .acmd("expression_appealkamikaze", ssbexo_wario_toot_kamikaze_expression, Low)
    .install()
    ;
}