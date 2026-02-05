use super::*;

//Nair ACMD
unsafe extern "C" fn ssbexo_ganon_nair_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }  
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        ATTACK (agent, 0, 0, Hash40::new("top"), 10.0, 64, 68, 0, 45, 11.0, 0.0, 12.0, 2.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BODY);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 2.0, false);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 55.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Nair Effect
unsafe extern "C" fn ssbexo_ganon_nair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_appeal_aura"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 2.5, true);
    }
}

//Nair Sound
unsafe extern "C" fn ssbexo_ganon_nair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        let rand = sv_math::rand(hash40("fighter"), 100);
        if (67..=100).contains(&rand) {
            PLAY_SE(agent, Hash40::new("vc_ganon_special_h01"));
        } 
        else if (34..=66).contains(&rand) {
            PLAY_SE(agent, Hash40::new("vc_ganon_attack01"));
        } 
        else {
            PLAY_SE(agent, Hash40::new("invalid"));
        }
        PLAY_SE(agent, Hash40::new("se_ganon_appeal_h02"));
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_swing_l"));
    }
}

//Nair Expression
unsafe extern "C" fn ssbexo_ganon_nair_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

//Fair ACMD
unsafe extern "C" fn ssbexo_ganon_fair_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 14.0, 361, 93, 0, 20, 4.0, -1.1, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 15.0, 361, 93, 0, 20, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("arml"), 15.0, 361, 93, 0, 20, 4.0, 8.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 3, 0, Hash40::new("top"), 15.0, 361, 93, 0, 20, 7.0, 0.0, 11.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 3, false);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 45.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Fair Effect
unsafe extern "C" fn ssbexo_ganon_fair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_flash"), Hash40::new("arml"), 7, 0, 0, 0, 0, 0, 1.6, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 12, 6, 180, -180, 100, 1.1, false);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_engokua_flash"), false, false);
    }
}

//Fair Sound
unsafe extern "C" fn ssbexo_ganon_fair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_ganon_swing_m"));
    }
}

//Fair Expression
unsafe extern "C" fn ssbexo_ganon_fair_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

//Bair ACMD
unsafe extern "C" fn ssbexo_ganon_bair_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, -1);
        MotionModule::set_rate(agent.module_accessor, 1.4);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 18.0, 361, 55, 0, 61, 5.0, 0.0, 6.5, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 18.0, 361, 55, 0, 61, 4.5, 0.0, 14.5, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 18.0, 361, 55, 0, 61, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 11.0, 55, 55, 0, 61, 5.0, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 11.0, 55, 55, 0, 61, 4.5, 0.0, 14.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 11.0, 55, 55, 0, 61, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 33.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Bair Effect
unsafe extern "C" fn ssbexo_ganon_bair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1000, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 39.66, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 9.49, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 5.524, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 3.344, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 2.325, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.763, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.424, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.213, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.079, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_entry_aura"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), true, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_entry_aura"), false, false);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -4, 11.5, -8, 0, 180, 0, 1.2, true);
        LAST_PARTICLE_SET_COLOR(agent, 3, 1.3, 0.5);
    }
    frame(agent.lua_state_agent, 33.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), false, false);
    }
}

//Bair Sound
unsafe extern "C" fn ssbexo_ganon_bair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_ganon_swing_m"));
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_smash_l02"));
    }
}

//Bair Expression
unsafe extern "C" fn ssbexo_ganon_bair_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

//Bair Landing ACMD
unsafe extern "C" fn ssbexo_ganon_bair_landing_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, -1);
    }
}

//Uair ACMD
unsafe extern "C" fn ssbexo_ganon_uair_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 11.0, 55, 93, 0, 30, 4.0, -1.1, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 12.0, 55, 93, 0, 30, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 13.0, 55, 93, 0, 30, 4.0, 8.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 6.0, 10, 93, 0, 24, 4.0, -1.1, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 7.0, 10, 93, 0, 24, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 8.0, 10, 93, 0, 24, 4.0, 8.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Uair Effect
unsafe extern "C" fn ssbexo_ganon_uair_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_flash"), Hash40::new("armr"), 7, 0, 0, 0, 0, 0, 1.0, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 16, 2.0, 0, 30, 90, 1.1, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 16, 2.0, 0, 110, 90, 1.1, true);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_engokua_flash"), false, false);
    }
}

//Uair Sound
unsafe extern "C" fn ssbexo_ganon_uair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_ganon_swing_l"));
    }
}

//Uair Expression
unsafe extern "C" fn ssbexo_ganon_uair_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

//Dair Effect
unsafe extern "C" fn ssbexo_ganon_dair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 2, 0, -90, 0, 0, 1.8, true);
        LAST_PARTICLE_SET_COLOR(agent, 0.8, 0.6, 3);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("ganon_attack_impact"), Hash40::new("top"), 0, 7, 0, 90, 0, 0, 1.5, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, -3, 0, 0, 0, 0, 2.5, true, 2);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attackairn", ssbexo_ganon_nair_acmd, Low)
    .effect_acmd("effect_attackairn", ssbexo_ganon_nair_effect, Low)
    .sound_acmd("sound_attackairn", ssbexo_ganon_nair_sound, Low)
    .expression_acmd("expression_attackairn", ssbexo_ganon_nair_expression, Low)
    .game_acmd("game_attackairf", ssbexo_ganon_fair_acmd, Low)
    .effect_acmd("effect_attackairf", ssbexo_ganon_fair_effect, Low)
    .sound_acmd("sound_attackairf", ssbexo_ganon_fair_sound, Low)
    .expression_acmd("expression_attackairf", ssbexo_ganon_fair_expression, Low)
    .game_acmd("game_attackairb", ssbexo_ganon_bair_acmd, Low)
    .effect_acmd("effect_attackairb", ssbexo_ganon_bair_effect, Low)
    .sound_acmd("sound_attackairb", ssbexo_ganon_bair_sound, Low)
    .expression_acmd("expression_attackairb", ssbexo_ganon_bair_expression, Low)
    .game_acmd("game_landingairb", ssbexo_ganon_bair_landing_acmd, Low)
    .game_acmd("game_attackairhi", ssbexo_ganon_uair_acmd, Low)
    .effect_acmd("effect_attackairhi", ssbexo_ganon_uair_effect, Low)
    .sound_acmd("sound_attackairhi", ssbexo_ganon_uair_sound, Low)
    .expression_acmd("expression_attackairhi", ssbexo_ganon_uair_expression, Low)
    .effect_acmd("effect_attackairlw", ssbexo_ganon_dair_effect, Low)
    .install()
    ;
}