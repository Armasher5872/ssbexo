use super::*;

//Standing Grab ACMD
unsafe extern "C" fn ssbexo_lucas_grab_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, Hash40::new("catch"), false, -1.0);
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, 0.5, Some(0.0), Some(0.0), Some(-5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    frame(lua_state, 14.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("throw"), 2.8, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 69.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Dash Grab ACMD
unsafe extern "C" fn ssbexo_lucas_dash_grab_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, Hash40::new("catch_dash"), false, -1.0);
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, 1.0, Some(0.0), Some(0.0), Some(-5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    frame(lua_state, 16.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("throw"), 2.8, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 79.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Pivot Grab ACMD
unsafe extern "C" fn ssbexo_lucas_pivot_grab_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, Hash40::new("catch_turn"), false, -1.0);
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, 1.0, Some(0.0), Some(0.0), Some(-5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    frame(lua_state, 16.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("throw"), 2.8, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 79.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

pub fn install() {
    Agent::new("lucas")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_catch", ssbexo_lucas_grab_acmd, Low)
    .acmd("game_catchdash", ssbexo_lucas_dash_grab_acmd, Low)
    .acmd("game_catchturn", ssbexo_lucas_pivot_grab_acmd, Low)
    .install()
    ;
}