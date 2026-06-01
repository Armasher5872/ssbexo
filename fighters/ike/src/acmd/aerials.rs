use super::*;

//Fair Sound
unsafe extern "C" fn ssbexo_ike_fair_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let pull = SoundModule::play_se(boma, Hash40::new("se_ike_appeal_pullout"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(boma, pull as i32, 1.0, 0);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        let swing = SoundModule::play_se(boma, Hash40::new("se_ike_appeal_stab"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(boma, swing as i32, 2.5, 0);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_ike_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_ike_attackair_f01"));
        PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
}

//Uair Sound
unsafe extern "C" fn ssbexo_ike_uair_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        let pull = SoundModule::play_se(boma, Hash40::new("se_ike_appeal_pullout"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(boma, pull as i32, 0.9, 0);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        let swing = SoundModule::play_se(boma, Hash40::new("se_ike_appeal_stab"), true, false, false, false, smash::app::enSEType(0));
        PLAY_SEQUENCE(agent, Hash40::new("seq_ike_rnd_attack"));
        SoundModule::set_se_vol(boma, swing as i32, 1.6, 0);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ike_attackair_h01"));
    }
}

pub fn install() {
    Agent::new("ike")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .sound_acmd("sound_attackairf", ssbexo_ike_fair_sound, Low)
    .sound_acmd("sound_attackairhi", ssbexo_ike_uair_sound, Low)
    .install()
    ;
}