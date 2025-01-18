use super::*;

//Side Special ACMD
unsafe extern "C" fn ssbexo_sheik_side_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SHEIK_STATUS_SPECIAL_LW_FLAG_ACCEL);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

//Grounded Side Special Effect
unsafe extern "C" fn ssbexo_sheik_grounded_side_special_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 7, 12, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sheik_chougyo_start"), Hash40::new("top"), -2.5, -6, -4, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sheik_chougyo"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sheik_chougyo"), false, false);
    }
}

//Aerial Side Special Effect
unsafe extern "C" fn ssbexo_sheik_aerial_side_special_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sheik_chougyo_start"), Hash40::new("top"), -2.5, 0, -4, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 6, 10, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sheik_chougyo"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sheik_chougyo"), false, false);
    }
}

//Side Special Sound
unsafe extern "C" fn ssbexo_sheik_side_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_sheik_rnd_special_l"));
        macros::PLAY_SE(agent, Hash40::new("se_sheik_special_l01"));
    }
}

//Grounded Side Special Expression
unsafe extern "C" fn ssbexo_sheik_grounded_side_special_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 2);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_jump"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Aerial Side Special Expression
unsafe extern "C" fn ssbexo_sheik_aerial_side_special_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_jump"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Down Special ACMD
unsafe extern "C" fn ssbexo_sheik_down_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HAS_VANISHED);
        WorkModule::set_int(agent.module_accessor, 180, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_VANISH_TIMER);
    }
}

//Down Special Effect
unsafe extern "C" fn ssbexo_sheik_down_special_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sheik_sakuretu_wind"), Hash40::new("top"), 0.0, 8.0, 0.0, 0, 0, 0, 1.0, false);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0.0, 8.0, 0.0, 0, 0, 0, 2.0, false);
    }
}

//Down Special Sound
unsafe extern "C" fn ssbexo_sheik_down_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_sheik_rnd_special_h"));
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_sheik_special_s03"));
    }
}

//Down Special Expression
unsafe extern "C" fn ssbexo_sheik_down_special_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosion"), 20, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Grounded Down Special Vanish Cancel Effect
unsafe extern "C" fn ssbexo_sheik_grounded_down_special_vanish_cancel_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0.0, 8.0, 0.0, 0, 0, 0, 2.0, false);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Down Special Vanish Cancel Effect
unsafe extern "C" fn ssbexo_sheik_aerial_down_special_vanish_cancel_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0.0, 8.0, 0.0, 0, 0, 0, 2.0, false);
    }
}

//Down Special Vanish Cancel Sound
unsafe extern "C" fn ssbexo_sheik_down_special_vanish_cancel_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_LANDING_SE(agent, Hash40::new("se_sheik_landing01"));
    }
}

//Down Special Vanish Cancel Expression
unsafe extern "C" fn ssbexo_sheik_down_special_vanish_cancel_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

//Down Special Vanish Attack ACMD
unsafe extern "C" fn ssbexo_sheik_down_special_vanish_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 0, 60, 60, 0, 8.5, 0.0, 16.0, 14.5, Some(0.0), Some(9.0), Some(14.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
            if WorkModule::is_flag(agent.module_accessor, *FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BACK_HIT) {
                macros::ATTACK(agent, 0, 1, Hash40::new("top"), 25.0, 361, 60, 0, 55, 8.5, 0.0, 16.0, 14.5, Some(0.0), Some(9.0), Some(14.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                AttackModule::set_attack_level(agent.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
            }
            else {
                macros::ATTACK(agent, 0, 1, Hash40::new("top"), 10.0, 361, 60, 0, 55, 8.5, 0.0, 16.0, 14.5, Some(0.0), Some(9.0), Some(14.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        }
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
            if WorkModule::is_flag(agent.module_accessor, *FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BACK_HIT) {
                macros::ATTACK(agent, 0, 1, Hash40::new("top"), 25.0, 361, 60, 0, 55, 8.5, 0.0, 16.0, 14.5, Some(0.0), Some(9.0), Some(14.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                AttackModule::set_attack_level(agent.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
            }
            else {
                macros::ATTACK(agent, 0, 1, Hash40::new("top"), 10.0, 361, 60, 0, 55, 8.5, 0.0, 16.0, 14.5, Some(0.0), Some(9.0), Some(14.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        }
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BACK_HIT);
    }
}

//Down Special Vanish Attack Effect
unsafe extern "C" fn ssbexo_sheik_down_special_vanish_attack_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_curse"), Hash40::new("top"), 0.0, 8.0, 0.0, 0, 0, 0, 1.5, false);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.6);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0.0, 8.0, 0.0, 0, 0, 0, 2.0, false);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_curse"), false, false);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_sheik_knife1"), Hash40::new("tex_sheik_knife2"), 3, Hash40::new("haver"), 0.0, 0.5, 0.0, Hash40::new("haver"), 0.0, 8.5, 0.4, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sheik_final_hit"), Hash40::new("top"), 0, 9, 7, -80, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.6);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 1);
    }
}

//Down Special Vanish Attack Sound
unsafe extern "C" fn ssbexo_sheik_down_special_vanish_attack_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_sheik_final11"));
    }
}

