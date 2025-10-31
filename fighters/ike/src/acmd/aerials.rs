use super::*;

//Nair Sound
unsafe extern "C" fn ssbexo_ike_nair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        let pull = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ike_appeal_pullout"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, pull as i32, 0.6, 0);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        let swing = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ike_appeal_stab"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, swing as i32, 1.0, 0);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_ike_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_ike_swing_ll"));
    }
}

//Fair Sound
unsafe extern "C" fn ssbexo_ike_fair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        let pull = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ike_appeal_pullout"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, pull as i32, 1.0, 0);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        let swing = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ike_appeal_stab"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, swing as i32, 2.5, 0);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_ike_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_ike_attackair_f01"));
        PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
}

//Bair Sound
unsafe extern "C" fn ssbexo_ike_bair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        let pull = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ike_appeal_pullout"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, pull as i32, 0.7, 0);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        let swing = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ike_appeal_stab"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, swing as i32, 1.2, 0);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_ike_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_ike_swing_l"));
        PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
}

//Uair Sound
unsafe extern "C" fn ssbexo_ike_uair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        let pull = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ike_appeal_pullout"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, pull as i32, 0.9, 0);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        let swing = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ike_appeal_stab"), true, false, false, false, smash::app::enSEType(0));
        PLAY_SEQUENCE(agent, Hash40::new("seq_ike_rnd_attack"));
        SoundModule::set_se_vol(agent.module_accessor, swing as i32, 1.6, 0);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ike_attackair_h01"));
    }
}

//Dair Sound
unsafe extern "C" fn ssbexo_ike_dair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        let pull = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ike_appeal_pullout"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, pull as i32, 1.0, 0);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_ike_rnd_attack"));
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        let swing = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ike_appeal_stab"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, swing as i32, 1.8, 0);
    }
    wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ike_attackair_l01"));
        PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
}

pub fn install() {
    Agent::new("ike")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .sound_acmd("sound_attackairn", ssbexo_ike_nair_sound, Low)
    .sound_acmd("sound_attackairf", ssbexo_ike_fair_sound, Low)
    .sound_acmd("sound_attackairb", ssbexo_ike_bair_sound, Low)
    .sound_acmd("sound_attackairhi", ssbexo_ike_uair_sound, Low)
    .sound_acmd("sound_attackairlw", ssbexo_ike_dair_sound, Low)
    .install()
    ;
}