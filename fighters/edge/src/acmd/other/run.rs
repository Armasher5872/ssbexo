use super::*;

//Run Effect
unsafe extern "C" fn ssbexo_edge_run_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    loop {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(lua_state, 14.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(lua_state, 24.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(lua_state, 37.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(lua_state, 46.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(lua_state, 60.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(lua_state, 70.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(lua_state, 80.0);
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        wait_loop_clear(agent);
    }
}

//Run Sound
unsafe extern "C" fn ssbexo_edge_run_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    loop {
        frame(lua_state, 3.0);
        if is_excute(agent) {
            PLAY_STEP(agent, Hash40::new("se_edge_step_right_l"));
        }
        frame(lua_state, 24.0);
        if is_excute(agent) {
            PLAY_STEP(agent, Hash40::new("se_edge_step_left_l"));
        }
        frame(lua_state, 45.0);
        if is_excute(agent) {
            PLAY_STEP(agent, Hash40::new("se_edge_step_right_l"));
        }
        frame(lua_state, 70.0);
        PLAY_STEP(agent, Hash40::new("se_edge_step_left_l"));
        wait_loop_clear(agent);
    }
}

//Run Expression
unsafe extern "C" fn ssbexo_edge_run_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    loop {
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
        }
        frame(lua_state, 3.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 24.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 45.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 70.0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        wait_loop_clear(agent);
    }
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("effect_run", ssbexo_edge_run_effect, Low)
    .acmd("sound_run", ssbexo_edge_run_sound, Low)
    .acmd("expression_run", ssbexo_edge_run_expression, Low)
    .install()
    ;
}