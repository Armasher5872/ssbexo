use super::*;

//Winged Run ACMD
unsafe extern "C" fn ssbexo_edge_run_wing_acmd(_agent: &mut L2CAgentBase) {}

//Winged Run Effect
unsafe extern "C" fn ssbexo_edge_run_wing_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    loop {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        wait(lua_state, 13.0);
        wait_loop_clear(agent);
    }
}

//Winged Run Sound
unsafe extern "C" fn ssbexo_edge_run_wing_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_edge_step_ll_loop"));
    }
}

//Winged Run Expression
unsafe extern "C" fn ssbexo_edge_run_wing_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    loop {
        if is_excute(agent) {
            slope!(agent, 0, *SLOPE_STATUS_LR);
        }
        frame(lua_state, 1.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_furafura"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 21.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_furafura"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 41.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_furafura"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 61.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_furafura"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 81.0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_furafura"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
        wait_loop_clear(agent);
    }
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_runwing", ssbexo_edge_run_wing_acmd, Low)
    .acmd("effect_runwing", ssbexo_edge_run_wing_effect, Low)
    .acmd("sound_runwing", ssbexo_edge_run_wing_sound, Low)
    .acmd("expression_runwing", ssbexo_edge_run_wing_expression, Low)
    .install()
    ;
}