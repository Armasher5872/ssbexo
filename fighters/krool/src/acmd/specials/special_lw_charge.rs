use super::*;

unsafe extern "C" fn ssbexo_krool_down_special_charge_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, Hash40::new("special_lw_charge"), false, -1.0);
            ArticleModule::set_rate(agent.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, 1.25);
        }
        MotionModule::set_rate(agent.module_accessor, 1.25);
    }
}

unsafe extern "C" fn ssbexo_krool_down_special_charge_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 45.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 55.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn ssbexo_krool_down_special_charge_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_krool_special_n01"));
    }
}

unsafe extern "C" fn ssbexo_krool_down_special_charge_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("krool")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallwcharge", ssbexo_krool_down_special_charge_acmd, Low)
    .effect_acmd("effect_speciallwcharge", ssbexo_krool_down_special_charge_effect, Low)
    .sound_acmd("sound_speciallwcharge", ssbexo_krool_down_special_charge_sound, Low)
    .expression_acmd("expression_speciallwcharge", ssbexo_krool_down_special_charge_expression, Low)
    .game_acmd("game_specialairlwcharge", ssbexo_krool_down_special_charge_acmd, Low)
    .effect_acmd("effect_specialairlwcharge", ssbexo_krool_down_special_charge_effect, Low)
    .sound_acmd("sound_specialairlwcharge", ssbexo_krool_down_special_charge_sound, Low)
    .expression_acmd("expression_specialairlwcharge", ssbexo_krool_down_special_charge_expression, Low)
    .install()
    ;
}