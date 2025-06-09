use super::*;

//Win 3 ACMD
unsafe extern "C" fn ssbexo_diddy_win_3_acmd(agent: &mut L2CAgentBase) {
    if !IS_EXIST_ARTICLE(agent, *FIGHTER_DIDDY_GENERATE_ARTICLE_DKBARREL) {
        if is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_DKBARREL, false, -1);
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_DKBARREL, Hash40::new("win_3"), false, -1.0);
        }
    }
}

//Win 3 Effect
unsafe extern "C" fn ssbexo_diddy_win_3_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 36.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
    frame(agent.lua_state_agent, 80.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 1.5, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
}

//Win 3 Sound
unsafe extern "C" fn ssbexo_diddy_win_3_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 36.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("se_diddy_squat_win03"));
        PLAY_SE_NO_3D(agent, Hash40::new("se_diddy_jump01_win03"));
    }
    frame(agent.lua_state_agent, 80.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("se_diddy_landing01_win03"));
    }
    frame(agent.lua_state_agent, 100.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("vc_diddy_win01"));
    }
}

//Win 3 Wait ACMD
unsafe extern "C" fn ssbexo_diddy_win_3_wait_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_DKBARREL, Hash40::new("win_3_wait"), false, -1.0);
    }
}

//Win 3 Wait Sound
unsafe extern "C" fn ssbexo_diddy_win_3_wait_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("se_diddy_win3"));
    }
}

pub fn install() {
    Agent::new("diddy")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_win3", ssbexo_diddy_win_3_acmd, Low)
    .effect_acmd("effect_win3", ssbexo_diddy_win_3_effect, Low)
    .sound_acmd("sound_win3", ssbexo_diddy_win_3_sound, Low)
    .game_acmd("game_win3wait", ssbexo_diddy_win_3_wait_acmd, Low)
    .sound_acmd("sound_win3wait", ssbexo_diddy_win_3_wait_sound, Low)
    .install()
    ;
}