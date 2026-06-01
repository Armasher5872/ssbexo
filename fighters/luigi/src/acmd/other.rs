use super::*;

//Win 1 ACMD
unsafe extern "C" fn ssbexo_luigi_win_1_acmd(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if IS_EXIST_ARTICLE(agent, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU) {
        if is_excute(agent) {
            ArticleModule::remove_exist(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, false, -1);
        if ArticleModule::is_exist(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU) {
            ArticleModule::change_status(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_WIN, ArticleOperationTarget(0));
        }
    }
}

//Win 1 Effect
unsafe extern "C" fn ssbexo_luigi_win_1_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 123.0);
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU) {
            let obakyumu_boma = get_article_boma(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU);
            let obakyumu_agent = get_weapon_common_from_accessor(&mut *obakyumu_boma);
            obakyumu_agent.clear_lua_stack();
            lua_args!(obakyumu_agent, Hash40::new("luigi_win_confetti"), Hash40::new("vacuum"), 0, 0, 0, 0, 0, -90, 1, 0, 0, 0, 0, 0, 0, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE);
            smash::app::sv_animcmd::EFFECT_ATTR(obakyumu_agent.lua_state_agent);
        }
    }
}

//Win 1 Sound
unsafe extern "C" fn ssbexo_luigi_win_1_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_luigi_clap"));
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_luigi_clap"));
    }
    frame(lua_state, 75.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_luigi_cheer"));
    }
    frame(lua_state, 100.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_luigi_final04"));
    }
    frame(lua_state, 115.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_luigi_succeed"));
    }
    frame(lua_state, 129.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_luigi_win_poltergust_pop"));
    }
    frame(lua_state, 146.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_luigi_step_left_m"));
    }
}

//Win 1 Expression
unsafe extern "C" fn ssbexo_luigi_win_1_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_win1", ssbexo_luigi_win_1_acmd, Low)
    .effect_acmd("effect_win1", ssbexo_luigi_win_1_effect, Low)
    .sound_acmd("sound_win1", ssbexo_luigi_win_1_sound, Low)
    .expression_acmd("expression_win1", ssbexo_luigi_win_1_expression, Low)
    .install()
    ;
}