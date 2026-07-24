use super::*;

//Pivot Grab Hi ACMD
unsafe extern "C" fn ssbexo_gaogaen_pivot_grab_hi_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 13.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 7.0, 0.0, 9.0, -2.0, Some(0.0), Some(13.0), Some(-17.6), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 3.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
}

//Pivot Grab Hi Sound
unsafe extern "C" fn ssbexo_gaogaen_pivot_grab_hi_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 11.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_swing_10"));
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_swing_10"));
    }
}

//Pivot Grab Hi Expression
unsafe extern "C" fn ssbexo_gaogaen_pivot_grab_hi_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("gaogaen")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_catchturnhi", ssbexo_gaogaen_pivot_grab_hi_acmd, Low)
    .acmd("sound_catchturnhi", ssbexo_gaogaen_pivot_grab_hi_sound, Low)
    .acmd("expression_catchturnhi", ssbexo_gaogaen_pivot_grab_hi_expression, Low)
    .install()
    ;
}