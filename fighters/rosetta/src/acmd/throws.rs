use super::*;

//Standing Grab ACMD
unsafe extern "C" fn ssbexo_rosetta_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 3.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 5.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.8, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(10.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 20.0);
    FT_MOTION_RATE(agent, 1.05);
    frame(agent.lua_state_agent, 40.0);
    FT_MOTION_RATE(agent, 1.0);
}

//Dash Grab ACMD
unsafe extern "C" fn ssbexo_rosetta_dash_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 3.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 5.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 2.6, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(10.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 18.0);
    FT_MOTION_RATE(agent, 1.04);
    frame(agent.lua_state_agent, 43.0);
    FT_MOTION_RATE(agent, 1.0);
}

//Pivot Grab ACMD
unsafe extern "C" fn ssbexo_rosetta_pivot_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 3.0);
    FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.8, 0.0, 8.0, -4.0, Some(0.0), Some(8.0), Some(-17.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 20.0);
    FT_MOTION_RATE(agent, 1.05);
    frame(agent.lua_state_agent, 40.0);
    FT_MOTION_RATE(agent, 1.0);
}

pub fn install() {
    Agent::new("rosetta")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_catch", ssbexo_rosetta_grab_acmd, Low)
    .game_acmd("game_catchdash", ssbexo_rosetta_dash_grab_acmd, Low)
    .game_acmd("game_catchturn", ssbexo_rosetta_pivot_grab_acmd, Low)
    .install()
    ;
}