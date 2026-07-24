use super::*;

//Zanshin Shot Fly ACMD
unsafe extern "C" fn ssbexo_edge_zanshin_shot_fly_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 0, 0, 0, 5.0, 0.0, 7.0, 0.9, Some(0.0), Some(1.0), Some(3.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 361, 0, 0, 0, 5.0, 0.0, -3.7, 2.2, Some(0.0), Some(1.0), Some(3.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
}

//Zanshin Shot Fly Effect
unsafe extern "C" fn ssbexo_edge_zanshin_shot_fly_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_zanshin_wave"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
    }
}

//Zanshin Shot Fly Sound
unsafe extern "C" fn ssbexo_edge_zanshin_shot_fly_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_edge_attackdash02"));
    }
}

pub fn install() {
    Agent::new("edge_swordbeamcloned")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_fly", ssbexo_edge_zanshin_shot_fly_acmd, Low)
    .acmd("effect_fly", ssbexo_edge_zanshin_shot_fly_effect, Low)
    .acmd("sound_fly", ssbexo_edge_zanshin_shot_fly_sound, Low)
    .install()
    ;
}