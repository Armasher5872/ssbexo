use super::*;

//Neutral Special ACMD
unsafe extern "C" fn ssbexo_armstrong_neutral_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0.0);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0.0);
        }
    }
    frame(agent.lua_state_agent, 80.0);
    if is_excute(agent) {
        if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
        }
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
        ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 40.0, 361, 46, 0, 80, 5.0, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("bust"), 40.0, 361, 46, 0, 80, 5.8, 0.0, 1.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("arml"), 40.0, 361, 46, 0, 80, 5.8, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
                QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
            }
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
            ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 40.0, 361, 46, 0, 80, 5.0, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("bust"), 40.0, 361, 46, 0, 80, 5.8, 0.0, 1.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 2, 0, Hash40::new("arml"), 40.0, 361, 46, 0, 80, 5.8, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        }
    }
    frame(agent.lua_state_agent, 84.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
        AttackModule::clear_all(agent.module_accessor);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
            AttackModule::clear_all(agent.module_accessor);
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
        }
    }
    frame(agent.lua_state_agent, 95.0);
    if is_excute(agent) {
        if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        }
        ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 361, 80, 0, 60, 6.0, 0.0, 7.0, 16.0, Some(0.0), Some(22.0), Some(16.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 20.0, 361, 80, 0, 60, 6.0, 0.0, 4.0, 16.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 100.0);
    if is_excute(agent) {
        if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        }
        ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 361, 80, 0, 60, 6.0, 0.0, 7.0, 30.0, Some(0.0), Some(22.0), Some(30.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 20.0, 361, 80, 0, 60, 6.0, 0.0, 4.0, 30.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 105.0);
    if is_excute(agent) {
        if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        }
        ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 361, 80, 0, 60, 6.0, 0.0, 7.0, 44.0, Some(0.0), Some(22.0), Some(44.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 20.0, 361, 80, 0, 60, 6.0, 0.0, 4.0, 44.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 110.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 159.0);
    if is_excute(agent) {
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
    }
}

//Neutral Special Effect
unsafe extern "C" fn ssbexo_armstrong_neutral_special_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("ganon_attack_fire"), Hash40::new("havel"), 2, 0, 0, 0, 0, 0, 0.8, 2, 2, 2, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
    }
    frame(agent.lua_state_agent, 55.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_down_smoke"), false, true);
    }
    frame(agent.lua_state_agent, 65.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_hold"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
    }
    frame(agent.lua_state_agent, 77.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 12, 3, 0, -40, -90, 1.4, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(agent.lua_state_agent, 80.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken"), Hash40::new("havel"), 0, 0.0, 0.0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(agent.lua_state_agent, 81.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 16, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("edge_gokumon_impact"), Hash40::new("top"), 0, 0, 16, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        LAST_EFFECT_SET_RATE(agent, 0.5);
    }
    frame(agent.lua_state_agent, 86.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 30, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("edge_gokumon_impact"), Hash40::new("top"), 0, 0, 30, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        LAST_EFFECT_SET_RATE(agent, 0.5);
    }
    frame(agent.lua_state_agent, 91.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 44, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("edge_gokumon_impact"), Hash40::new("top"), 0, 0, 44, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        LAST_EFFECT_SET_RATE(agent, 0.5);
    }
    frame(agent.lua_state_agent, 95.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 16, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        LANDING_EFFECT(agent, Hash40::new("ganon_majinken"), Hash40::new("top"), 16, 0, 0, 90, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 100.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 30, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        LANDING_EFFECT(agent, Hash40::new("ganon_majinken"), Hash40::new("top"), 30, 0, 0, 90, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 105.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 44, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        LANDING_EFFECT(agent, Hash40::new("ganon_majinken"), Hash40::new("top"), 44, 0, 0, 90, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

//Neutral Special Sound
unsafe extern "C" fn ssbexo_armstrong_neutral_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_special_n01"));
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_n01"));
    }
    frame(agent.lua_state_agent, 60.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_attack05"));
    }
    frame(agent.lua_state_agent, 80.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_n02"));
    }
    frame(agent.lua_state_agent, 95.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_attackhard_03"));
    }
    frame(agent.lua_state_agent, 100.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_attackhard_03"));
    }
    frame(agent.lua_state_agent, 105.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_attackhard_03"));
    }
}

//Aerial Neutral Special ACMD
unsafe extern "C" fn ssbexo_armstrong_aerial_neutral_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        }
        ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 14.0, 285, 65, 0, 65, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 14.0, 285, 65, 0, 65, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("handl"), 16.0, 285, 65, 0, 65, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
                QUAKE(agent, *CAMERA_QUAKE_KIND_M);
            }
            ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 14.0, 285, 65, 0, 65, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("arml"), 14.0, 285, 65, 0, 65, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("handl"), 16.0, 285, 65, 0, 65, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        }
        ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 14.0, 45, 65, 0, 65, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 14.0, 45, 65, 0, 65, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("handl"), 16.0, 45, 65, 0, 65, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
                QUAKE(agent, *CAMERA_QUAKE_KIND_M);
            }
            ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 14.0, 45, 65, 0, 65, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("arml"), 14.0, 45, 65, 0, 65, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("handl"), 16.0, 45, 65, 0, 65, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

