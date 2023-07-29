use super::*;

//Grab ACMD
#[acmd_script( agent = "roy", script = "game_catch", category = ACMD_GAME)]
unsafe fn ssbuexo_roy_grab_acmd(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(7.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

//Dash Grab ACMD
#[acmd_script( agent = "roy", script = "game_catchdash", category = ACMD_GAME)]
unsafe fn ssbuexo_roy_dash_grab_acmd(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 4.0, 4.0);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 3.0, 0.0, 7.25, 4.0, Some(0.0), Some(7.25), Some(9.1), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

//Pivot Grab ACMD
#[acmd_script( agent = "roy", script = "game_catchturn", category = ACMD_GAME)]
unsafe fn ssbuexo_roy_pivot_grab_acmd(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 7.0, -4.0, Some(0.0), Some(7.0), Some(-12.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_roy_grab_acmd,
        ssbuexo_roy_dash_grab_acmd,
        ssbuexo_roy_pivot_grab_acmd
    );
}