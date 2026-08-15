use super::*;

//Up Taunt Effect
unsafe extern "C" fn ssbexo_ganon_up_taunt_effect(_agent: &mut L2CAgentBase) {}

//Up Taunt Sound
unsafe extern "C" fn ssbexo_ganon_up_taunt_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 24.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_appeal_h01"));
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("effect_appealhir", ssbexo_ganon_up_taunt_effect, Low)
    .acmd("effect_appealhil", ssbexo_ganon_up_taunt_effect, Low)
    .acmd("sound_appealhir", ssbexo_ganon_up_taunt_sound, Low)
    .acmd("sound_appealhil", ssbexo_ganon_up_taunt_sound, Low)
    .install()
    ;
}