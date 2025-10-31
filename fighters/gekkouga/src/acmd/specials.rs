use super::*;

//Side Special ACMD
unsafe extern "C" fn ssbexo_gekkouga_side_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_FLAG_RESET_GRAVITY);
    }
}

//Side Special Attack Hi ACMD
unsafe extern "C" fn ssbexo_gekkouga_side_special_attack_hi_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legl"), 13.0, 90, 95, 0, 50, 8.5, 10.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("legl"), 13.0, 90, 95, 0, 50, 7.0, 2.5, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Side Special Attack Hi Effect
unsafe extern "C" fn ssbexo_gekkouga_side_special_attack_hi_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        FLASH(agent, 1, 1, 1, 0);
        EFFECT(agent, Hash40::new("gekkouga_kageuchi_warp_end"), Hash40::new("top"), 0, 7, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("gekkouga_migawari_appearance"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 4.0, 9.0, 0.0, 0, 0, 90, 1.0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(agent, 0.07, 0.05, 1.0);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

//Side Special Attack Hi Sound
unsafe extern "C" fn ssbexo_gekkouga_side_special_attack_hi_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_gekkouga_004"));
        PLAY_SE(agent, Hash40::new("se_gekkouga_special_l03"));
    }
}

//Side Special Attack Hi Expression
unsafe extern "C" fn ssbexo_gekkouga_side_special_attack_hi_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, true);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

//Side Special Attack Lw ACMD
unsafe extern "C" fn ssbexo_gekkouga_side_special_attack_lw_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legl"), 13.0, 270, 65, 0, 25, 8.5, 10.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("legl"), 13.0, 45, 95, 0, 50, 8.5, 10.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Side Special Attack Lw Effect
unsafe extern "C" fn ssbexo_gekkouga_side_special_attack_lw_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        FLASH(agent, 1, 1, 1, 0);
        EFFECT(agent, Hash40::new("gekkouga_kageuchi_warp_end"), Hash40::new("top"), 0, 7, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("gekkouga_migawari_appearance"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("gekkouga_pump_line"), Hash40::new("top"), 0, 10, 17, 0, 90, 0, 1, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

//Side Special Attack Lw Sound
unsafe extern "C" fn ssbexo_gekkouga_side_special_attack_lw_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_gekkouga_004"));
        PLAY_SE(agent, Hash40::new("se_gekkouga_special_l03"));
    }
}

//Side Special Attack Lw Expression
unsafe extern "C" fn ssbexo_gekkouga_side_special_attack_lw_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, true);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

//Grounded Down Special ACMD
unsafe extern "C" fn ssbexo_gekkouga_grounded_down_special_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_MAT, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_MAT, Hash40::new("special_lw"), false, -1.0);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_MAT, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("footl"), 6.0, 270, 65, 0, 25, 5.5, 0.0, -1.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 6.0, 270, 65, 0, 25, 5.5, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("legl"), 6.0, 270, 65, 0, 25, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 35.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_MAT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Mat Grounded Down Special ACMD
unsafe extern "C" fn ssbexo_gekkouga_mat_grounded_down_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        ReflectorModule::set_status_all(agent.module_accessor, ShieldStatus(*SHIELD_STATUS_NORMAL_GLOBAL), 0);
        ATTACK(agent, 0, 0, Hash40::new("tatami1"), 9.0, 80, 75, 0, 25, 3.0, 0.0, 0.0, 0.0, Some(24.0), Some(0.0), Some(0.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Aerial Down Special ACMD
unsafe extern "C" fn ssbexo_gekkouga_aerial_down_special_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_MAT, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_MAT, Hash40::new("special_air_lw_start"), false, -1.0);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_MAT, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("handl"), 6.0, 270, 65, 0, 25, 5.5, 0.0, -1.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 6.0, 270, 65, 0, 25, 5.5, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("shoulderl"), 6.0, 270, 65, 0, 25, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 31.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Mat Fall ACMD
unsafe extern "C" fn ssbexo_gekkouga_mat_fall_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ReflectorModule::set_status_all(agent.module_accessor, ShieldStatus(*SHIELD_STATUS_NORMAL_GLOBAL), 0);
        ATTACK(agent, 0, 0, Hash40::new("tatami1"), 9.0, 80, 45, 0, 25, 3.0, 0.0, 0.0, 0.0, Some(24.0), Some(0.0), Some(0.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
}

//Grounded Down Special Effect
unsafe extern "C" fn ssbexo_gekkouga_grounded_down_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 24, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 26, 15, 0, -90, 90, 1.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 15, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    }
}

