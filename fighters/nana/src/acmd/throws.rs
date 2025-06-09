use super::*;

//Standing Grab ACMD
unsafe extern "C" fn ssbexo_nana_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        CATCH(agent, 0, Hash40::new("top"), 3.6, 0.0, 5.0, 4.0, Some(0.0), Some(5.0), Some(6.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
        FT_MOTION_RATE(agent, 1.32);
    }
}

//Dash Grab ACMD
unsafe extern "C" fn ssbexo_nana_dash_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        CATCH(agent, 0, Hash40::new("top"), 2.9, 0.0, 5.0, 4.0, Some(0.0), Some(5.0), Some(8.1), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
        FT_MOTION_RATE(agent, 1.21);
    }
}

//Pivot Grab ACMD
unsafe extern "C" fn ssbexo_nana_pivot_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE(agent, 1.125);
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        CATCH(agent, 0, Hash40::new("top"), 3.6, 0.0, 5.0, -4.0, Some(0.0), Some(5.0), Some(-11.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
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
    Agent::new("nana")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_catch", ssbexo_nana_grab_acmd, Low)
    .game_acmd("game_catchdash", ssbexo_nana_dash_grab_acmd, Low)
    .game_acmd("game_catchturn", ssbexo_nana_pivot_grab_acmd, Low)
    .install()
    ;
}