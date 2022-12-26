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
#[acmd_script( agent = "yoshi", script = "game_catch", category = ACMD_GAME)]
unsafe fn ssbuexo_yoshi_grab_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
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
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let heavy_item = [*ITEM_KIND_BARREL, *ITEM_KIND_BOX, *ITEM_KIND_CARRIERBOX, *ITEM_KIND_KUSUDAMA, *ITEM_KIND_SNAKECBOX, *ITEM_KIND_WARIOBIKE].contains(&fighter_kind);
        let character_item = [*ITEM_KIND_DIDDYPEANUTS, *ITEM_KIND_KROOLCROWN, *ITEM_KIND_LINKARROW, *ITEM_KIND_LINKBOMB, *ITEM_KIND_MECHAKOOPA, *ITEM_KIND_METALBLADE, *ITEM_KIND_PACMANAPPLE, *ITEM_KIND_PACMANBELL, *ITEM_KIND_PACMANBOSS, *ITEM_KIND_PACMANCHERRY, *ITEM_KIND_PACMANKEY, *ITEM_KIND_PACMANMELON, *ITEM_KIND_PACMANORANGE, *ITEM_KIND_PACMANSTRAWBERRY, *ITEM_KIND_RICHTERHOLYWATER, *ITEM_KIND_ROBOTGYRO, *ITEM_KIND_SIMONHOLYWATER, *ITEM_KIND_SNAKEGRENADE, *ITEM_KIND_TOONLINKBOMB, *ITEM_KIND_YOUNGLINKBOMB].contains(&fighter_kind);
        if ItemModule::is_have_item(fighter.module_accessor, 0) {
            if heavy_item == true {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP, true);
            }
            else if fighter_kind == *ITEM_KIND_GRASS {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_GRASS_PULL, true);
            }
            else if fighter_kind == *ITEM_KIND_ASSIST {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_ASSIST_HOIST, true);
            }
            else {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP, true);
            }
            if character_item == true {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP, true);
            }
        }
        macros::CATCH(fighter, 0, Hash40::new("mouth2"), 3.5, -2.0, -0.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("top"), 5.5, 0.0, 6.0, 7.5, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        GrabModule::clear(fighter.module_accessor, 1);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 48.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
}

//Dash Grab ACMD
#[acmd_script( agent = "yoshi", script = "game_catchdash", category = ACMD_GAME)]
unsafe fn ssbuexo_yoshi_dash_grab_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
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
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let heavy_item = [*ITEM_KIND_BARREL, *ITEM_KIND_BOX, *ITEM_KIND_CARRIERBOX, *ITEM_KIND_KUSUDAMA, *ITEM_KIND_SNAKECBOX, *ITEM_KIND_WARIOBIKE].contains(&fighter_kind);
        let character_item = [*ITEM_KIND_DIDDYPEANUTS, *ITEM_KIND_KROOLCROWN, *ITEM_KIND_LINKARROW, *ITEM_KIND_LINKBOMB, *ITEM_KIND_MECHAKOOPA, *ITEM_KIND_METALBLADE, *ITEM_KIND_PACMANAPPLE, *ITEM_KIND_PACMANBELL, *ITEM_KIND_PACMANBOSS, *ITEM_KIND_PACMANCHERRY, *ITEM_KIND_PACMANKEY, *ITEM_KIND_PACMANMELON, *ITEM_KIND_PACMANORANGE, *ITEM_KIND_PACMANSTRAWBERRY, *ITEM_KIND_RICHTERHOLYWATER, *ITEM_KIND_ROBOTGYRO, *ITEM_KIND_SIMONHOLYWATER, *ITEM_KIND_SNAKEGRENADE, *ITEM_KIND_TOONLINKBOMB, *ITEM_KIND_YOUNGLINKBOMB].contains(&fighter_kind);
        if ItemModule::is_have_item(fighter.module_accessor, 0) {
            if heavy_item == true {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP, true);
            }
            else if fighter_kind == *ITEM_KIND_GRASS {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_GRASS_PULL, true);
            }
            else if fighter_kind == *ITEM_KIND_ASSIST {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_ASSIST_HOIST, true);
            }
            else {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP, true);
            }
            if character_item == true {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP, true);
            }
        }
        macros::CATCH(fighter, 0, Hash40::new("mouth2"), 3.5, -2.0, -0.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("top"), 5.5, 0.0, 6.0, 7.5, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        GrabModule::clear(fighter.module_accessor, 1);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
}

//Pivot Grab ACMD
#[acmd_script( agent = "yoshi", script = "game_catchturn", category = ACMD_GAME)]
unsafe fn ssbuexo_yoshi_pivot_grab_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
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
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let heavy_item = [*ITEM_KIND_BARREL, *ITEM_KIND_BOX, *ITEM_KIND_CARRIERBOX, *ITEM_KIND_KUSUDAMA, *ITEM_KIND_SNAKECBOX, *ITEM_KIND_WARIOBIKE].contains(&fighter_kind);
        let character_item = [*ITEM_KIND_DIDDYPEANUTS, *ITEM_KIND_KROOLCROWN, *ITEM_KIND_LINKARROW, *ITEM_KIND_LINKBOMB, *ITEM_KIND_MECHAKOOPA, *ITEM_KIND_METALBLADE, *ITEM_KIND_PACMANAPPLE, *ITEM_KIND_PACMANBELL, *ITEM_KIND_PACMANBOSS, *ITEM_KIND_PACMANCHERRY, *ITEM_KIND_PACMANKEY, *ITEM_KIND_PACMANMELON, *ITEM_KIND_PACMANORANGE, *ITEM_KIND_PACMANSTRAWBERRY, *ITEM_KIND_RICHTERHOLYWATER, *ITEM_KIND_ROBOTGYRO, *ITEM_KIND_SIMONHOLYWATER, *ITEM_KIND_SNAKEGRENADE, *ITEM_KIND_TOONLINKBOMB, *ITEM_KIND_YOUNGLINKBOMB].contains(&fighter_kind);
        if ItemModule::is_have_item(fighter.module_accessor, 0) {
            if heavy_item == true {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP, true);
            }
            else if fighter_kind == *ITEM_KIND_GRASS {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_GRASS_PULL, true);
            }
            else if fighter_kind == *ITEM_KIND_ASSIST {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_ASSIST_HOIST, true);
            }
            else {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP, true);
            }
            if character_item == true {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP, true);
            }
        }
        macros::CATCH(fighter, 0, Hash40::new("mouth2"), 3.5, -2.0, -0.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("top"), 5.5, 0.0, 6.0, 7.5, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        GrabModule::clear(fighter.module_accessor, 1);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_yoshi_grab_acmd,
        ssbuexo_yoshi_dash_grab_acmd,
        ssbuexo_yoshi_pivot_grab_acmd
    );
}