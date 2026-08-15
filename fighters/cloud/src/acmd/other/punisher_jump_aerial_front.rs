use super::*;

//Punisher Forward Aerial Jump Effect
unsafe extern "C" fn ssbexo_cloud_punisher_forward_aerial_jump_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Punisher Forward Aerial Jump Sound
unsafe extern "C" fn ssbexo_cloud_punisher_forward_aerial_jump_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_cloud_jump02"));
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("effect_punishjumpaerialfront", ssbexo_cloud_punisher_forward_aerial_jump_effect, Low)
    .acmd("sound_punishjumpaerialfront", ssbexo_cloud_punisher_forward_aerial_jump_sound, Low)
    .install()
    ;
}