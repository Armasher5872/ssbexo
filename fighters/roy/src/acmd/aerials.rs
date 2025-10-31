use super::*;

//Nair ACMD
unsafe extern "C" fn ssbexo_roy_nair_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 80, 30, 0, 52, 3.5, 0.0, 9.6, 8.0, Some(0.0), Some(11.7), Some(3.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword1"), 4.0, 80, 30, 0, 52, 3.5, 0.0, 0.0, 6.8, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 6.0, 80, 30, 0, 52, 4.8, 0.0, 9.6, 6.8, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 8.5, 50, 105, 0, 50, 5.6, 0.0, 0.0, -1.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword1"), 8.5, 50, 105, 0, 50, 4.2, 2.5, 0.0, 0.7, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword1"), 5.0, 50, 100, 0, 50, 4.2, 2.5, 0.0, 7.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 35.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Nair Effect
unsafe extern "C" fn ssbexo_roy_nair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 10, Hash40::new("sword1"), 0.0, 0.0, -0.8, Hash40::new("sword1"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
        EFFECT_FOLLOW(agent, Hash40::new("roy_sword_light"), Hash40::new("sword1"), 0, 0, 10.5, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.6);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
        EFFECT_OFF_KIND(agent, Hash40::new("roy_sword_light"), false, true);
    }
}

//Nair Sound
unsafe extern "C" fn ssbexo_roy_nair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_roy_rnd_attack_air"));
        PLAY_SE(agent, Hash40::new("se_roy_attackair_n01"));
    }
}

//Nair Expression
unsafe extern "C" fn ssbexo_roy_nair_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

//Fair ACMD
unsafe extern "C" fn ssbexo_roy_fair_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    FT_MOTION_RATE(agent, 0.66);
    frame(agent.lua_state_agent, 12.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 361, 83, 0, 50, 4.8, 0.0, 13.0, 4.0, Some(0.0), Some(10.0), Some(4.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 11.0, 361, 83, 0, 50, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword1"), 11.0, 361, 83, 0, 50, 3.5, 0.0, 0.0, 0.7, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword1"), 7.0, 361, 80, 0, 50, 3.5, 0.0, 0.0, 7.2, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 34.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Fair Effect
unsafe extern "C" fn ssbexo_roy_fair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 7, Hash40::new("sword1"), 0.0, 0.0, -0.8, Hash40::new("sword1"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
}

//Bair ACMD
unsafe extern "C" fn ssbexo_roy_bair_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 361, 107, 0, 35, 4.2, 0.0, 10.5, -7.0, Some(0.0), Some(10.5), Some(-4.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword1"), 12.0, 361, 107, 0, 35, 4.0, 2.5, 0.0, 0.4, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword1"), 9.0, 361, 100, 0, 10, 4.0, 2.5, 0.0, 7.2, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 42.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Bair Effect
unsafe extern "C" fn ssbexo_roy_bair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 7, Hash40::new("sword1"), 0.0, 0.0, -0.8, Hash40::new("sword1"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
        EFFECT_FOLLOW(agent, Hash40::new("roy_sword_light"), Hash40::new("sword1"), 0, 0, 10.5, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.6);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
        EFFECT_OFF_KIND(agent, Hash40::new("roy_sword_light"), false, true);
    }
}

//Bair Sound
unsafe extern "C" fn ssbexo_roy_bair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_swing_09"));
    }
    wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_roy_rnd_attack_air"));
        PLAY_SE(agent, Hash40::new("se_roy_attackair_b01"));
    }
}

//Bair Expression
unsafe extern "C" fn ssbexo_roy_bair_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
}

//Uair Effect
unsafe extern "C" fn ssbexo_roy_uair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 8, Hash40::new("sword1"), 0.0, 0.0, -0.8, Hash40::new("sword1"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
}

//Dair ACMD
unsafe extern "C" fn ssbexo_roy_dair_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 270, 90, 0, 20, 4.4, 0.0, -0.7, 0.4, Some(0.0), Some(-2.5), Some(0.4), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 361, 90, 0, 20, 4.4, 0.0, -0.7, 0.4, Some(0.0), Some(-2.5), Some(0.4), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 361, 90, 0, 30, 5.4, 0.0, -6.8, 0.4, Some(0.0), Some(-1.0), Some(0.4), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 52.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Dair Effect
unsafe extern "C" fn ssbexo_roy_dair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 20, Hash40::new("sword1"), 0.0, 0.0, -0.8, Hash40::new("sword1"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.2);
        EFFECT_FOLLOW(agent, Hash40::new("roy_sword_light"), Hash40::new("sword1"), 0, 0, 10.55, 0, 0, 0, 1.05, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.6);
        LAST_EFFECT_SET_RATE(agent, 1.1);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("roy_sword_light"), false, true);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
}

pub fn install() {
    Agent::new("roy")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attackairn", ssbexo_roy_nair_acmd, Low)
    .effect_acmd("effect_attackairn", ssbexo_roy_nair_effect, Low)
    .sound_acmd("sound_attackairn", ssbexo_roy_nair_sound, Low)
    .expression_acmd("expression_attackairn", ssbexo_roy_nair_expression, Low)
    .game_acmd("game_attackairf", ssbexo_roy_fair_acmd, Low)
    .effect_acmd("effect_attackairf", ssbexo_roy_fair_effect, Low)
    .game_acmd("game_attackairb", ssbexo_roy_bair_acmd, Low)
    .effect_acmd("effect_attackairb", ssbexo_roy_bair_effect, Low)
    .sound_acmd("sound_attackairb", ssbexo_roy_bair_sound, Low)
    .expression_acmd("expression_attackairb", ssbexo_roy_bair_expression, Low)
    .effect_acmd("effect_attackairhi", ssbexo_roy_uair_effect, Low)
    .game_acmd("game_attackairlw", ssbexo_roy_dair_acmd, Low)
    .effect_acmd("effect_attackairlw", ssbexo_roy_dair_effect, Low)
    .install()
    ;
}