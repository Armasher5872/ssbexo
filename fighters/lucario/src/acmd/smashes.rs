use super::*;

//Forward Smash Charge ACMD
unsafe extern "C" fn ssbexo_lucario_forward_smash_charge_acmd(agent: &mut L2CAgentBase) {
    if !ArticleModule::is_exist(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE) {
        if is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, false, -1);
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, Hash40::new("attack_s4_hold"), false, -1.0);
        }
    }
    else {
        if is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, Hash40::new("attack_s4_hold"), false, -1.0);
        }
    }
}

//Forward Smash Charge Effect
unsafe extern "C" fn ssbexo_lucario_forward_smash_charge_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, 5, 5, 5, 0, 0, 0, true);
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, false);
            EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, -6, 0, 0, 0, 0, 0.25, true);
            EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
            EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 6, 0, 0, 0, 0, 0.25, true);
        }
        wait(agent.lua_state_agent, 5.0);
    }
}

//Forward Smash ACMD
unsafe extern "C" fn ssbexo_lucario_forward_smash_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, Hash40::new("attack_s4"), false, -1.0);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 13.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, Hash40::new("attack_s4"), false, -1.0);
            ArticleModule::set_frame(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, 13.0);
        }
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 15.0, 361, 100, 0, 40, 5.0, 0.0, -8.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Smash Effect
unsafe extern "C" fn ssbexo_lucario_forward_smash_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 8, 10, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, -6, 0, 0, 0, 0, 0.25, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 6, 0, 0, 0, 0, 0.25, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, -6, 0, 0, 0, 0, 0.25, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 6, 0, 0, 0, 0, 0.25, true);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
}

//Forward Smash Sound
unsafe extern "C" fn ssbexo_lucario_forward_smash_sound(agent: &mut L2CAgentBase) {
    let current_aurapower = WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
    let middle_aurapower = WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_MIDDLE_AURAPOWER);
    let high_aurapower = WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_HIGH_AURAPOWER);
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start"));
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_lucario_attack06"));
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_lucario_smash_s01"));
    }
    if current_aurapower == high_aurapower || current_aurapower == middle_aurapower {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new_raw(0x1fffe030f8));
        }
    }
    else {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_lucario_smash_add01"));
        }
    }
}

//Forward Smash Expression
unsafe extern "C" fn ssbexo_lucario_forward_smash_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 12.0);
    sv_animcmd::execute(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl_l"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 45.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

//Up Smash Charge ACMD
unsafe extern "C" fn ssbexo_lucario_up_smash_charge_acmd(agent: &mut L2CAgentBase) {
    if !ArticleModule::is_exist(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE) {
        if is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, false, -1);
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, Hash40::new("attack_hi4_hold"), false, -1.0);
        }
    }
    else {
        if is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, Hash40::new("attack_hi4_hold"), false, -1.0);
        }
    }
}

//Up Smash Charge Effect
unsafe extern "C" fn ssbexo_lucario_up_smash_charge_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 5, 5, 5, 0, 0, 0, true);
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, false);
            EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("havel"), 0, -6, 0, 0, 0, 0, 0.25, true);
            EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.25, true);
            EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("havel"), 0, 6, 0, 0, 0, 0, 0.25, true);
        }
        wait(agent.lua_state_agent, 5.0);
    }
}

//Up Smash ACMD
unsafe extern "C" fn ssbexo_lucario_up_smash_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, Hash40::new("attack_hi4"), false, -1.0);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 6.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, Hash40::new("attack_hi4"), false, -1.0);
            ArticleModule::set_frame(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, 6.0);
        }
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
        }
        ATTACK(agent, 0, 0, Hash40::new("haver"), 2.0, 95, 100, 60, 0, 5.0, 0.0, -8.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 3.0, 367, 45, 20, 0, 5.0, 0.0, -8.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 3.0, 367, 45, 20, 0, 5.0, 0.0, -8.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 3.0, 367, 45, 20, 0, 5.0, 0.0, -8.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 3.0, 367, 45, 20, 0, 5.0, 0.0, -8.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 8.0, false);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 32.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("havel"), 4.0, 90, 160, 0, 55, 5.0, 0.0, -8.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 90, 160, 0, 55, 8.8, 0.0, 30.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 36.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
    }
}

//Up Smash Effect
unsafe extern "C" fn ssbexo_lucario_up_smash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 17, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, -6, 0, 0, 0, 0, 0.25, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 6, 0, 0, 0, 0, 0.25, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, -6, 0, 0, 0, 0, 0.25, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 6, 0, 0, 0, 0, 0.25, true);
    }
    frame(agent.lua_state_agent, 32.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("lucario_hattack_bomb"), Hash40::new("top"), 0, 21.5, 1, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, -6, 0, 0, 0, 0, 0.25, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 6, 0, 0, 0, 0, 0.25, true);
    }
}

//Up Smash Sound
unsafe extern "C" fn ssbexo_lucario_up_smash_sound(agent: &mut L2CAgentBase) {
    let current_aurapower = WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
    let middle_aurapower = WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_MIDDLE_AURAPOWER);
    let high_aurapower = WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_HIGH_AURAPOWER);
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start"));
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_lucario_attack06"));
    }
    frame(agent.lua_state_agent, 29.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_lucario_smash_s01"));
    }
    if current_aurapower == high_aurapower || current_aurapower == middle_aurapower {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_lucario_smash_add02"));
        }
    }
    else {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_lucario_smash_add01"));
        }
    }
}

//Up Smash Expression
unsafe extern "C" fn ssbexo_lucario_up_smash_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 5.0);
    sv_animcmd::execute(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(agent.lua_state_agent, 32.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

//Down Smash ACMD
unsafe extern "C" fn ssbexo_lucario_down_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 6.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
        }
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 33, 99, 0, 30, 4.3, 0.0, 6.0, 13.0, Some(0.0), Some(6.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 33, 99, 0, 30, 4.3, 0.0, 6.0, -15.0, Some(0.0), Some(6.0), Some(-10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("lucario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attacks4charge", ssbexo_lucario_forward_smash_charge_acmd, Low)
    .effect_acmd("effect_attacks4charge", ssbexo_lucario_forward_smash_charge_effect, Low)
    .game_acmd("game_attacks4", ssbexo_lucario_forward_smash_acmd, Low)
    .effect_acmd("effect_attacks4", ssbexo_lucario_forward_smash_effect, Low)
    .sound_acmd("sound_attacks4", ssbexo_lucario_forward_smash_sound, Low)
    .expression_acmd("expression_attacks4", ssbexo_lucario_forward_smash_expression, Low)
    .game_acmd("game_attackhi4charge", ssbexo_lucario_up_smash_charge_acmd, Low)
    .effect_acmd("effect_attackhi4charge", ssbexo_lucario_up_smash_charge_effect, Low)
    .game_acmd("game_attackhi4", ssbexo_lucario_up_smash_acmd, Low)
    .effect_acmd("effect_attackhi4", ssbexo_lucario_up_smash_effect, Low)
    .sound_acmd("sound_attackhi4", ssbexo_lucario_up_smash_sound, Low)
    .expression_acmd("expression_attackhi4", ssbexo_lucario_up_smash_expression, Low)
    .game_acmd("game_attacklw4", ssbexo_lucario_down_smash_acmd, Low)
    .install()
    ;
}