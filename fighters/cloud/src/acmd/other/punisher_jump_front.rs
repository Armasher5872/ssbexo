use super::*;

//Punisher Forward Fullhop Effect
unsafe extern "C" fn ssbexo_cloud_punisher_forward_fullhop_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
}

//Punisher Forward Fullhop Sound
unsafe extern "C" fn ssbexo_cloud_punisher_forward_fullhop_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 3.0);
    if !WorkModule::is_flag(boma, *FIGHTER_STATUS_JUMP_FLAG_GIMMICK_SPRING_JUMP_FROM_RING) {
        if is_excute(agent) {
            PLAY_SEQUENCE(agent, Hash40::new("seq_cloud_rnd_jump"));
            PLAY_STATUS(agent, Hash40::new("se_cloud_jump01"));
        }
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("effect_punishjumpfront", ssbexo_cloud_punisher_forward_fullhop_effect, Low)
    .acmd("sound_punishjumpfront", ssbexo_cloud_punisher_forward_fullhop_sound, Low)
    .install()
    ;
}