//Aerial Neutral Special Effect
unsafe extern "C" fn ssbexo_armstrong_aerial_neutral_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 4, 3, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, 0.88, 0.35, 0.13);
        smash::app::sv_animcmd::EFFECT_COLOR(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_rekkikyaku"), Hash40::new("handl"), 12, -1.5, 0, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
}

//Aerial Neutral Special Sound
unsafe extern "C" fn ssbexo_armstrong_aerial_neutral_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_dash_start"));
    }
    wait(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_l01"));
        PLAY_SE(agent, Hash40::new("vc_ganon_special_l02"));
    }
}

//Grounded Side Special Start ACMD
unsafe extern "C" fn ssbexo_armstrong_grounded_side_special_start_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 9.0);
        DamageModule::set_reaction_mul(agent.module_accessor, 0.85);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 6.8, 6.7, 7.5, 3.8);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 3.0, 7.5, Some(0.0), Some(12.0), Some(7.5), *FIGHTER_STATUS_KIND_CATCHED_GANON, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 31.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        AttackModule::clear_all(agent.module_accessor);
        AttackModule::set_power_up(agent.module_accessor, 1.0);
        DamageModule::set_reaction_mul(agent.module_accessor, 1.0);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
        WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
        WorkModule::set_float(agent.module_accessor, 1.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
        WorkModule::set_float(agent.module_accessor, 1.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
    }
    frame(agent.lua_state_agent, 42.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 7.0, 7.0, 7.5, 7.5);
    }
}

//Grounded Side Special Start Effect
unsafe extern "C" fn ssbexo_armstrong_grounded_side_special_start_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_flash"), Hash40::new("havel"), -1, 0, 0.5, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    for _ in 0..7 {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 2, 0, 4, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 4.0);
    }
}

//Aerial Side Special Start ACMD
unsafe extern "C" fn ssbexo_armstrong_aerial_side_special_start_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 6.0, 7.5, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 5.0, 0.0, 11.0, 7.5, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON, *COLLISION_SITUATION_MASK_GA);
    }
    FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_EXPLOSION_GRAVITY_ONOFF);
    }
    frame(agent.lua_state_agent, 32.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        AttackModule::clear_all(agent.module_accessor);
        AttackModule::set_power_up(agent.module_accessor, 1.0);
        WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
        WorkModule::set_float(agent.module_accessor, 1.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
    }
}

//Aerial Side Special Start Effect
unsafe extern "C" fn ssbexo_armstrong_aerial_side_special_start_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_flash"), Hash40::new("havel"), 0, 0, 0, -0.286, -45, 25, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Grounded Side Special Catch Effect
unsafe extern "C" fn ssbexo_armstrong_grounded_side_special_catch_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("sys_catch"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_catch"), Hash40::new("havel"), -1, 0, 0.5, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 31.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("ganon_engokua"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
    }
}

//Aerial Side Special Catch ACMD
unsafe extern "C" fn ssbexo_armstrong_aerial_side_special_catch_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, *FIGHTER_GANON_EXPLOSION_FALL_SETTING_CATCH, *FIGHTER_GANON_STATUS_WORK_ID_INT_EXPLOSION_FALL_SETTING);
    }
}

//Aerial Side Special Catch Effect
unsafe extern "C" fn ssbexo_armstrong_aerial_side_special_catch_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("sys_catch"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_flash"), Hash40::new("havel"), -1, 0, 0.5, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_catch"), Hash40::new("havel"), -1, 0, 0.5, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
    }
}

