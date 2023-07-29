use super::*;

//Grab ACMD
#[acmd_script( agent = "snake", script = "game_catch", category = ACMD_GAME)]
unsafe fn ssbuexo_snake_grab_acmd(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 8.2, 7.0, Some(0.0), Some(8.2), Some(11.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

//Dash Grab ACMD
#[acmd_script( agent = "snake", script = "game_catchdash", category = ACMD_GAME)]
unsafe fn ssbuexo_snake_dash_grab_acmd(fighter: &mut L2CAgentBase) 
{
    macros::FT_MOTION_RATE(fighter, 1.222);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
        macros::CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 8.2, 7.0, Some(0.0), Some(8.2), Some(11.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(fighter, 1.3);
}

//Pivot Grab ACMD
#[acmd_script( agent = "snake", script = "game_catchturn", category = ACMD_GAME)]
unsafe fn ssbuexo_snake_pivot_grab_acmd(fighter: &mut L2CAgentBase) 
{
    macros::FT_MOTION_RATE(fighter, 1.125);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
        macros::CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 8.2, -11.0, Some(0.0), Some(8.2), Some(-17.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    macros::FT_MOTION_RATE(fighter, 1.43);
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_snake_grab_acmd,
        ssbuexo_snake_dash_grab_acmd,
        ssbuexo_snake_pivot_grab_acmd
    );
}