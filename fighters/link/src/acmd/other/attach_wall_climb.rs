use super::*;

//Wall Climb Sound
unsafe extern "C" fn ssbexo_link_wall_climb_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    loop {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_link_step_right_s_ft"));
        }
        wait(lua_state, 5.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_link_step_left_s_ft"));
        }
        wait(lua_state, 5.0);
        wait_loop_clear(agent);
    }
}

//Wall Climb Expression
unsafe extern "C" fn ssbexo_link_wall_climb_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(lua_state, 1.0);
    loop {
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(lua_state, 6.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(lua_state, 1.0);
        wait_loop_clear(agent);
    }
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("sound_attachwallclimb", ssbexo_link_wall_climb_sound, Priority::Low)
    .acmd("expression_attachwallclimb", ssbexo_link_wall_climb_expression, Priority::Low)
    .install()
    ;
}