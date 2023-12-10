use super::*;

//Grab ACMD
unsafe extern "C" fn ssbexo_younglink_grab_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, false, 0);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, false, 0);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ArticleModule::change_status(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_SHOOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("shoot"), false, -1.0);
        macros::CATCH(agent, 0, Hash40::new("throw"), 2.8, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(agent, 1, Hash40::new("handr"), 1.5, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(agent, 2, Hash40::new("handr"), 4.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        GrabModule::clear(agent.module_accessor, 1);
        GrabModule::clear(agent.module_accessor, 2);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        ArticleModule::change_status_exist(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_REWIND);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("back"), false, 0.0);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_OFF_MAP_COLL_OFFSET);
    }
    frame(agent.lua_state_agent, 65.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, ArticleOperationTarget(0));
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, ArticleOperationTarget(0));
    }
}

//Dash Grab ACMD
unsafe extern "C" fn ssbexo_younglink_dash_grab_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, false, 0);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, false, 0);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        ArticleModule::change_status(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_SHOOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("shoot"), false, -1.0);
        macros::CATCH(agent, 0, Hash40::new("throw"), 2.8, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(agent, 1, Hash40::new("handr"), 1.5, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(agent, 2, Hash40::new("handr"), 4.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        GrabModule::clear(agent.module_accessor, 1);
        GrabModule::clear(agent.module_accessor, 2);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        ArticleModule::change_status_exist(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_REWIND);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("back"), false, 0.0);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_OFF_MAP_COLL_OFFSET);
    }
    frame(agent.lua_state_agent, 63.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, ArticleOperationTarget(0));
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, ArticleOperationTarget(0));
    }
}

//Pivot Grab ACMD
unsafe extern "C" fn ssbexo_younglink_pivot_grab_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, false, 0);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, false, 0);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        ArticleModule::change_status(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_SHOOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("shoot"), false, -1.0);
        macros::CATCH(agent, 0, Hash40::new("throw"), 2.8, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(agent, 1, Hash40::new("handr"), 1.5, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(agent, 2, Hash40::new("handr"), 4.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        GrabModule::clear(agent.module_accessor, 1);
        GrabModule::clear(agent.module_accessor, 2);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 53.0);
    if macros::is_excute(agent) {
        ArticleModule::change_status_exist(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_REWIND);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("back"), false, 0.0);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_OFF_MAP_COLL_OFFSET);
    }
    frame(agent.lua_state_agent, 72.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, ArticleOperationTarget(0));
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, ArticleOperationTarget(0));
    }
}

pub fn install() {
    Agent::new("younglink")
    .game_acmd("game_catch", ssbexo_younglink_grab_acmd)
    .game_acmd("game_catchdash", ssbexo_younglink_dash_grab_acmd)
    .game_acmd("game_catchturn", ssbexo_younglink_pivot_grab_acmd)
    .install()
    ;
}