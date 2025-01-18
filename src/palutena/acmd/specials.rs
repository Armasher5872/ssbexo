use super::*;

//Neutral Special ACMD
unsafe extern "C" fn ssbexo_palutena_neutral_special_acmd(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 1.2);
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_EXPLOSIVEFLAME, false, -1);
    }
}

//Grounded Neutral Special Effect
unsafe extern "C" fn ssbexo_palutena_grounded_neutral_special_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), -1, 21, 1, 0, 90, 0, 1, true, 0.7);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
    }
}

//Aerial Neutral Special Effect
unsafe extern "C" fn ssbexo_palutena_aerial_neutral_special_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), -1, 21, 1, 0, 90, 0, 1, true, 0.7);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light_trace"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
    }
}

//Neutral Special Sound
unsafe extern "C" fn ssbexo_palutena_neutral_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_palutena_special_s01"));
    }
    wait(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_palutena_rnd_special_n"));
    }
}

//Neutral Special Expression
unsafe extern "C" fn ssbexo_palutena_neutral_special_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
}

//Side Special ACMD
unsafe extern "C" fn ssbexo_palutena_side_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_REFLECTIONBOARD, false, -1);
    }
}

//Grounded Side Special Effect
unsafe extern "C" fn ssbexo_palutena_grounded_side_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("palutena_backlight"), 0);
    }
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT_FLW_POS(agent, Hash40::new("palutena_backlight"), Hash40::new("waist"), 10, -3, -1, 0, 90, 0, 1, true);
            EffectModule::set_disable_render_offset_last(agent.module_accessor);
            macros::LAST_EFFECT_SET_RATE(agent, 2.75);
        }
        else {
            if macros::is_excute(agent) {
                macros::EFFECT_FLW_POS(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 0, 21, 5, 0, -90, 0, 1, true);
                EffectModule::set_disable_render_offset_last(agent.module_accessor);
                macros::LAST_EFFECT_SET_RATE(agent, 2.75);
            }
        }
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 1, 1, 1, 0);
        macros::EFFECT_FLW_POS(agent, Hash40::new("palutena_counter_attack"), Hash40::new("stick"), 0, 8.5, 0, 0, 0, 0, 1.2, true);
        EffectModule::set_disable_render_offset_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("palutena_backlight"), -1);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
}

//Aerial Side Special Effect
unsafe extern "C" fn ssbexo_palutena_aerial_side_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("palutena_backlight"), 0);
    }
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT_FLW_POS(agent, Hash40::new("palutena_backlight"), Hash40::new("waist"), 10, -3, -1, 0, 90, 0, 1, true);
            EffectModule::set_disable_render_offset_last(agent.module_accessor);
            macros::LAST_EFFECT_SET_RATE(agent, 2.75);
        }
        else {
            if macros::is_excute(agent) {
                macros::EFFECT_FLW_POS(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 0, 21, 5, 0, -90, 0, 1, true);
                EffectModule::set_disable_render_offset_last(agent.module_accessor);
                macros::LAST_EFFECT_SET_RATE(agent, 2.75);
            }
        }
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 1, 1, 1, 0);
        macros::EFFECT_FLW_POS(agent, Hash40::new("palutena_counter_attack"), Hash40::new("stick"), 0, 8.5, 0, 0, 0, 0, 1.2, true);
        EffectModule::set_disable_render_offset_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("palutena_backlight"), -1);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
}

//Side Special Sound
unsafe extern "C" fn ssbexo_palutena_side_special_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_palutena_rnd_special_s"));
    }
}

//Side Special Expression
unsafe extern "C" fn ssbexo_palutena_side_special_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

//Reflection Board Shoot ACMD
unsafe extern "C" fn ssbexo_palutena_reflection_board_shoot_acmd(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 20, 100, 10, 0, 5.0, 0.0, 8.5, 0.0, Some(0.0), Some(-4.5), Some(0.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

//Reflection Board Break Effect
unsafe extern "C" fn ssbexo_palutena_reflection_board_break_effect(agent: &mut L2CAgentBase) {
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("palutena_mirror_break"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_RATE(agent, 0.8);
        }
        else {
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("palutena_mirror_break"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
                macros::LAST_EFFECT_SET_RATE(agent, 0.8);
            }
        }
    }
}

//Up Special Start ACMD
unsafe extern "C" fn ssbexo_palutena_up_special_start_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::SA_SET(agent, *SITUATION_KIND_AIR);
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    }
}

//Grounded Up Special Start Effect
unsafe extern "C" fn ssbexo_palutena_grounded_up_special_start_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_appeal_twinkle"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 0.7, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_appeal_twinkle"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 0.7, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 0, 21.5, -1, 0, -90, 0, 1, true, 0.7);
    }
}

//Aerial Up Special Start Effect
unsafe extern "C" fn ssbexo_palutena_aerial_up_special_start_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_appeal_twinkle"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 0.7, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_appeal_twinkle"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 0.7, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 0, 21.5, -1, 0, -90, 0, 1, true, 0.7);
    }
}

