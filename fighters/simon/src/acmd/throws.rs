use super::*;

//Grab ACMD
unsafe extern "C" fn ssbexo_simon_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 4.0, 6.0);
        MotionModule::set_rate(agent.module_accessor, 1.43);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
        CATCH(agent, 0, Hash40::new("top"), 3.3, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(8.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
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
unsafe extern "C" fn ssbexo_simon_dash_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 6.0, 4.0);
        MotionModule::set_rate(agent.module_accessor, 1.3);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
        CATCH(agent, 0, Hash40::new("top"), 2.6, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(11.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
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
unsafe extern "C" fn ssbexo_simon_pivot_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 6.5, 6.0);
        MotionModule::set_rate(agent.module_accessor, 1.273);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
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
    Agent::new("simon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_catch", ssbexo_simon_grab_acmd, Low)
    .game_acmd("game_catchdash", ssbexo_simon_dash_grab_acmd, Low)
    .game_acmd("game_catchturn", ssbexo_simon_pivot_grab_acmd, Low)
    .install()
    ;
}