use super::*;

//Side Special ACMD
unsafe extern "C" fn ssbexo_kirby_side_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 46, 71, 0, 82, 5.0, 0.0, 5.0, 3.0, Some(0.0), Some(5.0), Some(0.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 76, 50, 0, 80, 3.5, 0.0, 5.0, 4.0, Some(0.0), Some(5.0), Some(-0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 32.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 76, 50, 0, 80, 3.5, 0.0, 5.0, 4.0, Some(0.0), Some(5.0), Some(0.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 45.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        JostleModule::set_status(agent.module_accessor, true);
    }
}

//Side Special Effect
unsafe extern "C" fn ssbexo_kirby_side_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("kirby_dash"), Hash40::new("top"), 0, 6, 5, -90, 0, 160, 0.7, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        BURN_COLOR(agent, 2, 0.059, 0.008, 0);
        BURN_COLOR_FRAME(agent, 4, 2, 0.059, 0.008, 0.9);
    }
    frame(agent.lua_state_agent, 34.0);
    if is_excute(agent) {
        BURN_COLOR(agent, 2, 0.059, 0.008, 0.9);
        BURN_COLOR_FRAME(agent, 12, 2, 0.059, 0.008, 0);
        EFFECT_OFF_KIND(agent, Hash40::new("kirby_dash"), false, true);
    }
    frame(agent.lua_state_agent, 46.0);
    if is_excute(agent) {
        BURN_COLOR_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 48.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

//Side Special Sound
unsafe extern "C" fn ssbexo_kirby_side_special_sound(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_kirby_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_kirby_attackdash"));
    }
    frame(agent.lua_state_agent, 48.0);
    if is_excute(agent) {
        PLAY_LANDING_SE(agent, Hash40::new("se_kirby_landing02"));
    }
}

//Side Special Expression
unsafe extern "C" fn ssbexo_kirby_side_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 29.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 35.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 39.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 15);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Up Special 1 ACMD
unsafe extern "C" fn ssbexo_kirby_up_special_1_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi"), false, -1.0);
    }
}

//Up Special 1 Effect
unsafe extern "C" fn ssbexo_kirby_up_special_1_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("kirby_fcut_arc"), Hash40::new("kirby_fcut_arc"), Hash40::new("top"), 0, 12, 0, 0, 0, -149, 1, false, *EF_FLIP_YZ);
    }
}

//Up Special 1 Sound
unsafe extern "C" fn ssbexo_kirby_up_special_1_sound(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_kirby_006"));
        PLAY_SE(agent, Hash40::new("se_kirby_special_h01"));
    }
}

//Up Special 1 Expression
unsafe extern "C" fn ssbexo_kirby_up_special_1_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Up Special 2 ACMD
unsafe extern "C" fn ssbexo_kirby_up_special_2_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi2"), false, -1.0);
    }
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 85, 85, 0, 117, 3.5, 0.0, 3.5, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 85, 85, 0, 117, 3.5, 0.0, 3.5, 18.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 85, 85, 0, 117, 3.5, 0.0, 13.5, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 5.0, 85, 85, 0, 117, 3.5, 0.0, 13.5, 18.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INPUT);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INPUT);
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, ArticleOperationTarget(0));
    }
}

//Up Special 2 Effect
unsafe extern "C" fn ssbexo_kirby_up_special_2_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_kirby_cutter"), Hash40::new("tex_kirby_cutter"), 12, Hash40::new("haver"), 0.0, 3.0, 0.25, Hash40::new("haver"), 0.0, 15.0, 0.5, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        EFFECT_FOLLOW(agent, Hash40::new("kirby_fcut_rise"), Hash40::new("haver"), 0, 3, 0.3, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
}

//Up Special 2 Sound
unsafe extern "C" fn ssbexo_kirby_up_special_2_sound(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_kirby_007"));
        PLAY_SE(agent, Hash40::new("se_kirby_special_h02"));
    }
}

//Up Special 2 Expression
unsafe extern "C" fn ssbexo_kirby_up_special_2_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
}

//Up Special 3 ACMD
unsafe extern "C" fn ssbexo_kirby_up_special_3_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi3"), false, -1.0);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 2.0, 275, 100, 96, 0, 6.0, 0.0, 8.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
}

//Up Special 3 Effect
unsafe extern "C" fn ssbexo_kirby_up_special_3_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_kirby_cutter"), Hash40::new("tex_kirby_cutter"), 12, Hash40::new("haver"), 0.0, 2.5, 0.5, Hash40::new("haver"), 0.0, 12.0, 0.3, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, -4, 16, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//Up Special 3 Sound
unsafe extern "C" fn ssbexo_kirby_up_special_3_sound(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_kirby_special_h04"));
    }
}

//Up Special 3 Expression
unsafe extern "C" fn ssbexo_kirby_up_special_3_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
}

//Up Special 4 ACMD
unsafe extern "C" fn ssbexo_kirby_up_special_4_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi3"), false, -1.0);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 2.0, 275, 100, 96, 0, 6.0, 0.0, 8.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
}

//Up Special 4 Effect
unsafe extern "C" fn ssbexo_kirby_up_special_4_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_kirby_cutter"), Hash40::new("tex_kirby_cutter"), 12, Hash40::new("haver"), 0.0, 2.5, 0.5, Hash40::new("haver"), 0.0, 12.0, 0.3, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
    }
}

//Up Special 4 Sound
unsafe extern "C" fn ssbexo_kirby_up_special_4_sound(_agent: &mut L2CAgentBase) {}

//Up Special 4 Expression
unsafe extern "C" fn ssbexo_kirby_up_special_4_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
}

