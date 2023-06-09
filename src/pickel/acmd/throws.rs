use super::*;

//Standing Grab ACMD
#[acmd_script( agent = "pickel", script = "game_catch", category = ACMD_GAME )]
unsafe fn ssbuexo_pickel_grab_acmd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, false, 0);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("catch"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PICKEL_STATUS_CATCH_FLAG_SHOOT);
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("rod"), Hash40::new("rod_cast"), ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        macros::CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -4.0, -2.5, Some(0.0), Some(-4.8), Some(-2.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("top"), 1.8, 0.0, 7.0, 3.2, Some(0.0), Some(7.0), Some(10.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 2, Hash40::new("top"), 3.6, 0.0, 7.0, 5.0, Some(0.0), Some(7.0), Some(9.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        GrabModule::clear(fighter.module_accessor, 1);
        GrabModule::clear(fighter.module_accessor, 2);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -5.0, -2.5, Some(0.0), Some(-4.0), Some(-2.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -3.0, -2.5, Some(0.0), Some(-0.5), Some(-2.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -3.0, -2.5, Some(0.0), Some(0.0), Some(-2.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 38.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, true, *WEAPON_PICKEL_FISHINGROD_INSTANCE_WORK_ID_FLAG_ENABLE_REWIND);
    }
    frame(fighter.lua_state_agent, 45.0);
    macros::FT_MOTION_RATE(fighter, 2.0);
}

//Dash Grab ACMD
#[acmd_script( agent = "pickel", script = "game_catchdash", category = ACMD_GAME )]
unsafe fn ssbuexo_pickel_dash_grab_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, false, 0);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("catch_dash"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PICKEL_STATUS_CATCH_FLAG_SHOOT);
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("rod"), Hash40::new("rod_cast"), ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        macros::CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -5.0, -2.5, Some(0.0), Some(-4.8), Some(-2.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("top"), 1.8, 0.0, 7.0, 3.2, Some(0.0), Some(7.0), Some(10.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 2, Hash40::new("top"), 3.6, 0.0, 7.0, 5.0, Some(0.0), Some(7.0), Some(9.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        GrabModule::clear(fighter.module_accessor, 1);
        GrabModule::clear(fighter.module_accessor, 2);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -5.0, -2.5, Some(0.0), Some(-4.0), Some(-2.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -3.0, -2.5, Some(0.0), Some(-0.5), Some(-2.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -3.0, -2.5, Some(0.0), Some(0.0), Some(-2.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);   
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, true, *WEAPON_PICKEL_FISHINGROD_INSTANCE_WORK_ID_FLAG_ENABLE_REWIND);
    }
    frame(fighter.lua_state_agent, 51.0);
    macros::FT_MOTION_RATE(fighter, 2.0);
}

//Pivot Grab ACMD
#[acmd_script( agent = "pickel", script = "game_catchturn", category = ACMD_GAME )]
unsafe fn ssbuexo_pickel_pivot_grab_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, false, 0);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("catch_turn"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PICKEL_STATUS_CATCH_FLAG_SHOOT);
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("rod"), Hash40::new("rod_cast"), ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        macros::CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -5.0, 2.5, Some(0.0), Some(-4.8), Some(2.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("top"), 1.8, 0.0, 7.0, -3.2, Some(0.0), Some(7.0), Some(-10.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 2, Hash40::new("top"), 3.6, 0.0, 7.0, -5.0, Some(0.0), Some(7.0), Some(-9.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        GrabModule::clear(fighter.module_accessor, 1);
        GrabModule::clear(fighter.module_accessor, 2);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -5.0, -2.5, Some(0.0), Some(-4.0), Some(-2.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -3.0, -2.5, Some(0.0), Some(-0.5), Some(-2.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -3.0, -2.5, Some(0.0), Some(0.0), Some(-2.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);   
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, true, *WEAPON_PICKEL_FISHINGROD_INSTANCE_WORK_ID_FLAG_ENABLE_REWIND);
    }
    frame(fighter.lua_state_agent, 52.0);
    macros::FT_MOTION_RATE(fighter, 2.0);
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_pickel_grab_acmd,
        ssbuexo_pickel_dash_grab_acmd,
        ssbuexo_pickel_pivot_grab_acmd
    );
}