//Aerial Side Special Fall ACMD
unsafe extern "C" fn ssbexo_armstrong_aerial_side_special_fall_acmd(agent: &mut L2CAgentBase) {
    let explosion_air_speed_x = WorkModule::get_float(agent.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLOAT_EXPLOSION_AIR_SPEED_X);
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        SET_SPEED_EX(agent, explosion_air_speed_x, -5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        KineticModule::suspend_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, *FIGHTER_GANON_EXPLOSION_FALL_SETTING_FALL, *FIGHTER_GANON_STATUS_WORK_ID_INT_EXPLOSION_FALL_SETTING);
    }
}

//Aerial Side Special Fall Effect
unsafe extern "C" fn ssbexo_armstrong_aerial_side_special_fall_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_catch"), Hash40::new("havel"), -1, 0, 0.5, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
    }
}

//Up Special ACMD
unsafe extern "C" fn ssbexo_armstrong_up_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.4, 0.0, 16.0, 6.5, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_GANON, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 6.5, 0.0, 8.8, 13.7, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_GANON, *COLLISION_SITUATION_MASK_GA);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 0, 50, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR, 1);
        CATCH(agent, 0, Hash40::new("top"), 4.4, 0.0, 16.0, 6.5, Some(0.0), Some(18.0), Some(3.0), *FIGHTER_STATUS_KIND_CLUNG_GANON, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 29.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 34.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 7.0, 70, 90, 0, 70, 9.0, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 7.0, 70, 90, 0, 70, 6.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 46.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GANON_STATUS_SPECIAL_HI_FLAG_IS_CHECK_DIVE);
    }
}

//Up Special Effect
unsafe extern "C" fn ssbexo_armstrong_up_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_raijin_hold"), Hash40::new("armr"), 2, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 31.0);
    for _ in 0..4 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("ganon_attack_elec"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    frame(agent.lua_state_agent, 42.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_raijin_hold"), false, true);
    }
}

//Up Special Catch ACMD
unsafe extern "C" fn ssbexo_armstrong_up_special_catch_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 8.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
    }
}

//Up Special Catch Effect
unsafe extern "C" fn ssbexo_armstrong_up_special_catch_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FLW_POS_NO_STOP(agent, Hash40::new("sys_catch"), Hash40::new("haver"), 1, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FLW_POS_NO_STOP(agent, Hash40::new("ganon_raijin"), Hash40::new("haver"), 3, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
    }
}

/*
//Up Special Throw ACMD
unsafe extern "C" fn ssbexo_armstrong_up_special_throw_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 361, 108, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 8.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(agent.lua_state_agent, 2.0);
    FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 46.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GANON_STATUS_SPECIAL_HI_THROW_FLAG_FALL);
    }
}
*/

//Up Special Throw Effect
unsafe extern "C" fn ssbexo_armstrong_up_special_throw_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("ganon_raijin"), false, true);
        EFFECT(agent, Hash40::new("ganon_raijin_bomb"), Hash40::new("top"), 0, 12, -4, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
    }
}

//Grounded Down Special ACMD
unsafe extern "C" fn ssbexo_armstrong_grounded_down_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 9.0);
        DamageModule::set_reaction_mul(agent.module_accessor, 0.85);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        SEARCH(agent, 0, 0, Hash40::new("top"), 13.5, 0.0, 7.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        let damage_charge_multiplier = WorkModule::get_float(agent.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 13.5, 0.0, 9.0, 6.0, 0.0, 7.0, 0.0, damage_charge_multiplier, damage_charge_multiplier, 80, false, 1.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 100, 0, 60, 13.5, 6.0, 9.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        AttackModule::clear_all(agent.module_accessor);
        AttackModule::set_power_up(agent.module_accessor, 1.0);
        DamageModule::set_reaction_mul(agent.module_accessor, 1.0);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
        WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
        WorkModule::set_float(agent.module_accessor, 1.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
        WorkModule::set_float(agent.module_accessor, 1.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
    }
}

//Grounded Down Special Effect
unsafe extern "C" fn ssbexo_armstrong_grounded_down_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_dead_light"), Hash40::new("sys_dead_light"), Hash40::new("top"), 0, 8, 0.0, 0, -90, 0, 0.25, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
    }
    frame(agent.lua_state_agent, 30.0);
    for _ in 0..20 {
        if is_excute(agent) {
            EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_dead_light"), false, true);
        }
        wait(agent.lua_state_agent, 1.0);
    }
}

