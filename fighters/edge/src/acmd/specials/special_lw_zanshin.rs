use super::*;

//Zanshin ACMD
unsafe extern "C" fn ssbexo_edge_retaliation_stance_zanshin_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, FIGHTER_EDGE_GENERATE_ARTICLE_ZANSHIN_SHOT, false, -1);
        ATTACK(agent, 0, 0, Hash40::new("swordr1"), 11.0, 67, 56, 0, 45, 4.0, 0.0, -1.0, 4.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("swordr1"), 11.0, 67, 56, 0, 45, 4.0, 7.0, -1.0, 4.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("swordr1"), 11.0, 67, 56, 0, 45, 4.0, 15.0, -1.0, 4.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("swordr1"), 14.0, 67, 87, 0, 56, 4.0, 21.5, -1.0, 4.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//Grounded Zanshin Effect
unsafe extern "C" fn ssbexo_edge_grounded_retaliation_stance_zanshin_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_smash_flash"), Hash40::new("swordr2"), 0, 0, 0, 0, 0, -90, 1, true);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_edge_sword1"), Hash40::new("tex_edge_sword2"), 6, Hash40::new("swordr2"), -4, 0, -0.7, Hash40::new("swordr2"), 29.2, 0, 1.5, true, Hash40::new("null"), Hash40::new("swordr2"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_flare"), Hash40::new("swordr2"), 0, 0, 0, 0, 0, -90, 1, true);
        LAST_EFFECT_SET_RATE(agent, 3);
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_light3"), Hash40::new("swordr2"), 0, 0, 0, 0, 0, -90, 1, true);
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
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
}

//Aerial Zanshin Effect
unsafe extern "C" fn ssbexo_edge_aerial_retaliation_stance_zanshin_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_smash_flash"), Hash40::new("swordr2"), 0, 0, 0, 0, 0, -90, 1, true);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_edge_sword1"), Hash40::new("tex_edge_sword2"), 6, Hash40::new("swordr2"), -4, 0, -0.7, Hash40::new("swordr2"), 29.2, 0, 1.5, true, Hash40::new("null"), Hash40::new("swordl2"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_flare"), Hash40::new("swordr2"), 0, 0, 0, 0, 0, -90, 1, true);
        LAST_EFFECT_SET_RATE(agent, 3);
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_light3"), Hash40::new("swordr2"), 0, 0, 0, 0, 0, -90, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("edge_sword_flare"), false, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
}

//Zanshin Sound
unsafe extern "C" fn ssbexo_edge_retaliation_stance_zanshin_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_attackair_n01"));
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_edge_rnd_attack_smash"));
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_win03_03"));
    }
}

//Zanshin Expression
unsafe extern "C" fn ssbexo_edge_retaliation_stance_zanshin_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 10.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_78_smash"), 30);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl_l"), 9, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_speciallwzanshin", ssbexo_edge_retaliation_stance_zanshin_acmd, Low)
    .acmd("game_specialairlwzanshin", ssbexo_edge_retaliation_stance_zanshin_acmd, Low)
    .acmd("effect_speciallwzanshin", ssbexo_edge_grounded_retaliation_stance_zanshin_effect, Low)
    .acmd("effect_specialairlwzanshin", ssbexo_edge_aerial_retaliation_stance_zanshin_effect, Low)
    .acmd("sound_speciallwzanshin", ssbexo_edge_retaliation_stance_zanshin_sound, Low)
    .acmd("sound_specialairlwzanshin", ssbexo_edge_retaliation_stance_zanshin_sound, Low)
    .acmd("expression_speciallwzanshin", ssbexo_edge_retaliation_stance_zanshin_expression, Low)
    .acmd("expression_specialairlwzanshin", ssbexo_edge_retaliation_stance_zanshin_expression, Low)
    .install()
    ;
}