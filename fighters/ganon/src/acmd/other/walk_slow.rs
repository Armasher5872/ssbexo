use super::*;

//Slow Walk Effect
unsafe extern "C" fn ssbexo_ganon_slow_walk_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    loop {
        frame(lua_state, 4.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footr"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(lua_state, 85.0);
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footl"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        agent.clear_lua_stack();
        wait_loop_sync_mot(lua_state);
        agent.pop_lua_stack(1);
    }
}

//Slow Walk Sound
unsafe extern "C" fn ssbexo_ganon_slow_walk_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    loop {
        frame(lua_state, 3.0);
        if is_excute(agent) {
            PLAY_STEP(agent, Hash40::new("se_ganon_step_right_s"));
        }
        frame(lua_state, 85.0);
        PLAY_STEP(agent, Hash40::new("se_ganon_step_left_s"));
        agent.clear_lua_stack();
        wait_loop_sync_mot(lua_state);
        agent.pop_lua_stack(1);
    }
}

//Slow Walk Expression
unsafe extern "C" fn ssbexo_ganon_slow_walk_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    loop {
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 10);
        }
        frame(lua_state, 62.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_walk_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 140.0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_walk_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        agent.clear_lua_stack();
        wait_loop_sync_mot(lua_state);
        agent.pop_lua_stack(1);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("effect_walkslow", ssbexo_ganon_slow_walk_effect, Low)
    .acmd("sound_walkslow", ssbexo_ganon_slow_walk_sound, Low)
    .acmd("expression_walkslow", ssbexo_ganon_slow_walk_expression, Low)
    .install()
    ;
}