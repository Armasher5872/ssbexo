use super::*;

//Zanshin Shot Hit ACMD
unsafe extern "C" fn ssbexo_edge_zanshin_shot_hit_acmd(_agent: &mut L2CAgentBase) {}

//Zanshin Shot Hit Effect
unsafe extern "C" fn ssbexo_edge_zanshin_shot_hit_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_zanshin_hit"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
    }
}

//Zanshin Shot Hit Sound
unsafe extern "C" fn ssbexo_edge_zanshin_shot_hit_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE_REMAIN(agent, Hash40::new("se_edge_special_n02_03"));
    }
}

pub fn install() {
    Agent::new("edge_swordbeamcloned")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_hit", ssbexo_edge_zanshin_shot_hit_acmd, Low)
    .acmd("effect_hit", ssbexo_edge_zanshin_shot_hit_effect, Low)
    .acmd("sound_hit", ssbexo_edge_zanshin_shot_hit_sound, Low)
    .install()
    ;
}