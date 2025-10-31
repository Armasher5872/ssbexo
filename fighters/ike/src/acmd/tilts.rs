use super::*;

//Forward Tilt Sound
unsafe extern "C" fn ssbexo_ike_forward_tilt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        let pull = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ike_appeal_pullout"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, pull as i32, 0.7, 0);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        let swing = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ike_appeal_stab"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, swing as i32, 1.2, 0);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_ike_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_ike_swing_l"));
    }
}

//Up Tilt Sound
unsafe extern "C" fn ssbexo_ike_up_tilt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        let pull = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ike_appeal_pullout"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, pull as i32, 0.7, 0);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        let swing = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ike_appeal_stab"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, swing as i32, 1.0, 0);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_ike_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_ike_swing_l"));
    }
}

//Down Tilt Sound
unsafe extern "C" fn ssbexo_ike_down_tilt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        let pull = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ike_appeal_pullout"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, pull as i32, 0.6, 0);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        let swing = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ike_appeal_stab"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, swing as i32, 0.9, 0);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_ike_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_ike_swing_l"));
    }
}

pub fn install() {
    Agent::new("ike")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .sound_acmd("sound_attacks3", ssbexo_ike_forward_tilt_sound, Low)
    .sound_acmd("sound_attacks3hi", ssbexo_ike_forward_tilt_sound, Low)
    .sound_acmd("sound_attacks3lw", ssbexo_ike_forward_tilt_sound, Low)
    .sound_acmd("sound_attackhi3", ssbexo_ike_up_tilt_sound, Low)
    .sound_acmd("sound_attacklw3", ssbexo_ike_down_tilt_sound, Low)
    .install()
    ;
}