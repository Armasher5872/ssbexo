use super::*;

//Win 1 ACMD
unsafe extern "C" fn ssbexo_wario_win_1_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_GARLIC, true, -1);
        if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_GARLIC) {
            let garlic_boma = get_article_boma(agent.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_GARLIC);
            LinkModule::set_model_constraint_pos_ort(garlic_boma, *LINK_NO_CONSTRAINT, Hash40::new("have"), Hash40::new("haver"), (*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE | *CONSTRAINT_FLAG_OFFSET_ROT) as u32, true);
        }
    }
    frame(agent.lua_state_agent, 60.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_GARLIC, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Win 1 Effect
unsafe extern "C" fn ssbexo_wario_win_1_effect(_agent: &mut L2CAgentBase) {}

//Win 1 Sound
unsafe extern "C" fn ssbexo_wario_win_1_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("vc_wario_007"));
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("vc_wario_005"));
    }
    frame(agent.lua_state_agent, 115.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("vc_wario_final05"));
    }
}

//Win 1 Expression
unsafe extern "C" fn ssbexo_wario_win_1_expression(_agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_win1", ssbexo_wario_win_1_acmd, Low)
    .effect_acmd("effect_win1", ssbexo_wario_win_1_effect, Low)
    .sound_acmd("sound_win1", ssbexo_wario_win_1_sound, Low)
    .expression_acmd("expression_win1", ssbexo_wario_win_1_expression, Low)
    .install()
    ;
}