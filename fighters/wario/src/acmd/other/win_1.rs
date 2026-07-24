use super::*;

//Win 1 ACMD
unsafe extern "C" fn ssbexo_wario_win_1_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(boma, *FIGHTER_WARIO_GENERATE_ARTICLE_GARLIC, true, -1);
        if ArticleModule::is_exist(boma, *FIGHTER_WARIO_GENERATE_ARTICLE_GARLIC) {
            let garlic_boma = get_article_boma(boma, *FIGHTER_WARIO_GENERATE_ARTICLE_GARLIC);
            LinkModule::set_model_constraint_pos_ort(garlic_boma, *LINK_NO_CONSTRAINT, Hash40::new("have"), Hash40::new("haver"), (*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE | *CONSTRAINT_FLAG_OFFSET_ROT) as u32, true);
        }
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_WARIO_GENERATE_ARTICLE_GARLIC, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Win 1 Effect
unsafe extern "C" fn ssbexo_wario_win_1_effect(_agent: &mut L2CAgentBase) {}

//Win 1 Sound
unsafe extern "C" fn ssbexo_wario_win_1_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 25.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("vc_wario_007"));
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("vc_wario_005"));
    }
    frame(lua_state, 115.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("vc_wario_win02"));
    }
}

//Win 1 Expression
unsafe extern "C" fn ssbexo_wario_win_1_expression(_agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_win1", ssbexo_wario_win_1_acmd, Low)
    .acmd("effect_win1", ssbexo_wario_win_1_effect, Low)
    .acmd("sound_win1", ssbexo_wario_win_1_sound, Low)
    .acmd("expression_win1", ssbexo_wario_win_1_expression, Low)
    .install()
    ;
}