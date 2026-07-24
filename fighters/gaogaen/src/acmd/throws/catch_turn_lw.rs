use super::*;

//Pivot Grab Lw ACMD
unsafe extern "C" fn ssbexo_gaogaen_pivot_grab_lw_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 20, 10, 0, 4.0, 0.0, 9.0, -4.0, Some(0.0), Some(5.0), Some(-15.6), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_down_only(boma, 0, true);
        CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 9.0, -4.0, Some(0.0), Some(5.0), Some(-15.6), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
}

//Pivot Grab Lw Sound
unsafe extern "C" fn ssbexo_gaogaen_pivot_grab_lw_sound(agent: &mut L2CAgentBase) {
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

//Pivot Grab Lw Expression
unsafe extern "C" fn ssbexo_gaogaen_pivot_grab_lw_expression(agent: &mut L2CAgentBase) {
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
    .acmd("game_catchturnlw", ssbexo_gaogaen_pivot_grab_lw_acmd, Low)
    .acmd("sound_catchturnlw", ssbexo_gaogaen_pivot_grab_lw_sound, Low)
    .acmd("expression_catchturnlw", ssbexo_gaogaen_pivot_grab_lw_expression, Low)
    .install()
    ;
}