//Aerial Down Special ACMD
unsafe extern "C" fn ssbexo_armstrong_aerial_down_special_acmd(agent: &mut L2CAgentBase) {
    let damage_charge_multiplier = WorkModule::get_float(agent.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        SEARCH(agent, 0, 0, Hash40::new("top"), 13.5, 0.0, 7.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 13.5, 0.0, 9.0, 6.0, 0.0, 7.0, 0.0, damage_charge_multiplier, damage_charge_multiplier, 80, false, 1.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 100, 0, 60, 13.5, 6.0, 9.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Aerial Down Special Effect
unsafe extern "C" fn ssbexo_armstrong_aerial_down_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_dead_light"), Hash40::new("sys_dead_light"), Hash40::new("top"), 0, 8, 0.0, 0, -90, 0, 0.25, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
    }
    frame(agent.lua_state_agent, 30.0);
    for _ in 0..20 {
        if is_excute(agent) {
            EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_dead_light"), false, true);
        }
        wait(agent.lua_state_agent, 1.0);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialn", ssbexo_armstrong_neutral_special_acmd, Low)
    .effect_acmd("effect_specialn", ssbexo_armstrong_neutral_special_effect, Low)
    .sound_acmd("sound_specialn", ssbexo_armstrong_neutral_special_sound, Low)
    .game_acmd("game_specialairn", ssbexo_armstrong_aerial_neutral_special_acmd, Low)
    .effect_acmd("effect_specialairn", ssbexo_armstrong_aerial_neutral_special_effect, Low)
    .sound_acmd("sound_specialairn", ssbexo_armstrong_aerial_neutral_special_sound, Low)
    .game_acmd("game_specialsstart", ssbexo_armstrong_grounded_side_special_start_acmd, Low)
    .game_acmd("game_specialairsstart", ssbexo_armstrong_aerial_side_special_start_acmd, Low)
    .effect_acmd("effect_specialsstart", ssbexo_armstrong_grounded_side_special_start_effect, Low)
    .effect_acmd("effect_specialairsstart", ssbexo_armstrong_aerial_side_special_start_effect, Low)
    .game_acmd("game_specialairscatch", ssbexo_armstrong_aerial_side_special_catch_acmd, Low)
    .effect_acmd("effect_specials", ssbexo_armstrong_grounded_side_special_catch_effect, Low)
    .effect_acmd("effect_specialairscatch", ssbexo_armstrong_aerial_side_special_catch_effect, Low)
    .game_acmd("game_specialairsfall", ssbexo_armstrong_aerial_side_special_fall_acmd, Low)
    .effect_acmd("effect_specialairsfall", ssbexo_armstrong_aerial_side_special_fall_effect, Low)
    .game_acmd("game_specialhi", ssbexo_armstrong_up_special_acmd, Low)
    .effect_acmd("effect_specialhi", ssbexo_armstrong_up_special_effect, Low)
    .game_acmd("game_specialairhi", ssbexo_armstrong_up_special_acmd, Low)
    .effect_acmd("effect_specialairhi", ssbexo_armstrong_up_special_effect, Low)
    .game_acmd("game_specialhicatch", ssbexo_armstrong_up_special_catch_acmd, Low)
    .effect_acmd("effect_specialhicatch", ssbexo_armstrong_up_special_catch_effect, Low)
    //.game_acmd("game_specialhithrow", ssbexo_armstrong_up_special_throw_acmd, Low)
    .effect_acmd("effect_specialhithrow", ssbexo_armstrong_up_special_throw_effect, Low)
    .game_acmd("game_speciallw", ssbexo_armstrong_grounded_down_special_acmd, Low)
    .effect_acmd("effect_speciallw", ssbexo_armstrong_grounded_down_special_effect, Low)
    .game_acmd("game_specialairlw", ssbexo_armstrong_aerial_down_special_acmd, Low)
    .effect_acmd("effect_specialairlw", ssbexo_armstrong_aerial_down_special_effect, Low)
    .install()
    ;
}