//Down Special Vanish Attack Expression
unsafe extern "C" fn ssbexo_sheik_down_special_vanish_attack_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SHEIK_GENERATE_ARTICLE_KNIFE, false, -1);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_SHEIK_GENERATE_ARTICLE_KNIFE, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slash_critical"), 0);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_SHEIK_GENERATE_ARTICLE_KNIFE, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

pub fn install() {
    Agent::new("sheik")
    .game_acmd("game_specials", ssbexo_sheik_side_special_acmd, Priority::Low)
    .effect_acmd("effect_specials", ssbexo_sheik_grounded_side_special_effect, Priority::Low)
    .sound_acmd("sound_specials", ssbexo_sheik_side_special_sound, Priority::Low)
    .expression_acmd("expression_specials", ssbexo_sheik_grounded_side_special_expression, Priority::Low)
    .game_acmd("game_specialairs", ssbexo_sheik_side_special_acmd, Priority::Low)
    .effect_acmd("effect_specialairs", ssbexo_sheik_aerial_side_special_effect, Priority::Low)
    .sound_acmd("sound_specialairs", ssbexo_sheik_side_special_sound, Priority::Low)
    .expression_acmd("expression_specialairs", ssbexo_sheik_aerial_side_special_expression, Priority::Low)
    .game_acmd("game_speciallw", ssbexo_sheik_down_special_acmd, Priority::Low)
    .game_acmd("game_specialairlw", ssbexo_sheik_down_special_acmd, Priority::Low)
    .effect_acmd("effect_speciallw", ssbexo_sheik_down_special_effect, Priority::Low)
    .effect_acmd("effect_specialairlw", ssbexo_sheik_down_special_effect, Priority::Low)
    .sound_acmd("sound_speciallw", ssbexo_sheik_down_special_sound, Priority::Low)
    .sound_acmd("sound_specialairlw", ssbexo_sheik_down_special_sound, Priority::Low)
    .expression_acmd("expression_speciallw", ssbexo_sheik_down_special_expression, Priority::Low)
    .expression_acmd("expression_specialairlw", ssbexo_sheik_down_special_expression, Priority::Low)
    .effect_acmd("effect_speciallwvanishcancel", ssbexo_sheik_grounded_down_special_vanish_cancel_effect, Priority::Low)
    .effect_acmd("effect_specialairlwvanishcancel", ssbexo_sheik_aerial_down_special_vanish_cancel_effect, Priority::Low)
    .sound_acmd("sound_speciallwvanishcancel", ssbexo_sheik_down_special_vanish_cancel_sound, Priority::Low)
    .sound_acmd("sound_specialairlwvanishcancel", ssbexo_sheik_down_special_vanish_cancel_sound, Priority::Low)
    .expression_acmd("expression_speciallwvanishcancel", ssbexo_sheik_down_special_vanish_cancel_expression, Priority::Low)
    .expression_acmd("expression_specialairlwvanishcancel", ssbexo_sheik_down_special_vanish_cancel_expression, Priority::Low)
    .game_acmd("game_speciallwvanishattack", ssbexo_sheik_down_special_vanish_attack_acmd, Priority::Low)
    .game_acmd("game_specialairlwvanishattack", ssbexo_sheik_down_special_vanish_attack_acmd, Priority::Low)
    .effect_acmd("effect_speciallwvanishattack", ssbexo_sheik_down_special_vanish_attack_effect, Priority::Low)
    .effect_acmd("effect_specialairlwvanishattack", ssbexo_sheik_down_special_vanish_attack_effect, Priority::Low)
    .sound_acmd("sound_speciallwvanishattack", ssbexo_sheik_down_special_vanish_attack_sound, Priority::Low)
    .sound_acmd("sound_specialairlwvanishattack", ssbexo_sheik_down_special_vanish_attack_sound, Priority::Low)
    .expression_acmd("expression_speciallwvanishattack", ssbexo_sheik_down_special_vanish_attack_expression, Priority::Low)
    .expression_acmd("expression_specialairlwvanishattack", ssbexo_sheik_down_special_vanish_attack_expression, Priority::Low)
    .install()
    ;
}