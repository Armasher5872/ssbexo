use super::*;

//Side Special Air Loop ACMD
unsafe extern "C" fn ssbexo_wario_side_special_air_loop_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        SEARCH(agent, 0, 0, Hash40::new("top"), 10.0, 0.0, 7.0, 0.0, Some(0.0), Some(7.0), Some(10.0), *COLLISION_KIND_MASK_ALL, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true);
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 35, 89, 0, 42, 4.0, 0.0, 5.0, 5.8, Some(0.0), Some(9.7), Some(5.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 35, 89, 0, 42, 4.0, 0.0, 6.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.0);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 0.0);
    }
}

//Side Special Air Loop Effect
unsafe extern "C" fn ssbexo_wario_side_special_air_loop_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("wario_attack_dash"), Hash40::new("top"), 0, 4, 16, 0, 0, 0, 1, true);
    }
}

//Side Special Air Loop Sound
unsafe extern "C" fn ssbexo_wario_side_special_air_loop_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        let charge_sfx = SoundModule::play_se(agent.module_accessor, Hash40::new("se_wario_special_s04"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, charge_sfx as i32, 1.27, 0);
    }
}

//Side Special Air Loop Expression
unsafe extern "C" fn ssbexo_wario_side_special_air_loop_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialsairloop", ssbexo_wario_side_special_air_loop_acmd, Low)
    .effect_acmd("effect_specialsairloop", ssbexo_wario_side_special_air_loop_effect, Low)
    .sound_acmd("sound_specialsairloop", ssbexo_wario_side_special_air_loop_sound, Low)
    .expression_acmd("expression_specialsairloop", ssbexo_wario_side_special_air_loop_expression, Low)
    .install()
    ;
}