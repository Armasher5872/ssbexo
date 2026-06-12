use super::*;

unsafe extern "C" fn ssbexo_tantan_grab_start_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 3.5, 3.5);
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("throw"), 2.8, 0.0, -1.0, -12.0, Some(0.0), Some(-1.0), Some(-0.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("throw"), 1.4, 0.0, -1.0, -13.4, Some(0.0), Some(-1.0), Some(0.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("throw"), 2.8, 0.0, -1.0, -1.5, Some(0.0), Some(-1.0), Some(-0.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("throw"), 1.4, 0.0, -1.0, -2.9, Some(0.0), Some(-1.0), Some(0.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 51.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, false);
    }
    game_CaptureCutCommon(agent);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
}

pub fn install() {
    Agent::new("tantan")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_catchstart", ssbexo_tantan_grab_start_acmd, Low)
    .acmd("game_catchdashstart", ssbexo_tantan_grab_start_acmd, Low)
    .acmd("game_catchturnstart", ssbexo_tantan_grab_start_acmd, Low)
    .install()
    ;
}