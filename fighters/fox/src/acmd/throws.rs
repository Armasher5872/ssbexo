use super::*;

//Standing Grab ACMD
unsafe extern "C" fn ssbexo_fox_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        FT_START_ADJUST_MOTION_FRAME_arg1(agent, 1.0);
        CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(8.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

//Dash Grab ACMD
unsafe extern "C" fn ssbexo_fox_dash_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE(agent, 0.9);
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        FT_START_ADJUST_MOTION_FRAME_arg1(agent, 1.0);
        CATCH(agent, 0, Hash40::new("top"), 2.6, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(10.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

//Pivot Grab ACMD
unsafe extern "C" fn ssbexo_fox_pivot_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE(agent, 0.9);
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        FT_START_ADJUST_MOTION_FRAME_arg1(agent, 1.0);
        CATCH(agent, 0, Hash40::new("top"), 3.3, 0.0, 6.6, -4.0, Some(0.0), Some(6.6), Some(-14.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

pub fn install() {
    Agent::new("fox")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_catch", ssbexo_fox_grab_acmd, Low)
    .game_acmd("game_catchdash", ssbexo_fox_dash_grab_acmd, Low)
    .game_acmd("game_catchturn", ssbexo_fox_pivot_grab_acmd, Low)
    .install()
    ;
}