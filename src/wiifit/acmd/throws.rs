use super::*;

//Grab ACMD
unsafe extern "C" fn ssbexo_wiifit_grab_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.3, 0.0, 8.6, 4.0, Some(0.0), Some(8.6), Some(8.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

//Dash Grab ACMD
unsafe extern "C" fn ssbexo_wiifit_dash_grab_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 2.6, 0.0, 7.5, 4.0, Some(0.0), Some(7.5), Some(9.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

//Pivot Grab ACMD
unsafe extern "C" fn ssbexo_wiifit_pivot_grab_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.3, 0.0, 8.5, -4.0, Some(0.0), Some(8.5), Some(-14.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

pub fn install() {
    Agent::new("wiifit")
    .game_acmd("game_catch", ssbexo_wiifit_grab_acmd)
    .game_acmd("game_catchdash", ssbexo_wiifit_dash_grab_acmd)
    .game_acmd("game_catchturn", ssbexo_wiifit_pivot_grab_acmd)
    .install()
    ;
}