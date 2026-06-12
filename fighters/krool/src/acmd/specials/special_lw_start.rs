use super::*;

unsafe extern "C" fn ssbexo_krool_down_special_start_acmd(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, false, -1);
        if ArticleModule::is_exist(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS) {
            ArticleModule::change_motion(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, Hash40::new("special_lw_start"), false, -1.0);
        }
    }
}

unsafe extern "C" fn ssbexo_krool_down_special_start_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 7.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn ssbexo_krool_down_special_start_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_krool_special_n08"));
    }
}

unsafe extern "C" fn ssbexo_krool_down_special_start_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
}

pub fn install() {
    Agent::new("krool")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_speciallwstart", ssbexo_krool_down_special_start_acmd, Low)
    .acmd("effect_speciallwstart", ssbexo_krool_down_special_start_effect, Low)
    .acmd("sound_speciallwstart", ssbexo_krool_down_special_start_sound, Low)
    .acmd("expression_speciallwstart", ssbexo_krool_down_special_start_expression, Low)
    .acmd("game_specialairlwstart", ssbexo_krool_down_special_start_acmd, Low)
    .acmd("effect_specialairlwstart", ssbexo_krool_down_special_start_effect, Low)
    .acmd("sound_specialairlwstart", ssbexo_krool_down_special_start_sound, Low)
    .acmd("expression_specialairlwstart", ssbexo_krool_down_special_start_expression, Low)
    .install()
    ;
}