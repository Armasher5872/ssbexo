use super::*;

//Forward Smash ACMD
#[acmd_script( agent = "kirby", scripts = ["game_attacks4", "game_attacks4hi", "game_attacks4lw"], category = ACMD_GAME)]
unsafe fn ssbuexo_kirby_forward_smash_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, 0);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("attack_s4"), false, 0.0);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 48, 78, 0, 60, 5.4, 0.0, 4.5, 11.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 48, 78, 0, 60, 3.5, 0.0, 4.5, 5.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 54.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Forward Smash Effect
#[acmd_script( agent = "kirby", scripts = ["effect_attacks4", "effect_attacks4hi", "effect_attacks4lw"], category = ACMD_EFFECT)]
unsafe fn ssbuexo_kirby_forward_smash_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("haver"), -0.012, 11.999, 0.137, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    macros::COL_NORMAL(fighter);
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::FLASH(fighter, 1, 1, 1, 0.7);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("kirby_onigoroshi_wind"), Hash40::new("kirby_onigoroshi_wind"), Hash40::new("top"), 1, 6, 3, 13, -20, 0, 1, false, *EF_FLIP_YZ);
    }
}

//Forward Smash Sound
#[acmd_script( agent = "kirby", scripts = ["sound_attacks4", "sound_attacks4hi", "sound_attacks4lw"], category = ACMD_SOUND)]
unsafe fn ssbuexo_kirby_forward_smash_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_kirby_special_s02"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_kirby_002"));
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_kirby_special_s01"));
    }
    frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_kirby_special_s07"));
    }
}

//Forward Smash Expression
#[acmd_script( agent = "kirby", scripts = ["expression_attacks4", "expression_attacks4hi", "expression_attacks4lw"], category = ACMD_EXPRESSION)]
unsafe fn ssbuexo_kirby_forward_smash_expression(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 10);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackll"), 0);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
}

//Up Smash Charge Effect
#[acmd_script( agent = "kirby", script = "effect_attackhi4charge", category = ACMD_EFFECT)]
unsafe fn ssbuexo_kirby_up_smash_charge_effect(fighter: &mut L2CAgentBase) {
    for _ in 0..59 {
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -7, 0, 0, 0, 0, 0, 0.6, 10, 0, 6, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 1, -8, 0, 0, 0, 1, 2, 3, 3, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent, 6.0);
    }
}

//Up Smash ACMD
#[acmd_script( agent = "kirby", script = "game_attackhi4", category = ACMD_GAME)]
unsafe fn ssbuexo_kirby_up_smash_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, false, -1);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 16.0, 75, 80, 0, 60, 5.8, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HAMMER);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 16.0, 75, 80, 0, 60, 3.9, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HAMMER);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("handl"), 16.0, 75, 80, 0, 60, 5.8, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HAMMER);
        macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 16.0, 75, 80, 0, 60, 3.9, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HAMMER);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 56.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Up Smash Effect
#[acmd_script( agent = "kirby", script = "effect_attackhi4", category = ACMD_EFFECT)]
unsafe fn ssbuexo_kirby_up_smash_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), -6, 6, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::BURN_COLOR(fighter, 2, 0.15, 0.02, 0);
        macros::BURN_COLOR_FRAME(fighter, 2, 2, 0.15, 0.02, 0.7);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("kirby_onigoroshi_max"), Hash40::new("haver"), 0, 8, 0, 0, 0, 0, 1, false);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("kirby_onigoroshi_arc"), Hash40::new("kirby_onigoroshi_arc"), Hash40::new("top"), 1, 10, 3, 14, 55, -120, 1, false, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::BURN_COLOR(fighter, 2, 0.15, 0.02, 0.7);
        macros::BURN_COLOR_FRAME(fighter, 8, 2, 0.15, 0.02, 0);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("kirby_onigoroshi"), false, false);
        macros::BURN_COLOR_NORMAL(fighter);
    }
}

