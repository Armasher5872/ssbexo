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
#[acmd_script( agent = "sheik", script = "game_catch", category = ACMD_GAME)]
unsafe fn ssbuexo_sheik_grab_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.7);
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 7.0);
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
        macros::CATCH(fighter, 0, Hash40::new("top"), 3.1, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(9.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    macros::FT_MOTION_RATE(fighter, 1.0);
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
        macros::FT_MOTION_RATE(fighter, 1.05);
    }
}

//Dash Grab ACMD
#[acmd_script( agent = "sheik", script = "game_catchdash", category = ACMD_GAME)]
unsafe fn ssbuexo_sheik_dash_grab_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.8);
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 11.0);
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
        macros::CATCH(fighter, 0, Hash40::new("top"), 3.0, 0.0, 7.25, 4.0, Some(0.0), Some(7.25), Some(9.1), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    macros::FT_MOTION_RATE(fighter, 1.0);
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
        macros::FT_MOTION_RATE(fighter, 1.05);
    }
}

//Pivot Grab ACMD
#[acmd_script( agent = "sheik", script = "game_catchturn", category = ACMD_GAME)]
unsafe fn ssbuexo_sheik_pivot_grab_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.9);
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 12.0);
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
        macros::CATCH(fighter, 0, Hash40::new("top"), 3.1, 0.0, 6.0, -4.0, Some(0.0), Some(6.0), Some(-13.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    macros::FT_MOTION_RATE(fighter, 1.0);
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
        macros::FT_MOTION_RATE(fighter, 1.03);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_sheik_grab_acmd,
        ssbuexo_sheik_dash_grab_acmd,
        ssbuexo_sheik_pivot_grab_acmd
    );
}