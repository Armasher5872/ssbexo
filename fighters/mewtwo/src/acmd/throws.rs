use super::*;

//Standing Grab ACMD
unsafe extern "C" fn ssbexo_mewtwo_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.2, 0.0, 10.0, 4.5, Some(0.0), Some(10.0), Some(10.1), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
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
unsafe extern "C" fn ssbexo_mewtwo_dash_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.3, 0.0, 8.5, 4.5, Some(0.0), Some(8.5), Some(12.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
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
unsafe extern "C" fn ssbexo_mewtwo_pivot_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.2, 0.0, 8.5, -4.5, Some(0.0), Some(8.5), Some(-17.3), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
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
    Agent::new("mewtwo")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_catch", ssbexo_mewtwo_grab_acmd, Low)
    .game_acmd("game_catchdash", ssbexo_mewtwo_dash_grab_acmd, Low)
    .game_acmd("game_catchturn", ssbexo_mewtwo_pivot_grab_acmd, Low)
    .install()
    ;
}