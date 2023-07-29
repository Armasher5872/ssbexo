use super::*;

//Special Lw Laugh Sound
#[acmd_script( agent = "diddy", script = "sound_speciallwlaugh", category = ACMD_SOUND)]
unsafe fn ssbuexo_diddy_special_lw_laugh_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_diddy_furafura"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("vc_diddy_furafura"), 1.1);
    }
}

//Special Air Lw Laugh Sound
#[acmd_script( agent = "diddy", script = "sound_specialairlwlaugh", category = ACMD_SOUND)]
unsafe fn ssbuexo_diddy_special_air_lw_laugh_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_diddy_furafura"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("vc_diddy_furafura"), 1.1);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_diddy_special_lw_laugh_sound,
        ssbuexo_diddy_special_air_lw_laugh_sound
    );
}