//Up Special Start Sound
unsafe extern "C" fn ssbexo_palutena_up_special_start_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_palutena_special_h01"));
        macros::PLAY_SE(agent, Hash40::new("vc_palutena_special_h01"));
    }
}

//Up Special Start Expression
unsafe extern "C" fn ssbexo_palutena_up_special_start_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_attach_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Up Special Glide ACMD
unsafe extern "C" fn ssbexo_palutena_up_special_glide_acmd(_agent: &mut L2CAgentBase) {}

//Up Special Glide Effect
unsafe extern "C" fn ssbexo_palutena_up_special_glide_effect(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("palutena_appeal_twinkle"), false, false);
            macros::EFFECT_OFF_KIND(agent, Hash40::new("palutena_backlight"), false, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_appeal_twinkle"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 0.7, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_appeal_twinkle"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 0.7, true);
            macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 0, 21.5, -1, 0, -90, 0, 1, true, 0.7);
        }
        wait(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("palutena_appeal_twinkle"), false, false);
            macros::EFFECT_OFF_KIND(agent, Hash40::new("palutena_backlight"), false, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_appeal_twinkle"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 0.7, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_appeal_twinkle"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 0.7, true);
            macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 0, 21.5, -1, 0, -90, 0, 1, true, 0.7);
        }
    }
}

//Up Special Glide Sound
unsafe extern "C" fn ssbexo_palutena_up_special_glide_sound(_agent: &mut L2CAgentBase) {}

//Up Special Glide Expression
unsafe extern "C" fn ssbexo_palutena_up_special_glide_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_attach_item_visibility(agent.module_accessor, false, 0);
    }
    wait(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Up Special Turn ACMD
unsafe extern "C" fn ssbexo_palutena_up_special_turn_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        PostureModule::reverse_lr(agent.module_accessor);
    }
}

//Up Special Turn Effect
unsafe extern "C" fn ssbexo_palutena_up_special_turn_effect(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("palutena_appeal_twinkle"), false, false);
            macros::EFFECT_OFF_KIND(agent, Hash40::new("palutena_backlight"), false, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_appeal_twinkle"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 0.7, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_appeal_twinkle"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 0.7, true);
            macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 0, 21.5, -1, 0, -90, 0, 1, true, 0.7);
        }
        wait(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("palutena_appeal_twinkle"), false, false);
            macros::EFFECT_OFF_KIND(agent, Hash40::new("palutena_backlight"), false, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_appeal_twinkle"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 0.7, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_appeal_twinkle"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 0.7, true);
            macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 0, 21.5, -1, 0, -90, 0, 1, true, 0.7);
        }
    }
}

//Up Special Turn Sound
unsafe extern "C" fn ssbexo_palutena_up_special_turn_sound(_agent: &mut L2CAgentBase) {}

//Up Special Turn Expression
unsafe extern "C" fn ssbexo_palutena_up_special_turn_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_attach_item_visibility(agent.module_accessor, false, 0);
    }
    wait(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Down Special ACMD
unsafe extern "C" fn ssbexo_palutena_down_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 40.0);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) && !WorkModule::is_flag(agent.module_accessor, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT);
            WorkModule::set_int(agent.module_accessor, 735, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_TIMER);
        }
    }
}

//Down Special Effect
unsafe extern "C" fn ssbexo_palutena_down_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_appeal_twinkle"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.7, true);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("palutena_appeal_twinkle"), false, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_appeal_twinkle"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.7, true);
    }
    frame(agent.lua_state_agent, 40.0);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) && !WorkModule::is_flag(agent.module_accessor, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), -5, 23, 0, 0, 90, 0, 1, true, 0.7);
        }
    }
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("palutena_appeal_twinkle"), false, false);
    }
}

//Down Special Sound
unsafe extern "C" fn ssbexo_palutena_down_special_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_palutena_special_l01"));
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_palutena_appeal_l01"));
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_palutena_appeal_l01"));
    }
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_palutena_appeal_h01"));
        macros::PLAY_SE(agent, Hash40::new("se_palutena_appeal_l01"));
    }
}

//Down Special Expression
unsafe extern "C" fn ssbexo_palutena_down_special_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::AREA_WIND_2ND_arg10(agent, 0, 2, 90, 300, 1, 9, 35, 18, 70, 80);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::AREA_WIND_2ND_arg10(agent, 0, 2, 90, 300, 1, 9, 35, 18, 70, 80);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::AREA_WIND_2ND_arg10(agent, 0, 2, 90, 300, 1, 9, 35, 18, 70, 80);
    }
}

