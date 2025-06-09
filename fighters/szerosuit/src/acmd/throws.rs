use super::*;

//Grab ACMD
unsafe extern "C" fn ssbexo_szerosuit_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP, false, 0);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP, Hash40::new("catch"), false, -1.0);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("handr"), 2.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        GrabModule::clear(agent.module_accessor, 1);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 76.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Dash Grab ACMD
unsafe extern "C" fn ssbexo_szerosuit_dash_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP, false, 0);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP, Hash40::new("catch"), false, -1.0);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("handr"), 2.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        GrabModule::clear(agent.module_accessor, 1);
    }
    frame(agent.lua_state_agent, 27.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 76.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Pivot Grab ACMD
unsafe extern "C" fn ssbexo_szerosuit_pivot_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP, false, 0);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP, Hash40::new("catch"), false, -1.0);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("handr"), 2.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        GrabModule::clear(agent.module_accessor, 1);
    }
    frame(agent.lua_state_agent, 27.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 57.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

pub fn install() {
    Agent::new("szerosuit")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_catch", ssbexo_szerosuit_grab_acmd, Low)
    .game_acmd("game_catchdash", ssbexo_szerosuit_dash_grab_acmd, Low)
    .game_acmd("game_catchturn", ssbexo_szerosuit_pivot_grab_acmd, Low)
    .install()
    ;
}