//Aerial Down Special Effect
unsafe extern "C" fn ssbexo_gekkouga_aerial_down_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 24, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 27.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 26, 15, 0, -90, 90, 1.6, 0, 0, 0, 0, 0, 0, true);
    }
}

//Grounded Down Special Sound
unsafe extern "C" fn ssbexo_gekkouga_grounded_down_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_gekkouga_final03"));
        PLAY_SE(agent, Hash40::new("se_gekkouga_final02"));
    }
}

//Aerial Down Special Sound
unsafe extern "C" fn ssbexo_gekkouga_aerial_down_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_gekkouga_final03"));
        PLAY_SE(agent, Hash40::new("se_gekkouga_final02"));
    }
}

//Down Special Expression
unsafe extern "C" fn ssbexo_gekkouga_down_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

pub fn install() {
    Agent::new("gekkouga")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specials", ssbexo_gekkouga_side_special_acmd, Low)
    .game_acmd("game_specialsattackhi", ssbexo_gekkouga_side_special_attack_hi_acmd, Low)
    .game_acmd("game_specialairsattackhi", ssbexo_gekkouga_side_special_attack_hi_acmd, Low)
    .effect_acmd("effect_specialsattackhi", ssbexo_gekkouga_side_special_attack_hi_effect, Low)
    .effect_acmd("effect_specialairsattackhi", ssbexo_gekkouga_side_special_attack_hi_effect, Low)
    .sound_acmd("sound_specialsattackhi", ssbexo_gekkouga_side_special_attack_hi_sound, Low)
    .sound_acmd("sound_specialairsattackhi", ssbexo_gekkouga_side_special_attack_hi_sound, Low)
    .expression_acmd("expression_specialsattackhi", ssbexo_gekkouga_side_special_attack_hi_expression, Low)
    .expression_acmd("expression_specialairsattackhi", ssbexo_gekkouga_side_special_attack_hi_expression, Low)
    .game_acmd("game_specialsattacklw", ssbexo_gekkouga_side_special_attack_lw_acmd, Low)
    .game_acmd("game_specialairsattacklw", ssbexo_gekkouga_side_special_attack_lw_acmd, Low)
    .effect_acmd("effect_specialsattacklw", ssbexo_gekkouga_side_special_attack_lw_effect, Low)
    .effect_acmd("effect_specialairsattacklw", ssbexo_gekkouga_side_special_attack_lw_effect, Low)
    .sound_acmd("sound_specialsattacklw", ssbexo_gekkouga_side_special_attack_lw_sound, Low)
    .sound_acmd("sound_specialairsattacklw", ssbexo_gekkouga_side_special_attack_lw_sound, Low)
    .expression_acmd("expression_specialsattacklw", ssbexo_gekkouga_side_special_attack_lw_expression, Low)
    .expression_acmd("expression_specialairsattacklw", ssbexo_gekkouga_side_special_attack_lw_expression, Low)
    .game_acmd("game_speciallw", ssbexo_gekkouga_grounded_down_special_acmd, Low)
    .game_acmd("game_specialairlw", ssbexo_gekkouga_aerial_down_special_acmd, Low)
    .effect_acmd("effect_speciallw", ssbexo_gekkouga_grounded_down_special_effect, Low)
    .effect_acmd("effect_specialairlw", ssbexo_gekkouga_aerial_down_special_effect, Low)
    .sound_acmd("sound_speciallw", ssbexo_gekkouga_grounded_down_special_sound, Low)
    .sound_acmd("sound_specialairlw", ssbexo_gekkouga_aerial_down_special_sound, Low)
    .expression_acmd("expression_speciallw", ssbexo_gekkouga_down_special_expression, Low)
    .expression_acmd("expression_specialairlw", ssbexo_gekkouga_down_special_expression, Low)
    .install()
    ;
    Agent::new("gekkouga_mat")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallw", ssbexo_gekkouga_mat_grounded_down_special_acmd, Low)
    .game_acmd("game_specialairlw", ssbexo_gekkouga_mat_fall_acmd, Low)
    .install()
    ;
}