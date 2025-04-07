use super::*;

//High Jump Kick Start ACMD
unsafe extern "C" fn ssbexo_lucario_high_jump_kick_start_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::SET_SPEED_EX(agent, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
}

//High Jump Kick Start Effect
unsafe extern "C" fn ssbexo_lucario_high_jump_kick_start_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::COL_PRI(agent, 200);
        macros::FLASH(agent, 0, 0, 0, 0);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucario_kagebunshin"), true, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("lucario_kagebunshin_split"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("lucario_kagebunshin_split"), -1);
        macros::COL_NORMAL(agent);
    }
}

//High Jump Kick Start Sound
unsafe extern "C" fn ssbexo_lucario_high_jump_kick_start_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE_REMAIN(agent, Hash40::new("vc_lucario_007"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_REMAIN(agent, Hash40::new("se_lucario_special_l02"));
    }
}

//High Jump Kick Start Expression
unsafe extern "C" fn ssbexo_lucario_high_jump_kick_start_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_counter"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//High Jump Kick ACMD
unsafe extern "C" fn ssbexo_lucario_high_jump_kick_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("legl"), 6.0, 300, 120, 0, 60, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 6.0, 300, 120, 0, 60, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("toel"), 6.0, 300, 120, 0, 60, 5.0, 1.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
}

//High Jump Kick Effect
unsafe extern "C" fn ssbexo_lucario_high_jump_kick_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::COL_PRI(agent, 200);
        macros::FLASH(agent, 0.1, 0, 0.4, 0.6);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_kagebunshin_appear"), Hash40::new("top"), 0, 7.3, -2.5, 20, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH_FRM(agent, 3, 0.1, 0, 0.4, 0);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
}

//High Jump Kick Sound
unsafe extern "C" fn ssbexo_lucario_high_jump_kick_sound(agent: &mut L2CAgentBase) {
    let current_aurapower = WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
    let middle_aurapower = WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_MIDDLE_AURAPOWER);
    let high_aurapower = WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_HIGH_AURAPOWER);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_lucario_special_l01"));
    }
    if current_aurapower == high_aurapower {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_l03_l"));
        }
    }
    else if current_aurapower == middle_aurapower {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_l03_m"));
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_l03_s"));
        }
    }
}

//High Jump Kick Expression
unsafe extern "C" fn ssbexo_lucario_high_jump_kick_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

//High Jump Kick Landing ACMD
unsafe extern "C" fn ssbexo_lucario_high_jump_kick_landing_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 75, 80, 0, 50, 11.0, 0.0, 0.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bomb"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    if !AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
        if macros::is_excute(agent) {
            macros::FT_ADD_DAMAGE(agent, 10.0);
            StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_DOWN, false);
        }
    }
}

//High Jump Kick Landing Effect
unsafe extern "C" fn ssbexo_lucario_high_jump_kick_landing_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::COL_PRI(agent, 200);
        macros::FLASH(agent, 0, 0, 0, 0);
        macros::EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
        macros::EFFECT(agent, Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::COL_NORMAL(agent);
    }
}

//High Jump Kick Landing Sound
unsafe extern "C" fn ssbexo_lucario_high_jump_kick_landing_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_LANDING_SE(agent, Hash40::new("se_lucario_landing02"));
        macros::PLAY_SE(agent, Hash40::new("se_lucario_smash_add02"));
    }
}

//High Jump Kick Landing Expression
unsafe extern "C" fn ssbexo_lucario_high_jump_kick_landing_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Dashing Force Palm ACMD
unsafe extern "C" fn ssbexo_lucario_dashing_force_palm_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 2.0, 5.0);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::IS_GENERATABLE_ARTICLE(agent, *FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG) {
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG, false, -1);
            macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 5.3, Some(0.0), Some(6.0), Some(1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
            macros::CATCH(agent, 1, Hash40::new("top"), 3.0, 0.0, 6.0, 7.4, Some(0.0), Some(6.0), Some(1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

//Qigong Shoot ACMD
unsafe extern "C" fn ssbexo_lucario_qigong_shoot_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 25, 60, 0, 45, 5.6, 0.0, 0.0, 4.0, Some(0.0), Some(0.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 25, 60, 0, 45, 3.0, 0.0, 0.0, 2.0, Some(0.0), Some(0.0), Some(22.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 14.0, 25, 60, 0, 45, 2.0, 0.0, -1.0, -4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

//Grounded Dashing Force Palm Effect
unsafe extern "C" fn ssbexo_lucario_grounded_dashing_force_palm_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("lucario_hakkei"), Hash40::new("top"), -2.7, 6.8, -1.7, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_hakkei_aura"), Hash40::new("havel"), 0, 0, 0.5, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_hakkei_aura"), Hash40::new("haver"), 0, 0, 0.5, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("lucario_hakkei_start"), Hash40::new("havel"), -0.9, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Dashing Force Palm Effect
unsafe extern "C" fn ssbexo_lucario_aerial_dashing_force_palm_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("lucario_hakkei"), Hash40::new("top"), -2.7, 10, -2.5, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_hakkei_aura"), Hash40::new("havel"), 0, 0, 0.5, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_hakkei_aura"), Hash40::new("haver"), 0, 0, 0.5, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("lucario_hakkei_start"), Hash40::new("havel"), -0.9, 0, 0, 0, 0, 0, 1, true);
    }
}