//Up Special 5 ACMD
unsafe extern "C" fn ssbexo_kirby_up_special_5_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi4"), false, -1.0);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTERSHOT, false, -1);
    }
}

//Up Special 5 Effect
unsafe extern "C" fn ssbexo_kirby_up_special_5_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 0);
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("kirby_star"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Up Special 5 Sound
unsafe extern "C" fn ssbexo_kirby_up_special_5_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_kirby_008"));
        PLAY_SE(agent, Hash40::new("se_kirby_special_h03"));
    }
}

//Up Special 5 Expression
unsafe extern "C" fn ssbexo_kirby_up_special_5_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 7);
    }
}

//Aerial Stone ACMD
unsafe extern "C" fn ssbexo_kirby_aerial_down_special_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, Hash40::new("special_air_lw"), false, -1.0);
        MotionModule::set_rate(agent.module_accessor, 1.21);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_STONE_BLINK_ONOFF);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 25);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, Hash40::new("special_lw_to_air"), false, -1.0);
        AttackModule::clear(agent.module_accessor, 1, false);
        ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 70, 76, 0, 69, 6.5, 0.0, 2.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::init_attack_pos(agent.module_accessor, 0);
    }
}

//Aerial Stone Landing ACMD
unsafe extern "C" fn ssbexo_kirby_down_special_landing_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, Hash40::new("special_lw_to_ground"), false, -1.0);
        ATTACK(agent, 0, 1, Hash40::new("top"), 14.0, 25, 30, 0, 86, 6.0, 0.0, 4.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 1, Hash40::new("top"), 14.0, 25, 30, 0, 86, 6.0, 0.0, 4.0, -6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::init_attack_pos(agent.module_accessor, 0);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("kirby")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specials", ssbexo_kirby_side_special_acmd, Low)
    .game_acmd("game_specialairs", ssbexo_kirby_side_special_acmd, Low)
    .effect_acmd("effect_specials", ssbexo_kirby_side_special_effect, Low)
    .effect_acmd("effect_specialairs", ssbexo_kirby_side_special_effect, Low)
    .sound_acmd("sound_specials", ssbexo_kirby_side_special_sound, Low)
    .sound_acmd("sound_specialairs", ssbexo_kirby_side_special_sound, Low)
    .expression_acmd("expression_specials", ssbexo_kirby_side_special_expression, Low)
    .expression_acmd("expression_specialairs", ssbexo_kirby_side_special_expression, Low)
    .game_acmd("game_specialhi1", ssbexo_kirby_up_special_1_acmd, Low)
    .game_acmd("game_specialairhi1", ssbexo_kirby_up_special_1_acmd, Low)
    .effect_acmd("effect_specialhi1", ssbexo_kirby_up_special_1_effect, Low)
    .effect_acmd("effect_specialairhi1", ssbexo_kirby_up_special_1_effect, Low)
    .sound_acmd("sound_specialhi1", ssbexo_kirby_up_special_1_sound, Low)
    .sound_acmd("sound_specialairhi1", ssbexo_kirby_up_special_1_sound, Low)
    .expression_acmd("expression_specialhi1", ssbexo_kirby_up_special_1_expression, Low)
    .expression_acmd("expression_specialairhi1", ssbexo_kirby_up_special_1_expression, Low)
    .game_acmd("game_specialhi2", ssbexo_kirby_up_special_2_acmd, Low)
    .game_acmd("game_specialairhi2", ssbexo_kirby_up_special_2_acmd, Low)
    .effect_acmd("effect_specialhi2", ssbexo_kirby_up_special_2_effect, Low)
    .effect_acmd("effect_specialairhi2", ssbexo_kirby_up_special_2_effect, Low)
    .sound_acmd("sound_specialhi2", ssbexo_kirby_up_special_2_sound, Low)
    .sound_acmd("sound_specialairhi2", ssbexo_kirby_up_special_2_sound, Low)
    .expression_acmd("expression_specialhi2", ssbexo_kirby_up_special_2_expression, Low)
    .expression_acmd("expression_specialairhi2", ssbexo_kirby_up_special_2_expression, Low)
    .game_acmd("game_specialhi3", ssbexo_kirby_up_special_3_acmd, Low)
    .effect_acmd("effect_specialhi3", ssbexo_kirby_up_special_3_effect, Low)
    .sound_acmd("sound_specialhi3", ssbexo_kirby_up_special_3_sound, Low)
    .expression_acmd("expression_specialhi3", ssbexo_kirby_up_special_3_expression, Low)
    .game_acmd("game_specialhi4", ssbexo_kirby_up_special_4_acmd, Low)
    .effect_acmd("effect_specialhi4", ssbexo_kirby_up_special_4_effect, Low)
    .sound_acmd("sound_specialhi4", ssbexo_kirby_up_special_4_sound, Low)
    .expression_acmd("expression_specialhi4", ssbexo_kirby_up_special_4_expression, Low)
    .game_acmd("game_specialhi5", ssbexo_kirby_up_special_5_acmd, Low)
    .effect_acmd("effect_specialhi5", ssbexo_kirby_up_special_5_effect, Low)
    .sound_acmd("sound_specialhi5", ssbexo_kirby_up_special_5_sound, Low)
    .expression_acmd("expression_specialhi5", ssbexo_kirby_up_special_5_expression, Low)
    .game_acmd("game_specialairlw", ssbexo_kirby_aerial_down_special_acmd, Low)
    .game_acmd("game_speciallwtoground", ssbexo_kirby_down_special_landing_acmd, Low)
    .install()
    ;
}