pub fn install() {
    Agent::new("palutena")
    .game_acmd("game_specialn", ssbexo_palutena_neutral_special_acmd, Priority::Low)
    .game_acmd("game_specialairn", ssbexo_palutena_neutral_special_acmd, Priority::Low)
    .effect_acmd("effect_specialn", ssbexo_palutena_grounded_neutral_special_effect, Priority::Low)
    .effect_acmd("effect_specialairn", ssbexo_palutena_aerial_neutral_special_effect, Priority::Low)
    .sound_acmd("sound_specialn", ssbexo_palutena_neutral_special_sound, Priority::Low)
    .sound_acmd("sound_specialairn", ssbexo_palutena_neutral_special_sound, Priority::Low)
    .expression_acmd("expression_specialn", ssbexo_palutena_neutral_special_expression, Priority::Low)
    .expression_acmd("expression_specialairn", ssbexo_palutena_neutral_special_expression, Priority::Low)
    .game_acmd("game_specials", ssbexo_palutena_side_special_acmd, Priority::Low)
    .game_acmd("game_specialairs", ssbexo_palutena_side_special_acmd, Priority::Low)
    .effect_acmd("effect_specials", ssbexo_palutena_grounded_side_special_effect, Priority::Low)
    .effect_acmd("effect_specialairs", ssbexo_palutena_aerial_side_special_effect, Priority::Low)
    .sound_acmd("sound_specials", ssbexo_palutena_side_special_sound, Priority::Low)
    .sound_acmd("sound_specialairs", ssbexo_palutena_side_special_sound, Priority::Low)
    .expression_acmd("expression_specials", ssbexo_palutena_side_special_expression, Priority::Low)
    .expression_acmd("expression_specialairs", ssbexo_palutena_side_special_expression, Priority::Low)
    .game_acmd("game_specialhistart", ssbexo_palutena_up_special_start_acmd, Priority::Low)
    .game_acmd("game_specialairhistart", ssbexo_palutena_up_special_start_acmd, Priority::Low)
    .effect_acmd("effect_specialhistart", ssbexo_palutena_grounded_up_special_start_effect, Priority::Low)
    .effect_acmd("effect_specialairhistart", ssbexo_palutena_aerial_up_special_start_effect, Priority::Low)
    .sound_acmd("sound_specialhistart", ssbexo_palutena_up_special_start_sound, Priority::Low)
    .sound_acmd("sound_specialairhistart", ssbexo_palutena_up_special_start_sound, Priority::Low)
    .expression_acmd("expression_specialhistart", ssbexo_palutena_up_special_start_expression, Priority::Low)
    .expression_acmd("expression_specialairhistart", ssbexo_palutena_up_special_start_expression, Priority::Low)
    .game_acmd("game_specialhi", ssbexo_palutena_up_special_glide_acmd, Priority::Low)
    .game_acmd("game_specialairhi", ssbexo_palutena_up_special_glide_acmd, Priority::Low)
    .effect_acmd("effect_specialhi", ssbexo_palutena_up_special_glide_effect, Priority::Low)
    .effect_acmd("effect_specialairhi", ssbexo_palutena_up_special_glide_effect, Priority::Low)
    .sound_acmd("sound_specialhi", ssbexo_palutena_up_special_glide_sound, Priority::Low)
    .sound_acmd("sound_specialairhi", ssbexo_palutena_up_special_glide_sound, Priority::Low)
    .expression_acmd("expression_specialhi", ssbexo_palutena_up_special_glide_expression, Priority::Low)
    .expression_acmd("expression_specialairhi", ssbexo_palutena_up_special_glide_expression, Priority::Low)
    .game_acmd("game_specialhiturn", ssbexo_palutena_up_special_turn_acmd, Priority::Low)
    .game_acmd("game_specialairhiturn", ssbexo_palutena_up_special_turn_acmd, Priority::Low)
    .effect_acmd("effect_specialhiturn", ssbexo_palutena_up_special_turn_effect, Priority::Low)
    .effect_acmd("effect_specialairhiturn", ssbexo_palutena_up_special_turn_effect, Priority::Low)
    .sound_acmd("sound_specialhiturn", ssbexo_palutena_up_special_turn_sound, Priority::Low)
    .sound_acmd("sound_specialairhiturn", ssbexo_palutena_up_special_turn_sound, Priority::Low)
    .expression_acmd("expression_specialhiturn", ssbexo_palutena_up_special_turn_expression, Priority::Low)
    .expression_acmd("expression_specialairhiturn", ssbexo_palutena_up_special_turn_expression, Priority::Low)
    .game_acmd("game_speciallw", ssbexo_palutena_down_special_acmd, Priority::Low)
    .game_acmd("game_specialairlw", ssbexo_palutena_down_special_acmd, Priority::Low)
    .effect_acmd("effect_speciallw", ssbexo_palutena_down_special_effect, Priority::Low)
    .effect_acmd("effect_specialairlw", ssbexo_palutena_down_special_effect, Priority::Low)
    .sound_acmd("sound_speciallw", ssbexo_palutena_down_special_sound, Priority::Low)
    .sound_acmd("sound_specialairlw", ssbexo_palutena_down_special_sound, Priority::Low)
    .expression_acmd("expression_speciallw", ssbexo_palutena_down_special_expression, Priority::Low)
    .expression_acmd("expression_specialairlw", ssbexo_palutena_down_special_expression, Priority::Low)
    .install()
    ;
    Agent::new("palutena_reflectionboard")
    .game_acmd("game_shoot", ssbexo_palutena_reflection_board_shoot_acmd, Priority::Low)
    .effect_acmd("effect_break", ssbexo_palutena_reflection_board_break_effect, Priority::Low)
    .install()
    ;
}