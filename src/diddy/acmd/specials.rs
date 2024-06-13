use super::*;

//Special Lw Laugh Sound
unsafe extern "C" fn ssbexo_diddy_special_lw_laugh_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_diddy_furafura"));
        SoundModule::set_se_pitch_ratio(agent.module_accessor, Hash40::new("vc_diddy_furafura"), 1.1);
    }
}

pub fn install() {
    Agent::new("diddy")
    .sound_acmd("sound_speciallwlaugh", ssbexo_diddy_special_lw_laugh_sound, Priority::Low)
    .sound_acmd("sound_specialairlwlaugh", ssbexo_diddy_special_lw_laugh_sound, Priority::Low)
    .install()
    ;
}