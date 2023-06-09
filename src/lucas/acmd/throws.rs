use super::*;

//Standing Grab ACMD
#[acmd_script( agent = "lucas", script = "game_catch", category = ACMD_GAME )]
unsafe fn ssbuexo_lucas_grab_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, false, 0);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, Hash40::new("catch"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, 0.5, Some(0.0), Some(0.0), Some(-5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 2.8, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 69.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Dash Grab ACMD
#[acmd_script( agent = "lucas", script = "game_catchdash", category = ACMD_GAME )]
unsafe fn ssbuexo_lucas_dash_grab_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, false, 0);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, Hash40::new("catch_dash"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, 1.0, Some(0.0), Some(0.0), Some(-5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 2.8, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 79.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Pivot Grab ACMD
#[acmd_script( agent = "lucas", script = "game_catchturn", category = ACMD_GAME )]
unsafe fn ssbuexo_lucas_pivot_grab_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, false, 0);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, Hash40::new("catch_turn"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, 1.0, Some(0.0), Some(0.0), Some(-5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 2.8, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 79.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_lucas_grab_acmd,
        ssbuexo_lucas_dash_grab_acmd,
        ssbuexo_lucas_pivot_grab_acmd
    );
}