//Up Smash Sound
#[acmd_script( agent = "kirby", script = "sound_attackhi4", category = ACMD_SOUND)]
unsafe fn ssbuexo_kirby_up_smash_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_kirby_hammermax"));
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_kirby_special_s03"));
    }
}

//Up Smash Expression
#[acmd_script( agent = "kirby", script = "expression_attackhi4", category = ACMD_EXPRESSION)]
unsafe fn ssbuexo_kirby_up_smash_expression(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 10);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackll"), 0);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
}

//Up Smash Hammer Effect
#[acmd_script( agent = "kirby_hammer", script = "effect_attackhi4", category = ACMD_EFFECT)]
unsafe fn ssbuexo_kirby_up_smash_hammer_effect(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("kirby_onigoroshi_light"), Hash40::new("have"), 0, 0, 0, 0, 0, 0, 1, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("kirby_onigoroshi_body"), Hash40::new("have"), 0, 10, 0, 0, 0, 0, 1, false);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("kirby_onigoroshi_smoke"), Hash40::new("have"), 0, 10, 0, 0, 0, 0, 1, false);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("kirby_onigoroshi_body"), false, false);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("kirby_onigoroshi_light"), false, true);
    }
}

//Down Smash ACMD
#[acmd_script( agent = "kirby", script = "game_attacklw4", category = ACMD_GAME)]
unsafe fn ssbuexo_kirby_down_smash_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, 0);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("attack_lw4"), false, 0.0);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 3.0, 180, 60, 60, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(10.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 15.0, true);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 3.0, 180, 60, 60, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(10.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 15.0, true);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 8.0, 30, 90, 0, 70, 6.0, 0.0, 0.0, 0.0, Some(0.0), Some(10.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 15.0, true);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Down Smash Effect
#[acmd_script( agent = "kirby", script = "effect_attacklw4", category = ACMD_EFFECT)]
unsafe fn ssbuexo_kirby_down_smash_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_kirby_ultrasword1"), Hash40::new("tex_kirby_ultrasword2"), 5, Hash40::new("haver"), 0.0, 0.0, 0.0, Hash40::new("haver"), 0.0, 18.0, 0.0, true, Hash40::new(""), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, 1, 0, 0, 0, 0, 0.7, 1, 1, 1, 0, 90, 0, true, *EF_FLIP_YZ);
        sv_animcmd::EFFECT_FOLLOW_FLIP_RND(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, -8, -1.5, 0, 0, 0, 0.4, 1, 1, 1, 0, 90, 0, true, *EF_FLIP_YZ);
        sv_animcmd::EFFECT_FOLLOW_FLIP_RND(fighter.lua_state_agent);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, -6, -1.5, 0, 0, 0, 0.5, 1, 1, 1, 0, 90, 0, true, *EF_FLIP_YZ);
        sv_animcmd::EFFECT_FOLLOW_FLIP_RND(fighter.lua_state_agent);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, -1, 0, 0, 0, 0, 0.6, 1, 1, 1, 0, 90, 0, true, *EF_FLIP_YZ);
        sv_animcmd::EFFECT_FOLLOW_FLIP_RND(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, -9, -2, 0, 0, 0, 0.35, 1, 1, 1, 0, 90, 0, true, *EF_FLIP_YZ);
        sv_animcmd::EFFECT_FOLLOW_FLIP_RND(fighter.lua_state_agent);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 4);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_kirby_forward_smash_acmd,
        ssbuexo_kirby_forward_smash_effect,
        ssbuexo_kirby_forward_smash_sound,
        ssbuexo_kirby_forward_smash_expression,
        ssbuexo_kirby_up_smash_charge_effect,
        ssbuexo_kirby_up_smash_acmd,
        ssbuexo_kirby_up_smash_effect,
        ssbuexo_kirby_up_smash_sound,
        ssbuexo_kirby_up_smash_expression,
        ssbuexo_kirby_up_smash_hammer_effect,
        ssbuexo_kirby_down_smash_acmd,
        ssbuexo_kirby_down_smash_effect
    );
}