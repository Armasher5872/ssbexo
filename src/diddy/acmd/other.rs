use super::*;

//Win 3 ACMD
unsafe extern "C" fn ssbexo_diddy_win_3_acmd(agent: &mut L2CAgentBase) {
    if !macros::IS_EXIST_ARTICLE(agent, *FIGHTER_DIDDY_GENERATE_ARTICLE_DKBARREL) {
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_DKBARREL, false, -1);
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_DKBARREL, Hash40::new("win_3"), false, -1.0);
        }
    }
}

//Win 3 Effect
unsafe extern "C" fn ssbexo_diddy_win_3_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
    frame(agent.lua_state_agent, 80.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 1.5, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
}

//Win 3 Sound
unsafe extern "C" fn ssbexo_diddy_win_3_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("se_diddy_squat_win03"));
        macros::PLAY_SE_NO_3D(agent, Hash40::new("se_diddy_jump01_win03"));
    }
    frame(agent.lua_state_agent, 80.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("se_diddy_landing01_win03"));
    }
    frame(agent.lua_state_agent, 100.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_diddy_win01"));
    }
}

//Win 3 Wait ACMD
unsafe extern "C" fn ssbexo_diddy_win_3_wait_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_DKBARREL, Hash40::new("win_3_wait"), false, -1.0);
    }
}

//Win 3 Wait Sound
unsafe extern "C" fn ssbexo_diddy_win_3_wait_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("se_diddy_win3"));
    }
}

pub fn install() {
    Agent::new("diddy")
    .game_acmd("game_win3", ssbexo_diddy_win_3_acmd, Priority::Low)
    .effect_acmd("effect_win3", ssbexo_diddy_win_3_effect, Priority::Low)
    .sound_acmd("sound_win3", ssbexo_diddy_win_3_sound, Priority::Low)
    .game_acmd("game_win3wait", ssbexo_diddy_win_3_wait_acmd, Priority::Low)
    .sound_acmd("sound_win3wait", ssbexo_diddy_win_3_wait_sound, Priority::Low)
    .install()
    ;
}