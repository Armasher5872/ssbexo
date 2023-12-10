use super::*;

//Standing Tech Sound
unsafe extern "C" fn ssbexo_armstrong_passive_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_blowaway_s"));
        macros::STOP_SE(agent, Hash40::new("se_common_blowaway_m"));
        macros::STOP_SE(agent, Hash40::new("se_common_blowaway_l"));
        macros::PLAY_SE(agent, Hash40::new("vc_ganon_cliffcatch"));
        macros::PLAY_LANDING_SE(agent, Hash40::new("se_ganon_landing01"));
    }
}

//Ceiling Tech Sound
unsafe extern "C" fn ssbexo_armstrong_passive_ceil_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_blowaway_s"));
        macros::STOP_SE(agent, Hash40::new("se_common_blowaway_m"));
        macros::STOP_SE(agent, Hash40::new("se_common_blowaway_l"));
        macros::PLAY_SE(agent, Hash40::new("vc_ganon_cliffcatch"));
        macros::PLAY_LANDING_SE(agent, Hash40::new("se_ganon_landing01"));
    }
}

//Wall Tech Sound
unsafe extern "C" fn ssbexo_armstrong_passive_wall_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_blowaway_s"));
        macros::STOP_SE(agent, Hash40::new("se_common_blowaway_m"));
        macros::STOP_SE(agent, Hash40::new("se_common_blowaway_l"));
        macros::PLAY_SE(agent, Hash40::new("vc_ganon_cliffcatch"));
        macros::PLAY_SE(agent, Hash40::new("se_ganon_landing01"));
    }
}

pub fn install() {
    Agent::new("ganon")
    .sound_acmd("sound_passive", ssbexo_armstrong_passive_sound)
    .sound_acmd("sound_passiveceil", ssbexo_armstrong_passive_ceil_sound)
    .sound_acmd("sound_passivewall", ssbexo_armstrong_passive_wall_sound)
    .install()
    ;
}