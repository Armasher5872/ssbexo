use super::*;

//Up Taunt Effect
unsafe extern "C" fn ssbexo_wario_up_taunt_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 71.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("wario_kamitsuki_end"), Hash40::new("top"), 0, 10, -2, 180, 0, 0, 0.7, false);
    }
}

//Up Taunt Sound
unsafe extern "C" fn ssbexo_wario_up_taunt_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("vc_wario_appeal01"));
    }
    frame(lua_state, 65.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_appeal_h01"));
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("effect_appealhil", ssbexo_wario_up_taunt_effect, Low)
    .acmd("sound_appealhil", ssbexo_wario_up_taunt_sound, Low)
    .acmd("effect_appealhir", ssbexo_wario_up_taunt_effect, Low)
    .acmd("sound_appealhir", ssbexo_wario_up_taunt_sound, Low)
    .install()
    ;
}