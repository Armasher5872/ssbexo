use super::*;

//Turn Dash Sound
unsafe extern "C" fn ssbexo_ganon_turn_dash_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_dash_start"));
        SET_PLAY_INHIVIT(agent, Hash40::new("se_ganon_dash_start"), 20);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("sound_turndash", ssbexo_ganon_turn_dash_sound, Low)
    .install()
    ;
}