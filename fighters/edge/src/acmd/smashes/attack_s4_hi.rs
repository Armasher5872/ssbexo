use super::*;

//Telluric Fury ACMD
unsafe extern "C" fn ssbexo_edge_telluric_fury_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    FT_MOTION_RATE(agent, 18.0/16.0);
    frame(lua_state, 16.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 361, 60, 60, 0, 5.8, 0.0, 8.5, 6.5, Some(0.0), Some(8.5), Some(6.5), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 180, 60, 60, 0, 5.5, 0.0, 9.0, 16.5, Some(0.0), Some(9.0), Some(21.5), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 20.0);
    FT_MOTION_RATE(agent, 20.0/30.0);
    frame(lua_state, 30.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 60, 60, 0, 5.8, 0.0, 8.5, 6.5, Some(0.0), Some(8.5), Some(6.5), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 180, 60, 60, 0, 5.5, 0.0, 9.0, 16.5, Some(0.0), Some(9.0), Some(21.5), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 34.0);
    FT_MOTION_RATE(agent, 34.0/43.0);
    frame(lua_state, 43.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("swordl1"), 11.0, 61, 113, 0, 47, 4.0, 0.5, 0.0, 3.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("swordl1"), 11.0, 61, 113, 0, 47, 4.0, 7.0, 0.0, 3.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("swordl1"), 11.0, 61, 113, 0, 47, 4.0, 14.0, 0.0, 3.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("swordl1"), 11.0, 61, 113, 0, 47, 4.0, 21.0, 0.0, 3.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 46.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//Telluric Fury Effect
unsafe extern "C" fn ssbexo_edge_telluric_fury_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_smash_flash"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_edge_sword1"), Hash40::new("tex_edge_sword2"), 6, Hash40::new("swordl2"), -4, 0, -0.7, Hash40::new("swordl2"), 29.2, 0, 1.5, true, Hash40::new("null"), Hash40::new("swordl2"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_flare"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        LAST_EFFECT_SET_RATE(agent, 3);
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_light3"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.9, 10, 0, 4, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.1);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("edge_sword_flare"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword"), Hash40::new("top"), -1, 13.2, 3.5, 0, -50, 24, 0.95, true);
        LAST_EFFECT_SET_RATE(agent, 1.1);
        EFFECT_FOLLOW(agent, Hash40::new("edge_slash_light"), Hash40::new("top"), -1, 13.2, 3.5, 0, 40, 24, 1.05, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_smash_flash"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_edge_sword1"), Hash40::new("tex_edge_sword2"), 6, Hash40::new("swordl2"), -4, 0, -0.7, Hash40::new("swordl2"), 29.2, 0, 1.5, true, Hash40::new("null"), Hash40::new("swordl2"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_flare"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        LAST_EFFECT_SET_RATE(agent, 3);
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_light3"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.9, 10, 0, 4, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.1);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("edge_sword_flare"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword"), Hash40::new("top"), -1, 13.2, 3.5, 0, -50, 24, 0.95, true);
        LAST_EFFECT_SET_RATE(agent, 1.1);
        EFFECT_FOLLOW(agent, Hash40::new("edge_slash_light"), Hash40::new("top"), -1, 13.2, 3.5, 0, 40, 24, 1.05, true);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_smash_flash"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_edge_sword1"), Hash40::new("tex_edge_sword2"), 6, Hash40::new("swordl2"), -4, 0, -0.7, Hash40::new("swordl2"), 29.2, 0, 1.5, true, Hash40::new("null"), Hash40::new("swordl2"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_flare"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        LAST_EFFECT_SET_RATE(agent, 3);
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_light3"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.9, 10, 0, 4, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.1);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("edge_sword_flare"), false, true);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
}

//Telluric Fury Sound
unsafe extern "C" fn ssbexo_edge_telluric_fury_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_smash_s01"));
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_attackair_l02"));
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_smash_l02"));
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_edge_rnd_attack_smash"));
        PLAY_SE(agent, Hash40::new("se_edge_smash_h02"));
    }
}

//Telluric Fury Expression
unsafe extern "C" fn ssbexo_edge_telluric_fury_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("swordl1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_78_slash"), 1);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitll"), 4, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_78_slash"), 1);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitll"), 4, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_78_slash"), 1);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitll"), 4, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_attacks4hiwing", ssbexo_edge_telluric_fury_acmd, Low)
    .acmd("effect_attacks4hiwing", ssbexo_edge_telluric_fury_effect, Low)
    .acmd("sound_attacks4hiwing", ssbexo_edge_telluric_fury_sound, Low)
    .acmd("expression_attacks4hiwing", ssbexo_edge_telluric_fury_expression, Low)
    .install()
    ;
}