//Dashing Force Palm Sound
unsafe extern "C" fn ssbexo_lucario_dashing_force_palm_sound(agent: &mut L2CAgentBase) {
    let current_aurapower = WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
    let middle_aurapower = WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_MIDDLE_AURAPOWER);
    let high_aurapower = WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_HIGH_AURAPOWER);
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s03"));
    }
    wait(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_REMAIN(agent, Hash40::new("vc_lucario_004"));
    }
    if current_aurapower == high_aurapower {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s02_l"));
        }
    }
    else if current_aurapower == middle_aurapower {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s02_m"));
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s02_s"));
        }
    }
}

//Dashing Force Palm Expression
unsafe extern "C" fn ssbexo_lucario_dashing_force_palm_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 54.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

//Power Up Punch ACMD
unsafe extern "C" fn ssbexo_lucario_power_up_punch_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 12.0, 1, 55, 95, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 12.0, 1, 55, 95, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("handr"), 12.0, 1, 55, 95, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Power Up Punch Effect
unsafe extern "C" fn ssbexo_lucario_power_up_punch_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_hakkei_far"), Hash40::new("handr"), 0, 0, 1, 180, 0, 0, 0.8, true);
    }
}

//Power Up Punch Sound
unsafe extern "C" fn ssbexo_lucario_power_up_punch_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_lucario_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_lucario_attackl_s"));
    }
}

//Power Up Punch Expression
unsafe extern "C" fn ssbexo_lucario_power_up_punch_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

//Grounded Side Special ACMD
unsafe extern "C" fn ssbexo_lucario_grounded_side_special_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, Hash40::new("special_s"), false, -1.0);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 6.0, 367, 80, 60, 0, 5.0, 0.0, -8.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 6.0, 367, 80, 60, 0, 5.0, 0.0, -8.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 30.0, false);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Aerial Side Special ACMD
unsafe extern "C" fn ssbexo_lucario_aerial_side_special_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, Hash40::new("special_air_s"), false, -1.0);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 6.0, 367, 80, 60, 0, 5.0, 0.0, -8.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 6.0, 367, 80, 60, 0, 5.0, 0.0, -8.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 30.0, false);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Grounded Side Special Effect
unsafe extern "C" fn ssbexo_lucario_grounded_side_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, -6, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 6, 0, 0, 0, 0, 0.25, true);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, -6, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 6, 0, 0, 0, 0, 0.25, true);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Side Special Effect
unsafe extern "C" fn ssbexo_lucario_aerial_side_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, -6, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 6, 0, 0, 0, 0, 0.25, true);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, -6, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 6, 0, 0, 0, 0, 0.25, true);
    }
}

//Side Special Sound
unsafe extern "C" fn ssbexo_lucario_side_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucario_jump01"));
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_lucario_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_lucario_attackl_s"));
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucario_attackl_s"));
    }
}

//Grounded Side Special Expression
unsafe extern "C" fn ssbexo_lucario_grounded_side_special_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

//Aerial Side Special Expression
unsafe extern "C" fn ssbexo_lucario_aerial_side_special_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

