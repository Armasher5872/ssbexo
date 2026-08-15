use super::*;

//Punisher Forward Shorthop Effect
unsafe extern "C" fn ssbexo_cloud_punisher_forward_shorthop_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
}

//Punisher Forward Shorthop Sound
unsafe extern "C" fn ssbexo_cloud_punisher_forward_shorthop_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_cloud_jump03"));
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("effect_punishjumpfrontmini", ssbexo_cloud_punisher_forward_shorthop_effect, Low)
    .acmd("sound_punishjumpfrontmini", ssbexo_cloud_punisher_forward_shorthop_sound, Low)
    .install()
    ;
}