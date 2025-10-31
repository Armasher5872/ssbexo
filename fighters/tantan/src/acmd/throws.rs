use super::*;

unsafe extern "C" fn ssbexo_tantan_grab_start_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 3.5, 3.5);
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("throw"), 2.8, 0.0, -1.0, -12.0, Some(0.0), Some(-1.0), Some(-0.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("throw"), 1.4, 0.0, -1.0, -13.4, Some(0.0), Some(-1.0), Some(0.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("throw"), 2.8, 0.0, -1.0, -1.5, Some(0.0), Some(-1.0), Some(-0.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("throw"), 1.4, 0.0, -1.0, -2.9, Some(0.0), Some(-1.0), Some(0.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 51.0);
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    game_CaptureCutCommon(agent);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
    frame(agent.lua_state_agent, 60.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
}

pub fn install() {
    Agent::new("tantan")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_catchstart", ssbexo_tantan_grab_start_acmd, Low)
    .game_acmd("game_catchdashstart", ssbexo_tantan_grab_start_acmd, Low)
    .game_acmd("game_catchturnstart", ssbexo_tantan_grab_start_acmd, Low)
    .install()
    ;
}