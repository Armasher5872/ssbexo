use super::*;

//Forward Smash ACMD
unsafe extern "C" fn ssbexo_ganon_forward_smash_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, -1);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 29.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 24.0, 40, 75, 0, 61, 5.0, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 24.0, 40, 75, 0, 61, 4.5, 0.0, 14.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 24.0, 40, 75, 0, 61, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Smash Effect
unsafe extern "C" fn ssbexo_ganon_forward_smash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1000, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 5, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.8, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 18, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_entry_aura"), false, false);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_trace"), Hash40::new("haver"), 0, 1.5, 0, 0, 0, 0, 1, true);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 31.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 1.5, 0, 18, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 1.5, 0, 18, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_DETACH_KIND(agent, Hash40::new("ganon_sword_trace"), -1);
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
    frame(agent.lua_state_agent, 35.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), false, false);
    }
}

//Up Smash ACMD
unsafe extern "C" fn ssbexo_ganon_up_smash_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, -1);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 24.0, 85, 71, 0, 40, 5.0, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 24.0, 78, 71, 0, 40, 4.5, 0.0, 14.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 21.0, 75, 75, 0, 40, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Up Smash Effect
unsafe extern "C" fn ssbexo_ganon_up_smash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1000, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 52.63, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 6.711, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 2, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 18, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_entry_aura"), false, false);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ganon_sword1"), Hash40::new("tex_ganon_sword2"), 7, Hash40::new("haver"), 0, 2.3, 0, Hash40::new("haver"), 0, 18.5, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
        LANDING_EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 1.5, 0, -18, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), false, false);
    }
}

//Down Smash ACMD
unsafe extern "C" fn ssbexo_ganon_down_smash_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, -1);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 25.0, 40, 45, 0, 70, 5.0, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 25.0, 40, 45, 0, 70, 4.5, 0.0, 12.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("top"), 8.5, 79, 105, 0, 34, 5.5, 0.0, 5.0, -7.0, Some(0.0), Some(5.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 3, 0, Hash40::new("top"), 7.0, 37, 150, 0, 30, 5.0, 0.0, 5.0, -15.0, Some(0.0), Some(5.0), Some(16.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 5, 0.2);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Smash Effect
unsafe extern "C" fn ssbexo_ganon_down_smash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1000, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 9.533, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 2.835, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.703, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 18, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.298, true);
       EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.111, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_entry_aura"), false, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 12, 0, -90, 0, 0, 1, true);
        LAST_PARTICLE_SET_COLOR(agent, 2, 1, 0.5);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("ganon_ground_smash_crack"), Hash40::new("top"), -2, 0, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_attack_elec"), Hash40::new("top"), 0, 3, -12, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_attack_elec"), Hash40::new("top"), 0, 3, 12, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_attack_elec"), Hash40::new("top"), 0, 3, -6, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_attack_elec"), Hash40::new("top"), 0, 3, 6, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_attack_elec"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1.5, true);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), false, false);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_attack_elec"), Hash40::new("top"), 0, 3, -12, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_attack_elec"), Hash40::new("top"), 0, 3, 12, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_attack_elec"), Hash40::new("top"), 0, 3, -6, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_attack_elec"), Hash40::new("top"), 0, 3, 6, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_attack_elec"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1.5, true);
    }
}

//Down Smash Sound
unsafe extern "C" fn ssbexo_ganon_down_smash_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start"));
    }
     frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_smash_l01"));
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        let rand = sv_math::rand(hash40("fighter"), 100);
        if (10..=100).contains(&rand) {
            PLAY_SE(agent, Hash40::new("vc_ganon_win02"));
        }
        else {
            PLAY_SE(agent, Hash40::new("vc_ganon_attack07"));
        }
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_appeal_s01"));
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_smash_l02"));
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_appeal_h02"));
    }
}

//Down Smash Expression
unsafe extern "C" fn ssbexo_ganon_down_smash_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 10.0);
    execute(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 10, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attacks4", ssbexo_ganon_forward_smash_acmd, Low)
    .effect_acmd("effect_attacks4", ssbexo_ganon_forward_smash_effect, Low)
    .game_acmd("game_attackhi4", ssbexo_ganon_up_smash_acmd, Low)
    .effect_acmd("effect_attackhi4", ssbexo_ganon_up_smash_effect, Low)
    .game_acmd("game_attacklw4", ssbexo_ganon_down_smash_acmd, Low)
    .effect_acmd("effect_attacklw4", ssbexo_ganon_down_smash_effect, Low)
    .sound_acmd("sound_attacklw4", ssbexo_ganon_down_smash_sound, Low)
    .expression_acmd("expression_attacklw4", ssbexo_ganon_down_smash_expression, Low)
    .install()
    ;
}