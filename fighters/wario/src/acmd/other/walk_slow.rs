use super::*;

//Slow Walk Effect
unsafe extern "C" fn ssbexo_wario_slow_walk_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    loop {
        frame(lua_state, 60.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
        }
        frame(lua_state, 115.0);
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
        wait_loop_clear(agent);
    }
}

//Slow Walk Sound
unsafe extern "C" fn ssbexo_wario_slow_walk_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    loop {
        frame(lua_state, 60.0);
        if is_excute(agent) {
            PLAY_STEP(agent, Hash40::new("se_wario_step_right_s"));
        }
        frame(lua_state, 115.0);
        PLAY_STEP(agent, Hash40::new("se_wario_step_left_s"));
        wait_loop_clear(agent);
    }
}

//Slow Walk Expression
unsafe extern "C" fn ssbexo_wario_slow_walk_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    loop {
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
        }
        frame(lua_state, 60.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 115.0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        wait_loop_clear(agent);
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("effect_walkslow", ssbexo_wario_slow_walk_effect, Low)
    .acmd("sound_walkslow", ssbexo_wario_slow_walk_sound, Low)
    .acmd("expression_walkslow", ssbexo_wario_slow_walk_expression, Low)
    .install()
    ;
}