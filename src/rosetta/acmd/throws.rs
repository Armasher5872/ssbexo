use super::*;

//Standing Grab ACMD
#[acmd_script( agent = "rosetta", script = "game_catch", category = ACMD_GAME )]
unsafe fn ssbuexo_rosetta_grab_acmd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 3.8, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(10.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    macros::FT_MOTION_RATE(fighter, 1.05);
    frame(fighter.lua_state_agent, 40.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}

//Dash Grab ACMD
#[acmd_script( agent = "rosetta", script = "game_catchdash", category = ACMD_GAME )]
unsafe fn ssbuexo_rosetta_dash_grab_acmd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 2.6, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(10.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 18.0);
    macros::FT_MOTION_RATE(fighter, 1.04);
    frame(fighter.lua_state_agent, 43.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}

//Pivot Grab ACMD
#[acmd_script( agent = "rosetta", script = "game_catchturn", category = ACMD_GAME )]
unsafe fn ssbuexo_rosetta_pivot_grab_acmd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(fighter, 0.8);
    frame(fighter.lua_state_agent, 8.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 3.8, 0.0, 8.0, -4.0, Some(0.0), Some(8.0), Some(-17.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    macros::FT_MOTION_RATE(fighter, 1.05);
    frame(fighter.lua_state_agent, 40.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_rosetta_grab_acmd,
        ssbuexo_rosetta_dash_grab_acmd,
        ssbuexo_rosetta_pivot_grab_acmd
    );
}