use super::*;

//Standing Grab ACMD
unsafe extern "C" fn ssbexo_eflame_grab_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 2);
    frame(lua_state, 2.0);
    FT_MOTION_RATE(agent, 1);
    frame(lua_state, 6.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.3, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(8.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

//Dash Grab ACMD
unsafe extern "C" fn ssbexo_eflame_dash_grab_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.6, 3.0, Some(0.0), Some(6.6), Some(11.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

//Pivot Grab ACMD
unsafe extern "C" fn ssbexo_eflame_pivot_grab_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.3, 0.0, 6.6, -6.0, Some(0.0), Some(6.6), Some(-14.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

pub fn install() {
    Agent::new("eflame")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_catch", ssbexo_eflame_grab_acmd, Low)
    .acmd("game_catchdash", ssbexo_eflame_dash_grab_acmd, Low)
    .acmd("game_catchturn", ssbexo_eflame_pivot_grab_acmd, Low)
    .install()
    ;
}