use super::*;

//Standing Grab ACMD
unsafe extern "C" fn ssbexo_pacman_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.2, 10.7, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("throw"), 3.0, 0.0, 0.5, 0.0, Some(0.0), Some(-2.5), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.2, 10.7, Some(0.0), Some(6.2), Some(16.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("throw"), 3.5, 0.0, 0.5, 0.0, Some(0.0), Some(-2.5), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 27.0);
    if is_excute(agent) {
        CATCH(agent, 1, Hash40::new("throw"), 4.0, 0.0, 0.5, 0.0, Some(0.0), Some(-2.5), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 34.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

//Dash Grab ACMD
unsafe extern "C" fn ssbexo_pacman_dash_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.2, 10.7, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("throw"), 3.0, 0.0, 0.5, 0.5, Some(0.0), Some(-2.5), Some(0.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.2, 10.7, Some(0.0), Some(6.2), Some(16.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("throw"), 3.5, 0.0, 0.5, 1.5, Some(0.0), Some(-2.5), Some(1.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        CATCH(agent, 1, Hash40::new("throw"), 4.0, 0.0, 0.5, 2.0, Some(0.0), Some(-2.5), Some(2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 36.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

//Pivot Grab ACMD
unsafe extern "C" fn ssbexo_pacman_pivot_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.2, -10.7, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("throw"), 3.0, 0.0, 0.5, 0.0, Some(0.0), Some(-2.5), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.2, -10.7, Some(0.0), Some(6.2), Some(-16.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("throw"), 3.5, 0.0, 0.5, 0.0, Some(0.0), Some(-2.5), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        CATCH(agent, 1, Hash40::new("throw"), 4.0, 0.0, 0.5, 0.0, Some(0.0), Some(-2.5), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 37.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

pub fn install() {
    Agent::new("pacman")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_catch", ssbexo_pacman_grab_acmd, Low)
    .game_acmd("game_catchdash", ssbexo_pacman_dash_grab_acmd, Low)
    .game_acmd("game_catchturn", ssbexo_pacman_pivot_grab_acmd, Low)
    .install()
    ;
}