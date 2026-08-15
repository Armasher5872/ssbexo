use super::*;

//Down Taunt Sound
unsafe extern "C" fn ssbexo_cloud_down_taunt_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_appeal_l01"));
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_cloud_appeal03"));
    }
    frame(lua_state, 115.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_appeal_l02"));
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("sound_appeallwl", ssbexo_cloud_down_taunt_sound, Low)
    .acmd("sound_appeallwr", ssbexo_cloud_down_taunt_sound, Low)
    .install()
    ;
}