//Grounded Side Special Hi ACMD
unsafe extern "C" fn ssbexo_lucario_grounded_side_special_hi_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, Hash40::new("special_s_hi"), false, -1.0);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 10.0, 85, 120, 0, 55, 5.0, 0.0, -8.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Aerial Side Special Hi ACMD
unsafe extern "C" fn ssbexo_lucario_aerial_side_special_hi_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, Hash40::new("special_air_s_hi"), false, -1.0);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 10.0, 85, 120, 0, 55, 5.0, 0.0, -8.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Grounded Side Special Hi Effect
unsafe extern "C" fn ssbexo_lucario_grounded_side_special_hi_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, -6, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 6, 0, 0, 0, 0, 0.25, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, -6, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 6, 0, 0, 0, 0, 0.25, true);
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Side Special Hi Effect
unsafe extern "C" fn ssbexo_lucario_aerial_side_special_hi_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, -6, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 6, 0, 0, 0, 0, 0.25, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, -6, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 6, 0, 0, 0, 0, 0.25, true);
    }
}

//Side Special Hi Sound
unsafe extern "C" fn ssbexo_lucario_side_special_hi_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_REMAIN(agent, Hash40::new("vc_lucario_005"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s01"));
        macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s02_l"));
    }
}

//Grounded Side Special Hi Expression
unsafe extern "C" fn ssbexo_lucario_grounded_side_special_hi_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

//Aerial Side Special Hi Expression
unsafe extern "C" fn ssbexo_lucario_aerial_side_special_hi_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

//Grounded Side Special Lw ACMD
unsafe extern "C" fn ssbexo_lucario_grounded_side_special_lw_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, Hash40::new("special_s_lw"), false, -1.0);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 10.0, 300, 40, 160, 0, 5.0, 0.0, -8.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 30.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 3.5);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Aerial Side Special Lw ACMD
unsafe extern "C" fn ssbexo_lucario_aerial_side_special_lw_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, Hash40::new("special_air_s_lw"), false, -1.0);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 10.0, 300, 40, 160, 0, 5.0, 0.0, -8.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 40.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 3.5);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Grounded Side Special Lw Effect
unsafe extern "C" fn ssbexo_lucario_grounded_side_special_lw_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, -6, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 6, 0, 0, 0, 0, 0.25, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, -6, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 6, 0, 0, 0, 0, 0.25, true);
    }
}

//Aerial Side Special Lw Effect
unsafe extern "C" fn ssbexo_lucario_aerial_side_special_lw_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, -6, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 6, 0, 0, 0, 0, 0.25, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, -6, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 6, 0, 0, 0, 0, 0.25, true);
    }
}

//Side Special Lw Sound
unsafe extern "C" fn ssbexo_lucario_side_special_lw_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_REMAIN(agent, Hash40::new("vc_lucario_005"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s01"));
        macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s02_l"));
    }
}

//Grounded Side Special Lw Expression
unsafe extern "C" fn ssbexo_lucario_grounded_side_special_lw_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 49.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

//Aerial Side Special Lw Expression
unsafe extern "C" fn ssbexo_lucario_aerial_side_special_lw_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

//Down Special ACMD
unsafe extern "C" fn ssbexo_lucario_down_special_acmd(agent: &mut L2CAgentBase) {
    let aura_level = WorkModule::get_int(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURA_LEVEL);
    frame(agent.lua_state_agent, 78.0);
    if aura_level < 9 {
        if macros::is_excute(agent) {
            agent.gimmick_flash();
            WorkModule::inc_int(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURA_LEVEL);
        }
    }
    if aura_level == 9 {
        if macros::is_excute(agent) {
            agent.gimmick_flash();
            WorkModule::inc_int(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURA_LEVEL);
            //macros::FILL_SCREEN_MODEL_COLOR(agent, 0, 10, 0.3, 0.3, 0.3, 0, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, *EFFECT_SCREEN_PRIO_FINAL);
        }
    }
}

//Down Special Effect
unsafe extern "C" fn ssbexo_lucario_down_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_shield_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, true);
    }
    frame(agent.lua_state_agent, 55.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_shield_smoke"), false, false);
    }
    frame(agent.lua_state_agent, 62.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

//Down Special Sound
unsafe extern "C" fn ssbexo_lucario_down_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucario_appeal_h01"));
        macros::PLAY_SE(agent, Hash40::new("vc_lucario_appeal01"));
    }
}

