use super::*;

//Neutral Special End Sound
unsafe extern "C" fn ssbexo_link_special_n_end_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_link_special_n01"));
    }
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .sound_acmd("sound_specialnend", ssbexo_link_special_n_end_sound, Low)
    .sound_acmd("sound_specialairnend", ssbexo_link_special_n_end_sound, Low)
    .install()
    ;
}