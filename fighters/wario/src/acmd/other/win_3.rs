use super::*;

//Win 3 ACMD
unsafe extern "C" fn ssbexo_wario_win_3_acmd(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(boma, FIGHTER_WARIO_GENERATE_ARTICLE_CHAIR, false, -1);
        if ArticleModule::is_exist(boma, FIGHTER_WARIO_GENERATE_ARTICLE_CHAIR) {
            let chair_boma = get_article_boma(boma, FIGHTER_WARIO_GENERATE_ARTICLE_CHAIR);
            ModelModule::set_scale(chair_boma, 1.25);
        }
    }
}

//Win 3 Effect
unsafe extern "C" fn ssbexo_wario_win_3_effect(_agent: &mut L2CAgentBase) {}

//Win 3 Sound
unsafe extern "C" fn ssbexo_wario_win_3_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("vc_wario_win03"));
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_win3_01_win03"));
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_win3_01_win03"));
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_win3_01_win03"));
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_win3_01_win03"));
    }
    frame(lua_state, 63.0);
    if is_excute(agent) {
        let sound = SoundModule::play_se(boma, Hash40::new("se_common_landing_iron"), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(boma, sound as i32, 4.0, 0);
    }
    frame(lua_state, 75.0);
    if is_excute(agent) {
        let sound = SoundModule::play_se(boma, Hash40::new("se_common_step_iron"), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(boma, sound as i32, 4.0, 0);
    }
    frame(lua_state, 88.0);
    if is_excute(agent) {
        let sound = SoundModule::play_se(boma, Hash40::new("se_common_step_iron"), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(boma, sound as i32, 4.0, 0);
    }
    frame(lua_state, 100.0);
    if is_excute(agent) {
        let sound = SoundModule::play_se(boma, Hash40::new("se_common_step_iron"), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(boma, sound as i32, 4.0, 0);
    }
}

//Win 3 Expression
unsafe extern "C" fn ssbexo_wario_win_3_expression(_agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_win3", ssbexo_wario_win_3_acmd, Low)
    .acmd("effect_win3", ssbexo_wario_win_3_effect, Low)
    .acmd("sound_win3", ssbexo_wario_win_3_sound, Low)
    .acmd("expression_win3", ssbexo_wario_win_3_expression, Low)
    .install()
    ;
}