//Down Special Expression
unsafe extern "C" fn ssbexo_lucario_down_special_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 32, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Mega Evolve ACMD
unsafe extern "C" fn ssbexo_lucario_mega_evolve_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("gamemodel"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("lucari_heavyouch_mouth"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("lucari_open_mouth"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("lucario_attack_eye"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("lucario_blink_eye"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("lucario_close_mouth"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("lucario_eye"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("lucario_halfblink_eye"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("lucario_halfclose_mouth"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("lucario_heavyattack_eye"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("lucario_heavyouch_eye"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("lucario_openblink"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("lucario_ouch_eye"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("lucario_small_eye"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("waist_fur"), false);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_LUCARIOM, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Mega Evolve Effect
unsafe extern "C" fn ssbexo_lucario_mega_evolve_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("lucario_final_megasinka"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}

//Mega Evolve Sound
unsafe extern "C" fn ssbexo_lucario_mega_evolve_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE_REMAIN(agent, Hash40::new("vc_lucario_final01"));
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_REMAIN(agent, Hash40::new("se_lucario_final01"));
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_REMAIN(agent, Hash40::new("se_lucario_final01_02"));
    }
}

//Mega Evolve Expression
unsafe extern "C" fn ssbexo_lucario_mega_evolve_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        START_INFO_FLASH_EYE(agent.lua_state_agent);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 84.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_rush"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("lucario")
    .game_acmd("game_highjumpkickstart", ssbexo_lucario_high_jump_kick_start_acmd, Priority::Low)
    .effect_acmd("effect_highjumpkickstart", ssbexo_lucario_high_jump_kick_start_effect, Priority::Low)
    .sound_acmd("sound_highjumpkickstart", ssbexo_lucario_high_jump_kick_start_sound, Priority::Low)
    .expression_acmd("expression_highjumpkickstart", ssbexo_lucario_high_jump_kick_start_expression, Priority::Low)
    .game_acmd("game_highjumpkick", ssbexo_lucario_high_jump_kick_acmd, Priority::Low)
    .effect_acmd("effect_highjumpkick", ssbexo_lucario_high_jump_kick_effect, Priority::Low)
    .sound_acmd("sound_highjumpkick", ssbexo_lucario_high_jump_kick_sound, Priority::Low)
    .expression_acmd("expression_highjumpkick", ssbexo_lucario_high_jump_kick_expression, Priority::Low)
    .game_acmd("game_highjumpkicklanding", ssbexo_lucario_high_jump_kick_landing_acmd, Priority::Low)
    .effect_acmd("effect_highjumpkicklanding", ssbexo_lucario_high_jump_kick_landing_effect, Priority::Low)
    .sound_acmd("sound_highjumpkicklanding", ssbexo_lucario_high_jump_kick_landing_sound, Priority::Low)
    .expression_acmd("expression_highjumpkicklanding", ssbexo_lucario_high_jump_kick_landing_expression, Priority::Low)
    .game_acmd("game_dashingforcepalm", ssbexo_lucario_dashing_force_palm_acmd, Priority::Low)
    .game_acmd("game_dashingforcepalmair", ssbexo_lucario_dashing_force_palm_acmd, Priority::Low)
    .effect_acmd("effect_dashingforcepalm", ssbexo_lucario_grounded_dashing_force_palm_effect, Priority::Low)
    .effect_acmd("effect_dashingforcepalmair", ssbexo_lucario_aerial_dashing_force_palm_effect, Priority::Low)
    .sound_acmd("sound_dashingforcepalm", ssbexo_lucario_dashing_force_palm_sound, Priority::Low)
    .sound_acmd("sound_dashingforcepalmair", ssbexo_lucario_dashing_force_palm_sound, Priority::Low)
    .expression_acmd("expression_dashingforcepalm", ssbexo_lucario_dashing_force_palm_expression, Priority::Low)
    .expression_acmd("expression_dashingforcepalmair", ssbexo_lucario_dashing_force_palm_expression, Priority::Low)
    .game_acmd("game_poweruppunch", ssbexo_lucario_power_up_punch_acmd, Priority::Low)
    .effect_acmd("effect_poweruppunch", ssbexo_lucario_power_up_punch_effect, Priority::Low)
    .sound_acmd("sound_poweruppunch", ssbexo_lucario_power_up_punch_sound, Priority::Low)
    .expression_acmd("expression_poweruppunch", ssbexo_lucario_power_up_punch_expression, Priority::Low)
    .game_acmd("game_specials", ssbexo_lucario_grounded_side_special_acmd, Priority::Low)
    .game_acmd("game_specialairs", ssbexo_lucario_aerial_side_special_acmd, Priority::Low)
    .effect_acmd("effect_specials", ssbexo_lucario_grounded_side_special_effect, Priority::Low)
    .effect_acmd("effect_specialairs", ssbexo_lucario_aerial_side_special_effect, Priority::Low)
    .sound_acmd("sound_specials", ssbexo_lucario_side_special_sound, Priority::Low)
    .sound_acmd("sound_specialairs", ssbexo_lucario_side_special_sound, Priority::Low)
    .expression_acmd("expression_specials", ssbexo_lucario_grounded_side_special_expression, Priority::Low)
    .expression_acmd("expression_specialairs", ssbexo_lucario_aerial_side_special_expression, Priority::Low)
    .game_acmd("game_specialshi", ssbexo_lucario_grounded_side_special_hi_acmd, Priority::Low)
    .game_acmd("game_specialairshi", ssbexo_lucario_aerial_side_special_hi_acmd, Priority::Low)
    .effect_acmd("effect_specialshi", ssbexo_lucario_grounded_side_special_hi_effect, Priority::Low)
    .effect_acmd("effect_specialairshi", ssbexo_lucario_aerial_side_special_hi_effect, Priority::Low)
    .sound_acmd("sound_specialshi", ssbexo_lucario_side_special_hi_sound, Priority::Low)
    .sound_acmd("sound_specialairshi", ssbexo_lucario_side_special_hi_sound, Priority::Low)
    .expression_acmd("expression_specialshi", ssbexo_lucario_grounded_side_special_hi_expression, Priority::Low)
    .expression_acmd("expression_specialairshi", ssbexo_lucario_aerial_side_special_hi_expression, Priority::Low)
    .game_acmd("game_specialslw", ssbexo_lucario_grounded_side_special_lw_acmd, Priority::Low)
    .game_acmd("game_specialairslw", ssbexo_lucario_aerial_side_special_lw_acmd, Priority::Low)
    .effect_acmd("effect_specialslw", ssbexo_lucario_grounded_side_special_lw_effect, Priority::Low)
    .effect_acmd("effect_specialairslw", ssbexo_lucario_aerial_side_special_lw_effect, Priority::Low)
    .sound_acmd("sound_specialslw", ssbexo_lucario_side_special_lw_sound, Priority::Low)
    .sound_acmd("sound_specialairslw", ssbexo_lucario_side_special_lw_sound, Priority::Low)
    .expression_acmd("expression_specialslw", ssbexo_lucario_grounded_side_special_lw_expression, Priority::Low)
    .expression_acmd("expression_specialairslw", ssbexo_lucario_aerial_side_special_lw_expression, Priority::Low)
    .game_acmd("game_speciallw", ssbexo_lucario_down_special_acmd, Priority::Low)
    .game_acmd("game_specialairlw", ssbexo_lucario_down_special_acmd, Priority::Low)
    .effect_acmd("effect_speciallw", ssbexo_lucario_down_special_effect, Priority::Low)
    .effect_acmd("effect_specialairlw", ssbexo_lucario_down_special_effect, Priority::Low)
    .sound_acmd("sound_speciallw", ssbexo_lucario_down_special_sound, Priority::Low)
    .sound_acmd("sound_specialairlw", ssbexo_lucario_down_special_sound, Priority::Low)
    .expression_acmd("expression_speciallw", ssbexo_lucario_down_special_expression, Priority::Low)
    .expression_acmd("expression_specialairlw", ssbexo_lucario_down_special_expression, Priority::Low)
    .game_acmd("game_megaevolve", ssbexo_lucario_mega_evolve_acmd, Priority::Low)
    .game_acmd("game_megaevolveair", ssbexo_lucario_mega_evolve_acmd, Priority::Low)
    .effect_acmd("effect_megaevolve", ssbexo_lucario_mega_evolve_effect, Priority::Low)
    .effect_acmd("effect_megaevolveair", ssbexo_lucario_mega_evolve_effect, Priority::Low)
    .sound_acmd("sound_megaevolve", ssbexo_lucario_mega_evolve_sound, Priority::Low)
    .sound_acmd("sound_megaevolveair", ssbexo_lucario_mega_evolve_sound, Priority::Low)
    .expression_acmd("expression_megaevolve", ssbexo_lucario_mega_evolve_expression, Priority::Low)
    .expression_acmd("expression_megaevolveair", ssbexo_lucario_mega_evolve_expression, Priority::Low)
    .install()
    ;
    Agent::new("lucario_qigong")
    .game_acmd("game_shoot", ssbexo_lucario_qigong_shoot_acmd, Priority::Low)
    .install()
    ;
}