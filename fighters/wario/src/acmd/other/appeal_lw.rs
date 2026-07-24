use super::*;

//Down Taunt Effect
unsafe extern "C" fn ssbexo_wario_down_taunt_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("wario_kamitsuki_end"), Hash40::new("top"), 4, 8.5, 8, -90, 0, 0, 0.7, false);
        EFFECT_FOLLOW(agent, Hash40::new("wario_kamitsuki_end"), Hash40::new("top"), 0, 6.5, 0, -90, 0, 0, 0.7, false);
    }
}

//Down Taunt Sound
unsafe extern "C" fn ssbexo_wario_down_taunt_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 18.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_wario_011"));
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("effect_appeallwl", ssbexo_wario_down_taunt_effect, Low)
    .acmd("sound_appeallwl", ssbexo_wario_down_taunt_sound, Low)
    .acmd("effect_appeallwr", ssbexo_wario_down_taunt_effect, Low)
    .acmd("sound_appeallwr", ssbexo_wario_down_taunt_sound, Low)
    .install()
    ;
}