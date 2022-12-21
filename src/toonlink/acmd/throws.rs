#![allow(unused_macros)]
use {
    smash::{
        lua2cpp::L2CAgentBase, 
        phx::Hash40,
        app::{
            lua_bind::*,
            sv_animcmd::*,
            *
        },
        lib::lua_const::*,
    },
    smash_script::*,
    smashline::*,
};

//Grab ACMD
#[acmd_script( agent = "toonlink", script = "game_catch", category = ACMD_GAME)]
unsafe fn ssbuexo_toonlink_grab_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, false, 0);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, false, 0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_SHOOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("shoot"), false, -1.0);
        // if there is a heavy item to pick up nearby, grab it
        if !ItemModule::is_have_item(fighter.module_accessor, 0) {
            WorkModule::off_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_PICKUP_ITEM);
            ItemModule::pickup_item(fighter.module_accessor, ItemSize{_address: *ITEM_SIZE_HEAVY as u8}, *FIGHTER_HAVE_ITEM_WORK_MAIN, *ITEM_TRAIT_ALL, QuickItemTreatType{_address: *QUICK_ITEM_TREAT_TYPE_FORCE_HAVE as u8}, ItemPickupSearchMode{_address: *ITEM_PICKUP_SEARCH_MODE_NORMAL as u8});
        }
        // if there is a light item nearby, grab it
        if !ItemModule::is_have_item(fighter.module_accessor, 0) {
            WorkModule::off_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_PICKUP_ITEM);
            ItemModule::pickup_item(fighter.module_accessor, ItemSize{_address: *ITEM_SIZE_LIGHT as u8}, *FIGHTER_HAVE_ITEM_WORK_MAIN, *ITEM_TRAIT_ALL, QuickItemTreatType{_address: *QUICK_ITEM_TREAT_TYPE_FORCE_HAVE as u8}, ItemPickupSearchMode{_address: *ITEM_PICKUP_SEARCH_MODE_NORMAL as u8});
        }
        // if you have any items at all, transition into pickup
        if ItemModule::is_have_item(fighter.module_accessor, 0) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP, true);
        }
        macros::CATCH(fighter, 0, Hash40::new("throw"), 2.8, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("handr"), 1.5, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 2, Hash40::new("handr"), 4.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        GrabModule::clear(fighter.module_accessor, 1);
        GrabModule::clear(fighter.module_accessor, 2);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_REWIND);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("back"), false, 0.0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_OFF_MAP_COLL_OFFSET);
    }
    frame(fighter.lua_state_agent, 55.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, ArticleOperationTarget(0));
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, ArticleOperationTarget(0));
    }
}

//Dash Grab ACMD
#[acmd_script( agent = "toonlink", script = "game_catchdash", category = ACMD_GAME)]
unsafe fn ssbuexo_toonlink_dash_grab_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, false, 0);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, false, 0);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_SHOOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("back"), false, -1.0);
        // if there is a heavy item to pick up nearby, grab it
        if !ItemModule::is_have_item(fighter.module_accessor, 0) {
            WorkModule::off_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_PICKUP_ITEM);
            ItemModule::pickup_item(fighter.module_accessor, ItemSize{_address: *ITEM_SIZE_HEAVY as u8}, *FIGHTER_HAVE_ITEM_WORK_MAIN, *ITEM_TRAIT_ALL, QuickItemTreatType{_address: *QUICK_ITEM_TREAT_TYPE_FORCE_HAVE as u8}, ItemPickupSearchMode{_address: *ITEM_PICKUP_SEARCH_MODE_NORMAL as u8});
        }
        // if there is a light item nearby, grab it
        if !ItemModule::is_have_item(fighter.module_accessor, 0) {
            WorkModule::off_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_PICKUP_ITEM);
            ItemModule::pickup_item(fighter.module_accessor, ItemSize{_address: *ITEM_SIZE_LIGHT as u8}, *FIGHTER_HAVE_ITEM_WORK_MAIN, *ITEM_TRAIT_ALL, QuickItemTreatType{_address: *QUICK_ITEM_TREAT_TYPE_FORCE_HAVE as u8}, ItemPickupSearchMode{_address: *ITEM_PICKUP_SEARCH_MODE_NORMAL as u8});
        }
        // if you have any items at all, transition into pickup
        if ItemModule::is_have_item(fighter.module_accessor, 0) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP, true);
        }
        macros::CATCH(fighter, 0, Hash40::new("throw"), 2.8, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("handr"), 1.5, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 2, Hash40::new("handr"), 4.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        GrabModule::clear(fighter.module_accessor, 1);
        GrabModule::clear(fighter.module_accessor, 2);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_REWIND);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("back"), false, 0.0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_OFF_MAP_COLL_OFFSET);
    }
    frame(fighter.lua_state_agent, 63.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, ArticleOperationTarget(0));
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, ArticleOperationTarget(0));
    }
}

//Pivot Grab ACMD
#[acmd_script( agent = "toonlink", script = "game_catchturn", category = ACMD_GAME)]
unsafe fn ssbuexo_toonlink_pivot_grab_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, false, 0);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, false, 0);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_SHOOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("back"), false, -1.0);
        // if there is a heavy item to pick up nearby, grab it
        if !ItemModule::is_have_item(fighter.module_accessor, 0) {
            WorkModule::off_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_PICKUP_ITEM);
            ItemModule::pickup_item(fighter.module_accessor, ItemSize{_address: *ITEM_SIZE_HEAVY as u8}, *FIGHTER_HAVE_ITEM_WORK_MAIN, *ITEM_TRAIT_ALL, QuickItemTreatType{_address: *QUICK_ITEM_TREAT_TYPE_FORCE_HAVE as u8}, ItemPickupSearchMode{_address: *ITEM_PICKUP_SEARCH_MODE_NORMAL as u8});
        }
        // if there is a light item nearby, grab it
        if !ItemModule::is_have_item(fighter.module_accessor, 0) {
            WorkModule::off_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_PICKUP_ITEM);
            ItemModule::pickup_item(fighter.module_accessor, ItemSize{_address: *ITEM_SIZE_LIGHT as u8}, *FIGHTER_HAVE_ITEM_WORK_MAIN, *ITEM_TRAIT_ALL, QuickItemTreatType{_address: *QUICK_ITEM_TREAT_TYPE_FORCE_HAVE as u8}, ItemPickupSearchMode{_address: *ITEM_PICKUP_SEARCH_MODE_NORMAL as u8});
        }
        // if you have any items at all, transition into pickup
        if ItemModule::is_have_item(fighter.module_accessor, 0) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP, true);
        }
        macros::CATCH(fighter, 0, Hash40::new("throw"), 2.8, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("handr"), 1.5, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 2, Hash40::new("handr"), 4.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        GrabModule::clear(fighter.module_accessor, 1);
        GrabModule::clear(fighter.module_accessor, 2);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_REWIND);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("back"), false, 0.0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_OFF_MAP_COLL_OFFSET);
    }
    frame(fighter.lua_state_agent, 58.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, ArticleOperationTarget(0));
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, ArticleOperationTarget(0));
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_toonlink_grab_acmd,
        ssbuexo_toonlink_dash_grab_acmd,
        ssbuexo_toonlink_pivot_grab_acmd
    );
}