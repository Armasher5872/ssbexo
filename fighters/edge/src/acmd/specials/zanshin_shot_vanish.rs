use super::*;

//Zanshin Shot Vanish ACMD
unsafe extern "C" fn ssbexo_edge_zanshin_shot_vanish_acmd(_agent: &mut L2CAgentBase) {}

//Zanshin Shot Vanish Effect
unsafe extern "C" fn ssbexo_edge_zanshin_shot_vanish_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_fire3_hold_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
}

//Zanshin Shot Vanish Sound
unsafe extern "C" fn ssbexo_edge_zanshin_shot_vanish_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_special_n02_01"));
    }
}

pub fn install() {
    Agent::new("edge_swordbeamcloned")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_vanish", ssbexo_edge_zanshin_shot_vanish_acmd, Low)
    .acmd("effect_vanish", ssbexo_edge_zanshin_shot_vanish_effect, Low)
    .acmd("sound_vanish", ssbexo_edge_zanshin_shot_vanish_sound, Low)
    .install()
    ;
}