use super::*;

//Guard Breaker Start ACMD
unsafe extern "C" fn ssbexo_miiswordsman_guard_breaker_start_acmd(_agent: &mut L2CAgentBase) {}

//Guard Breaker Start Effect
unsafe extern "C" fn ssbexo_miiswordsman_guard_breaker_start_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 1.5, 9, 0, 0, 180, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("miiswordsman_rapid_slash_enchant"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
}

//Guard Breaker Start Sound
unsafe extern "C" fn ssbexo_miiswordsman_guard_breaker_start_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_miiswordsman_special_s04"));
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_miiswordsman_special_s01"));
    }
}

//Grounded Guard Breaker Start Expression
unsafe extern "C" fn ssbexo_miiswordsman_grounded_guard_breaker_start_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x26769bd1de), 0, 0, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
}

//Aerial Guard Breaker Start Expression
unsafe extern "C" fn ssbexo_miiswordsman_aerial_guard_breaker_start_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x26769bd1de), 0, 0, 0);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
}

//Guard Breaker Loop Effect
unsafe extern "C" fn ssbexo_miiswordsman_guard_breaker_loop_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    if macros::is_excute(agent) {
        macros::FLASH(agent, 0.6, 0.6, 0.6, 0.3);
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("null"), Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 4, 0, -5, 0, 0, 0, 0.75, 4, 0, 3.5, 0, 0, 0, true);
        sv_animcmd::EFFECT_BRANCH_SITUATION(agent.lua_state_agent);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH_FRM(agent, 2, 0.6, 0.6, 0.6, 0);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 0.2, 0.5, 1, 0.25);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH_FRM(agent, 1, 0, 0.2, 1, 0);
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 6, 0, 0, 0, 0, 1, 2, 2, 8, 0, 0, 0, true);
    }
    wait(agent.lua_state_agent, 3.0);
    macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 6, 0, 0, 0, 0, 1, 2, 2, 8, 0, 0, 0, true);
}

//Guard Breaker Loop Expression
unsafe extern "C" fn ssbexo_miiswordsman_guard_breaker_loop_expression(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
            ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(agent.lua_state_agent, 1.0);
    }
}

//Guard Breaker Attack ACMD
unsafe extern "C" fn ssbexo_miiswordsman_guard_breaker_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::set_power_up(agent.module_accessor, WorkModule::get_float(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLOAT_GUARD_BREAKER_POWER));
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 15.0, 361, 130, 0, 50, 1.2, 0.0, 14.0, 0.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 10.0, 361, 130, 0, 50, 1.2, 0.0, 14.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 7.0, 361, 120, 0, 50, 3.5, 0.0, 2.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 16, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(agent) {
            AttackModule::set_power_up(agent.module_accessor, WorkModule::get_float(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLOAT_GUARD_BREAKER_POWER));
            macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 15.0, 361, 130, 0, 50, 1.2, 0.0, 14.0, 0.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 10.0, 361, 130, 0, 50, 1.2, 0.0, 14.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 7.0, 361, 120, 0, 50, 3.5, 0.0, 2.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 16, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Guard Breaker Attack Hi ACMD
unsafe extern "C" fn ssbexo_miiswordsman_guard_breaker_attack_hi_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::set_power_up(agent.module_accessor, WorkModule::get_float(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLOAT_GUARD_BREAKER_POWER));
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 15.0, 361, 130, 0, 50, 1.2, 0.0, 14.0, 0.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 10.0, 361, 130, 0, 50, 1.2, 0.0, 14.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 7.0, 361, 120, 0, 50, 3.5, 0.0, 2.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 16, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(agent) {
            AttackModule::set_power_up(agent.module_accessor, WorkModule::get_float(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLOAT_GUARD_BREAKER_POWER));
            macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 15.0, 361, 130, 0, 50, 1.2, 0.0, 14.0, 0.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 10.0, 361, 130, 0, 50, 1.2, 0.0, 14.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 7.0, 361, 120, 0, 50, 3.5, 0.0, 2.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 16, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Guard Breaker Attack Lw ACMD
unsafe extern "C" fn ssbexo_miiswordsman_guard_breaker_attack_lw_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::set_power_up(agent.module_accessor, WorkModule::get_float(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLOAT_GUARD_BREAKER_POWER));
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 15.0, 361, 130, 0, 50, 1.2, 0.0, 14.0, 0.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 10.0, 361, 130, 0, 50, 1.2, 0.0, 14.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 7.0, 361, 120, 0, 50, 3.5, 0.0, 2.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 16, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(agent) {
            AttackModule::set_power_up(agent.module_accessor, WorkModule::get_float(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLOAT_GUARD_BREAKER_POWER));
            macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 15.0, 361, 130, 0, 50, 1.2, 0.0, 14.0, 0.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 10.0, 361, 130, 0, 50, 1.2, 0.0, 14.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 7.0, 361, 120, 0, 50, 3.5, 0.0, 2.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 16, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Guard Breaker Attack Effect
unsafe extern "C" fn ssbexo_miiswordsman_guard_breaker_attack_effect(agent: &mut L2CAgentBase) {
    let tex_sword = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD) as u64;
    let sword_add = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD) as u64;
    let sword_flare = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE) as u64;
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 4, 0, -5, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new_raw(tex_sword), Hash40::new_raw(sword_add), 3, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 0.2, 0.0, true, Hash40::new_raw(sword_flare), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 2.0, 0.2);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 2);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
}

//Guard Breaker Attack Sound
unsafe extern "C" fn ssbexo_miiswordsman_guard_breaker_attack_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_miiswordsman_special_s01"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_special_c2_s01"));
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_miiswordsman_special_c2_s01"));
        macros::SET_PLAY_INHIVIT(agent, Hash40::new("se_miiswordsman_special_c2_s01"), 15);
    }
}

//Guard Breaker Attack Expression
unsafe extern "C" fn ssbexo_miiswordsman_guard_breaker_attack_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Shuriken of Light ACMD
unsafe extern "C" fn ssbexo_miiswordsman_shuriken_of_light_fire_acmd(agent: &mut L2CAgentBase) {
    let shuriken_of_light_count = WorkModule::get_int(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_LIGHT_SHURIKEN_COUNT);
    if shuriken_of_light_count == 3 {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_LIGHTSHURIKEN, false, -1);
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            ArticleModule::shoot(agent.module_accessor, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_LIGHTSHURIKEN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_LIGHTSHURIKEN, false, -1);
        }
        frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            ArticleModule::shoot(agent.module_accessor, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_LIGHTSHURIKEN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
        frame(agent.lua_state_agent, 19.0);
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_LIGHTSHURIKEN, false, -1);
        }
        frame(agent.lua_state_agent, 21.0);
        if macros::is_excute(agent) {
            ArticleModule::shoot(agent.module_accessor, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_LIGHTSHURIKEN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
    }
    else if shuriken_of_light_count == 2 {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_LIGHTSHURIKEN, false, -1);
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            ArticleModule::shoot(agent.module_accessor, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_LIGHTSHURIKEN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_LIGHTSHURIKEN, false, -1);
        }
        frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            ArticleModule::shoot(agent.module_accessor, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_LIGHTSHURIKEN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
    }
    else {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_LIGHTSHURIKEN, false, -1);
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            ArticleModule::shoot(agent.module_accessor, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_LIGHTSHURIKEN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
    }
}

//Shuriken of Light Fly ACMD
unsafe extern "C" fn ssbexo_miiswordsman_shuriken_of_light_fly_acmd(agent: &mut L2CAgentBase) {
    let owner_boma = get_owner_boma(agent);
    let shuriken_of_light_count = WorkModule::get_int(owner_boma, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_LIGHT_SHURIKEN_COUNT);
    if shuriken_of_light_count == 3 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 45, 0, 20, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -1, 0.0, 0, true, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            AttackModule::enable_safe_pos(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 45, 0, 20, 2.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-3.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -1, 0.0, 0, true, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        }
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 361, 60, 0, 20, 2.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -1.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        }
        frame(agent.lua_state_agent, 16.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 95, 0, 30, 2.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        }
        frame(agent.lua_state_agent, 24.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 130, 0, 40, 2.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        }
    }
    else if shuriken_of_light_count == 2 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 361, 45, 0, 20, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            AttackModule::enable_safe_pos(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 361, 45, 0, 20, 2.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-3.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 80, 60, 60, 0, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            AttackModule::set_paralyze_frame(agent.module_accessor, 0, 25, false);
            AttackModule::enable_safe_pos(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 80, 60, 60, 0, 2.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-3.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            AttackModule::set_paralyze_frame(agent.module_accessor, 0, 25, false);
        }
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 80, 40, 40, 0, 2.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            AttackModule::set_paralyze_frame(agent.module_accessor, 0, 25, false);
        }
        frame(agent.lua_state_agent, 16.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.5, 80, 20, 20, 0, 2.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            AttackModule::set_paralyze_frame(agent.module_accessor, 0, 25, false);
        }
        frame(agent.lua_state_agent, 24.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 80, 10, 0, 0, 2.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            AttackModule::set_paralyze_frame(agent.module_accessor, 0, 25, false);
        }
    }
}

//Blurring Slashes 1 ACMD
unsafe extern "C" fn ssbexo_miiswordsman_blurring_slashes_1_acmd(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_BLURRING_SLASHES_CANCEL) {
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 20, 0, 100, 10.0, 0.0, 10.0, 9.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
            if macros::is_excute(agent) {
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 20, 0, 100, 10.0, 0.0, 10.0, 9.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    else {
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 366, 100, 60, 0, 10.0, 0.0, 10.0, 9.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
            if macros::is_excute(agent) {
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 366, 100, 60, 0, 10.0, 0.0, 10.0, 9.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

//Grounded Blurring Slashes 1 Effect
unsafe extern "C" fn ssbexo_miiswordsman_grounded_blurring_slashes_1_effect(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_BLURRING_SLASHES_CANCEL) {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_miiswordsman_sword65"), Hash40::new("tex_miiswordsman_sword66"), 3, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 0.2, 0.0, true, Hash40::new(""), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 2.0, 0.2);
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("haver"), -0.0, 12, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 2);
        }
    }
    else {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_miiswordsman_sword63"), Hash40::new("tex_miiswordsman_sword64"), 3, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 0.2, 0.0, true, Hash40::new(""), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 2.0, 0.2);
        }
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_rapid_slash_arc"), Hash40::new("top"), 0.75, 11, 1, -20, -70, 90, 1.2, true);
            macros::LAST_EFFECT_SET_RATE(agent, 1.2);
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("haver"), -0.0, 12, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 2);
        }
    }
}

//Aerial Blurring Slashes 1 Effect
unsafe extern "C" fn ssbexo_miiswordsman_aerial_blurring_slashes_1_effect(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_BLURRING_SLASHES_CANCEL) {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_miiswordsman_sword65"), Hash40::new("tex_miiswordsman_sword66"), 3, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 0.2, 0.0, true, Hash40::new(""), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 2.0, 0.2);
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("haver"), -0.0, 12, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 2);
        }
    }
    else {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_miiswordsman_sword63"), Hash40::new("tex_miiswordsman_sword64"), 3, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 0.2, 0.0, true, Hash40::new(""), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 2.0, 0.2);
        }
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_rapid_slash_arc"), Hash40::new("top"), 0.75, 11, 1, -20, -70, 90, 1.2, true);
            macros::LAST_EFFECT_SET_RATE(agent, 1.2);
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("haver"), -0.0, 12, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 2);
        }
    }
}

//Blurring Slashes 1 Sound
unsafe extern "C" fn ssbexo_miiswordsman_blurring_slashes_1_sound(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_BLURRING_SLASHES_CANCEL) {
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_attack01"));
        }
    }
    else {
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_attack01"));
            macros::PLAY_SE(agent, Hash40::new("se_miiswordsman_special_c3_n02"));
        }   
    }
}

//Blurring Slashes 1 Expression
unsafe extern "C" fn ssbexo_miiswordsman_blurring_slashes_1_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_NONE), AttackDirectionAxis(*ATTACK_DIRECTION_NONE));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
}

//Blurring Slashes 2 ACMD
unsafe extern "C" fn ssbexo_miiswordsman_blurring_slashes_2_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 366, 100, 60, 0, 10.0, 0.0, 10.0, 9.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 366, 100, 60, 0, 10.0, 0.0, 10.0, 9.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Blurring Slashes 2 Effect
unsafe extern "C" fn ssbexo_miiswordsman_blurring_slashes_2_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_miiswordsman_sword63"), Hash40::new("tex_miiswordsman_sword64"), 3, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 0.2, 0.0, true, Hash40::new(""), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 2.0, 0.2);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 2);
    }
}

//Blurring Slashes 2 Sound
unsafe extern "C" fn ssbexo_miiswordsman_blurring_slashes_2_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_miiswordsman_special_c3_n02"));
    }
}

//Blurring Slashes 2 Expression
unsafe extern "C" fn ssbexo_miiswordsman_blurring_slashes_2_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_NONE), AttackDirectionAxis(*ATTACK_DIRECTION_NONE));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
}

//Airborne Assault ACMD
unsafe extern "C" fn ssbexo_miiswordsman_airborne_assault_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 3.7, 0.0, 12.0, 6.0, Some(0.0), Some(3.5), Some(6.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 3.7, 0.0, 12.0, 2.0, Some(0.0), Some(3.5), Some(2.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 3.7, 0.0, 12.0, -2.0, Some(0.0), Some(3.5), Some(-2.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

//Airborne Assault Hit ACMD
unsafe extern "C" fn ssbexo_miiswordsman_airborne_assault_hit_acmd(agent: &mut L2CAgentBase) {
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 50, 85, 0, 70, 7.0, 0.0, 4.0, 11.0, Some(0.0), Some(1.5), Some(11.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 50, 85, 0, 70, 7.0, 0.0, 4.0, 3.0, Some(0.0), Some(1.5), Some(3.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Rending Serrations 1 Forward ACMD
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_1s_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 4.0, 361, 30, 0, 20, 3.5, 0.0, 2.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 13.0, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE);
    }
}

//Rending Serrations 1 Forward Effect
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_1s_effect(agent: &mut L2CAgentBase) {
    let tex_sword = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD) as u64;
    let sword_add = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD) as u64;
    let sword_flare = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE) as u64;
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new_raw(tex_sword), Hash40::new_raw(sword_add), 6, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 10.8, 0.0, true, Hash40::new_raw(sword_flare), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
}

//Rending Serrations 1 Forward Sound
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_1s_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_attack01"));
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_miiswordsman_swing_s"));
    }
}

//Rending Serrations 1 Forward Expression
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_1s_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_Z));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashs"), 0);
    }
}

//Rending Serrations 2 Forward ACMD
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_2s_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 30, 0, 25, 6.5, 0.0, 9.0, 13.0, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 90, 30, 0, 25, 5.5, 0.0, 9.0, 16.0, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 361, 30, 0, 25, 5.5, 0.0, 9.0, 8.0, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 14.0, false);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE);
    }
}

//Rending Serrations 2 Forward Effect
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_2s_effect(agent: &mut L2CAgentBase) {
    let tex_sword = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD) as u64;
    let sword_add = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD) as u64;
    let sword_flare = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE) as u64;
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new_raw(tex_sword), Hash40::new_raw(sword_add), 6, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 10.8, 0.0, true, Hash40::new_raw(sword_flare), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
}

//Rending Serrations 2 Forward Sound
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_2s_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_attack01"));
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_miiswordsman_swing_s"));
    }
}

//Rending Serrations 2 Forward Expression
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_2s_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashs"), 0);
    }
}

//Rending Serrations 3 Forward ACMD
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_3s_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 5.0, 361, 30, 0, 20, 3.5, 0.0, 2.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 20.0, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE);
    }
}

//Rending Serrations 3 Forward Effect
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_3s_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.5, 6, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 0.0);
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
        macros::EFFECT_FLW_POS(agent, Hash40::new("miiswordsman_blastwind_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, -90, 0.45, true);
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
}

//Rending Serrations 3 Forward Sound
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_3s_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_attack01"));
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_miiswordsman_swing_s"));
    }
}

//Rending Serrations 3 Forward Expression
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_3s_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_Z));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
}

//Rending Serrations 4 Forward ACMD
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_4s_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 10.0, 361, 80, 0, 50, 3.5, 0.0, 2.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 1.0, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Rending Serrations 4 Forward Effect
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_4s_effect(agent: &mut L2CAgentBase) {
    let tex_sword = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD) as u64;
    let sword_add = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD) as u64;
    let sword_flare = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE) as u64;
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new_raw(tex_sword), Hash40::new_raw(sword_add), 6, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 10.8, 0.0, true, Hash40::new_raw(sword_flare), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
}

//Rending Serrations 4 Forward Sound
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_4s_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_attack01"));
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_miiswordsman_swing_s"));
    }
}

//Rending Serrations 4 Forward Expression
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_4s_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_Z));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

//Rending Serrations 1 Up ACMD
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_1hi_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 4.0, 70, 30, 0, 20, 3.5, 0.0, 2.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE);
    }
}

//Rending Serrations 1 Up Effect
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_1hi_effect(agent: &mut L2CAgentBase) {
    let tex_sword = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD) as u64;
    let sword_add = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD) as u64;
    let sword_flare = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE) as u64;
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new_raw(tex_sword), Hash40::new_raw(sword_add), 6, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 10.8, 0.0, true, Hash40::new_raw(sword_flare), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
}

//Rending Serrations 1 Up Sound
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_1hi_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_attack01"));
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_miiswordsman_swing_s"));
    }
}

//Rending Serrations 1 Up Expression
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_1hi_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_Z));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashs"), 0);
    }
}

//Rending Serrations 2 Up ACMD
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_2hi_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 4.0, 70, 30, 0, 20, 3.5, 0.0, 2.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 15.0, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE);
    }
}

//Rending Serrations 2 Up Effect
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_2hi_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.5, 6, -20, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.264, 0.47, 1.3);
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
        macros::EFFECT_FLW_POS(agent, Hash40::new("miiswordsman_blastwind_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, -90, 0.45, true);
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
}

//Rending Serrations 2 Up Sound
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_2hi_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_attack01"));
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_miiswordsman_swing_s"));
    }
}

//Rending Serrations 2 Up Expression
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_2hi_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_Z));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashs"), 0);
    }
}

//Rending Serrations 3 Up ACMD
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_3hi_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 5.0, 70, 100, 100, 0, 3.5, 0.0, 2.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 20.0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE);
    }
}

//Rending Serrations 3 Up Effect
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_3hi_effect(agent: &mut L2CAgentBase) {
    let tex_sword = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD) as u64;
    let sword_add = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD) as u64;
    let sword_flare = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE) as u64;
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new_raw(tex_sword), Hash40::new_raw(sword_add), 6, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 10.8, 0.0, true, Hash40::new_raw(sword_flare), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
}

//Rending Serrations 3 Up Sound
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_3hi_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_attack01"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_miiswordsman_swing_s"));
    }
}

//Rending Serrations 3 Up Expression
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_3hi_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_Z));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashs"), 0);
    }
}

//Rending Serrations 4 Up ACMD
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_4hi_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 8.0, 70, 100, 0, 70, 3.5, 0.0, 2.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 1.0, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Rending Serrations 4 Up Effect
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_4hi_effect(agent: &mut L2CAgentBase) {
    let tex_sword = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD) as u64;
    let sword_add = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD) as u64;
    let sword_flare = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE) as u64;
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new_raw(tex_sword), Hash40::new_raw(sword_add), 6, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 10.8, 0.0, true, Hash40::new_raw(sword_flare), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
}

//Rending Serrations 4 Up Sound
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_4hi_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_attack01"));
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_miiswordsman_swing_s"));
    }
}

//Rending Serrations 4 Up Expression
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_4hi_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_Z));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

//Rending Serrations 1 Down ACMD
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_1lw_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 4.0, 10, 30, 0, 20, 3.5, 0.0, 2.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE);
    }
}

//Rending Serrations 1 Down Effect
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_1lw_effect(agent: &mut L2CAgentBase) {
    let tex_sword = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD) as u64;
    let sword_add = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD) as u64;
    let sword_flare = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE) as u64;
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new_raw(tex_sword), Hash40::new_raw(sword_add), 6, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 10.8, 0.0, true, Hash40::new_raw(sword_flare), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
}

//Rending Serrations 1 Down Sound
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_1lw_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_attack01"));
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_miiswordsman_swing_s"));
    }
}

//Rending Serrations 1 Down Expression
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_1lw_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashs"), 0);
    }
}

//Rending Serrations 2 Down ACMD
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_2lw_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 4.0, 10, 30, 0, 20, 3.5, 0.0, 2.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE);
    }
}

//Rending Serrations 2 Down Effect
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_2lw_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.5, 6, -20, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.264, 0.47, 1.3);
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
        macros::EFFECT_FLW_POS(agent, Hash40::new("miiswordsman_blastwind_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, -90, 0.45, true);
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
}

//Rending Serrations 2 Down Sound
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_2lw_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_attack01"));
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_miiswordsman_swing_s"));
    }
}

//Rending Serrations 2 Down Expression
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_2lw_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_Z));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashs"), 0);
    }
}

//Rending Serrations 3 Down ACMD
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_3lw_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 5.0, 10, 30, 0, 20, 3.5, 0.0, 2.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_level(agent.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_2 as u8);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE);
    }
}

//Rending Serrations 3 Down Effect
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_3lw_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
}

//Rending Serrations 3 Down Sound
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_3lw_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_attack01"));
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_miiswordsman_swing_s"));
    }
}

//Rending Serrations 3 Down Expression
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_3lw_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_Z));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
}

//Rending Serrations 4 Down ACMD
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_4lw_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 12.0, 330, 80, 0, 20, 3.5, 0.0, 2.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 1.0, 1.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Rending Serrations 4 Down Effect
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_4lw_effect(agent: &mut L2CAgentBase) {
    let tex_sword = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD) as u64;
    let sword_add = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD) as u64;
    let sword_flare = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE) as u64;
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new_raw(tex_sword), Hash40::new_raw(sword_add), 6, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 10.8, 0.0, true, Hash40::new_raw(sword_flare), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
}

//Rending Serrations 4 Down Sound
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_4lw_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_attack01"));
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_miiswordsman_swing_s"));
    }
}

//Rending Serrations 4 Down Expression
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_4lw_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

//Rending Serrations 1 Back ACMD
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_1b_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 4.0, 10, 30, 0, 20, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(4.0), Some(0.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 15.0, false);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE);
    }
}

//Rending Serrations 1 Back Effect
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_1b_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, -2.0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
}

//Rending Serrations 1 Back Sound
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_1b_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_attack01"));
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_miiswordsman_swing_s"));
    }
}

//Rending Serrations 1 Back Expression
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_1b_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_Z));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashs"), 0);
    }
}

//Rending Serrations 2 Back ACMD
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_2b_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 4.0, 10, 30, 0, 20, 3.5, 0.0, 2.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 20.0, false);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE);
        macros::REVERSE_LR(agent);
    }
}

//Rending Serrations 2 Back Effect
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_2b_effect(agent: &mut L2CAgentBase) {
    let tex_sword = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD) as u64;
    let sword_add = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD) as u64;
    let sword_flare = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE) as u64;
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new_raw(tex_sword), Hash40::new_raw(sword_add), 6, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 10.8, 0.0, true, Hash40::new_raw(sword_flare), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
}

//Rending Serrations 2 Back Sound
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_2b_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_attack01"));
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_miiswordsman_swing_s"));
    }
}

//Rending Serrations 2 Back Expression
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_2b_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashs"), 0);
    }
}

//Rending Serrations 3 Back ACMD
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_3b_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 6.0, 10, 30, 0, 20, 3.5, 0.0, 2.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 22.0, false);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_RENDING_SERRATIONS_TRANSITION_ENABLE);
        macros::REVERSE_LR(agent);
    }
}

//Rending Serrations 3 Back Effect
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_3b_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 15, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
}

//Rending Serrations 3 Back Sound
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_3b_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_attack01"));
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_miiswordsman_swing_s"));
    }
}

//Rending Serrations 3 Back Expression
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_3b_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_Z));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
}

//Rending Serrations 4 Back ACMD
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_4b_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 15.0, 361, 80, 0, 50, 3.5, 0.0, 2.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 1.0, 1.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::REVERSE_LR(agent);
    }
}

//Rending Serrations 4 Back Effect
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_4b_effect(agent: &mut L2CAgentBase) {
    let tex_sword = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD) as u64;
    let sword_add = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD) as u64;
    let sword_flare = WorkModule::get_int64(agent.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE) as u64;
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new_raw(tex_sword), Hash40::new_raw(sword_add), 6, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 10.8, 0.0, true, Hash40::new_raw(sword_flare), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
}

//Rending Serrations 4 Back Sound
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_4b_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_attack01"));
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_miiswordsman_swing_s"));
    }
}

//Rending Serrations 4 Back Expression
unsafe extern "C" fn ssbexo_miiswordsman_rending_serrations_4b_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_Z));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

pub fn install() {
    Agent::new("miiswordsman")
    .game_acmd("game_specialn1", ssbexo_miiswordsman_guard_breaker_start_acmd, Priority::Low)
    .effect_acmd("effect_specialn1", ssbexo_miiswordsman_guard_breaker_start_effect, Priority::Low)
    .sound_acmd("sound_specialn1", ssbexo_miiswordsman_guard_breaker_start_sound, Priority::Low)
    .expression_acmd("expression_specialn1", ssbexo_miiswordsman_grounded_guard_breaker_start_expression, Priority::Low)
    .game_acmd("game_specialairn1", ssbexo_miiswordsman_guard_breaker_start_acmd, Priority::Low)
    .effect_acmd("effect_specialairn1", ssbexo_miiswordsman_guard_breaker_start_effect, Priority::Low)
    .sound_acmd("sound_specialairn1", ssbexo_miiswordsman_guard_breaker_start_sound, Priority::Low)
    .expression_acmd("expression_specialairn1", ssbexo_miiswordsman_aerial_guard_breaker_start_expression, Priority::Low)
    .effect_acmd("effect_specialn1loop", ssbexo_miiswordsman_guard_breaker_loop_effect, Priority::Low)
    .effect_acmd("effect_specialairn1loop", ssbexo_miiswordsman_guard_breaker_loop_effect, Priority::Low)
    .expression_acmd("expression_specialn1loop", ssbexo_miiswordsman_guard_breaker_loop_expression, Priority::Low)
    .expression_acmd("expression_specialairn1loop", ssbexo_miiswordsman_guard_breaker_loop_expression, Priority::Low)
    .game_acmd("game_specialn1attack", ssbexo_miiswordsman_guard_breaker_attack_acmd, Priority::Low)
    .game_acmd("game_specialn1attackhi", ssbexo_miiswordsman_guard_breaker_attack_hi_acmd, Priority::Low)
    .game_acmd("game_specialn1attacklw", ssbexo_miiswordsman_guard_breaker_attack_lw_acmd, Priority::Low)
    .game_acmd("game_specialairn1attack", ssbexo_miiswordsman_guard_breaker_attack_acmd, Priority::Low)
    .game_acmd("game_specialairn1attackhi", ssbexo_miiswordsman_guard_breaker_attack_hi_acmd, Priority::Low)
    .game_acmd("game_specialairn1attacklw", ssbexo_miiswordsman_guard_breaker_attack_lw_acmd, Priority::Low)
    .effect_acmd("effect_specialn1attack", ssbexo_miiswordsman_guard_breaker_attack_effect, Priority::Low)
    .effect_acmd("effect_specialn1attackhi", ssbexo_miiswordsman_guard_breaker_attack_effect, Priority::Low)
    .effect_acmd("effect_specialn1attacklw", ssbexo_miiswordsman_guard_breaker_attack_effect, Priority::Low)
    .effect_acmd("effect_specialairn1attack", ssbexo_miiswordsman_guard_breaker_attack_effect, Priority::Low)
    .effect_acmd("effect_specialairn1attackhi", ssbexo_miiswordsman_guard_breaker_attack_effect, Priority::Low)
    .effect_acmd("effect_specialairn1attacklw", ssbexo_miiswordsman_guard_breaker_attack_effect, Priority::Low)
    .sound_acmd("sound_specialn1attack", ssbexo_miiswordsman_guard_breaker_attack_sound, Priority::Low)
    .sound_acmd("sound_specialn1attackhi", ssbexo_miiswordsman_guard_breaker_attack_sound, Priority::Low)
    .sound_acmd("sound_specialn1attacklw", ssbexo_miiswordsman_guard_breaker_attack_sound, Priority::Low)
    .sound_acmd("sound_specialairn1attack", ssbexo_miiswordsman_guard_breaker_attack_sound, Priority::Low)
    .sound_acmd("sound_specialairn1attackhi", ssbexo_miiswordsman_guard_breaker_attack_sound, Priority::Low)
    .sound_acmd("sound_specialairn1attacklw", ssbexo_miiswordsman_guard_breaker_attack_sound, Priority::Low)
    .expression_acmd("expression_specialn1attack", ssbexo_miiswordsman_guard_breaker_attack_expression, Priority::Low)
    .expression_acmd("expression_specialn1attackhi", ssbexo_miiswordsman_guard_breaker_attack_expression, Priority::Low)
    .expression_acmd("expression_specialn1attacklw", ssbexo_miiswordsman_guard_breaker_attack_expression, Priority::Low)
    .expression_acmd("expression_specialairn1attack", ssbexo_miiswordsman_guard_breaker_attack_expression, Priority::Low)
    .expression_acmd("expression_specialairn1attackhi", ssbexo_miiswordsman_guard_breaker_attack_expression, Priority::Low)
    .expression_acmd("expression_specialairn1attacklw", ssbexo_miiswordsman_guard_breaker_attack_expression, Priority::Low)
    .game_acmd("game_specialn2", ssbexo_miiswordsman_shuriken_of_light_fire_acmd, Priority::Low)
    .game_acmd("game_specialairn2", ssbexo_miiswordsman_shuriken_of_light_fire_acmd, Priority::Low)
    .game_acmd("game_specialn31", ssbexo_miiswordsman_blurring_slashes_1_acmd, Priority::Low)
    .game_acmd("game_specialairn31", ssbexo_miiswordsman_blurring_slashes_1_acmd, Priority::Low)
    .effect_acmd("effect_specialn31", ssbexo_miiswordsman_grounded_blurring_slashes_1_effect, Priority::Low)
    .effect_acmd("effect_specialairn31", ssbexo_miiswordsman_aerial_blurring_slashes_1_effect, Priority::Low)
    .sound_acmd("sound_specialn31", ssbexo_miiswordsman_blurring_slashes_1_sound, Priority::Low)
    .sound_acmd("sound_specialairn31", ssbexo_miiswordsman_blurring_slashes_1_sound, Priority::Low)
    .expression_acmd("expression_specialn31", ssbexo_miiswordsman_blurring_slashes_1_expression, Priority::Low)
    .expression_acmd("expression_specialairn31", ssbexo_miiswordsman_blurring_slashes_1_expression, Priority::Low)
    .game_acmd("game_specialn32", ssbexo_miiswordsman_blurring_slashes_2_acmd, Priority::Low)
    .game_acmd("game_specialairn32", ssbexo_miiswordsman_blurring_slashes_2_acmd, Priority::Low)
    .effect_acmd("effect_specialn32", ssbexo_miiswordsman_blurring_slashes_2_effect, Priority::Low)
    .effect_acmd("effect_specialairn32", ssbexo_miiswordsman_blurring_slashes_2_effect, Priority::Low)
    .sound_acmd("sound_specialn32", ssbexo_miiswordsman_blurring_slashes_2_sound, Priority::Low)
    .sound_acmd("sound_specialairn32", ssbexo_miiswordsman_blurring_slashes_2_sound, Priority::Low)
    .expression_acmd("expression_specialn32", ssbexo_miiswordsman_blurring_slashes_2_expression, Priority::Low)
    .expression_acmd("expression_specialairn32", ssbexo_miiswordsman_blurring_slashes_2_expression, Priority::Low)
    .game_acmd("game_specials1", ssbexo_miiswordsman_airborne_assault_acmd, Priority::Low)
    .game_acmd("game_specialairs1", ssbexo_miiswordsman_airborne_assault_acmd, Priority::Low)
    .game_acmd("game_specials1hit", ssbexo_miiswordsman_airborne_assault_hit_acmd, Priority::Low)
    .game_acmd("game_specialairs1hit", ssbexo_miiswordsman_airborne_assault_hit_acmd, Priority::Low)
    .game_acmd("game_specials21s", ssbexo_miiswordsman_rending_serrations_1s_acmd, Priority::Low)
    .game_acmd("game_specialairs21s", ssbexo_miiswordsman_rending_serrations_1s_acmd, Priority::Low)
    .game_acmd("game_specials22s", ssbexo_miiswordsman_rending_serrations_2s_acmd, Priority::Low)
    .game_acmd("game_specialairs22s", ssbexo_miiswordsman_rending_serrations_2s_acmd, Priority::Low)
    .game_acmd("game_specials23s", ssbexo_miiswordsman_rending_serrations_3s_acmd, Priority::Low)
    .game_acmd("game_specialairs23s", ssbexo_miiswordsman_rending_serrations_3s_acmd, Priority::Low)
    .game_acmd("game_specials24s", ssbexo_miiswordsman_rending_serrations_4s_acmd, Priority::Low)
    .game_acmd("game_specialairs24s", ssbexo_miiswordsman_rending_serrations_4s_acmd, Priority::Low)
    .game_acmd("game_specials21hi", ssbexo_miiswordsman_rending_serrations_1hi_acmd, Priority::Low)
    .game_acmd("game_specialairs21hi", ssbexo_miiswordsman_rending_serrations_1hi_acmd, Priority::Low)
    .game_acmd("game_specials22hi", ssbexo_miiswordsman_rending_serrations_2hi_acmd, Priority::Low)
    .game_acmd("game_specialairs22hi", ssbexo_miiswordsman_rending_serrations_2hi_acmd, Priority::Low)
    .game_acmd("game_specials23hi", ssbexo_miiswordsman_rending_serrations_3hi_acmd, Priority::Low)
    .game_acmd("game_specialairs23hi", ssbexo_miiswordsman_rending_serrations_3hi_acmd, Priority::Low)
    .game_acmd("game_specials24hi", ssbexo_miiswordsman_rending_serrations_4hi_acmd, Priority::Low)
    .game_acmd("game_specialairs24hi", ssbexo_miiswordsman_rending_serrations_4hi_acmd, Priority::Low)
    .game_acmd("game_specials21lw", ssbexo_miiswordsman_rending_serrations_1lw_acmd, Priority::Low)
    .game_acmd("game_specialairs21lw", ssbexo_miiswordsman_rending_serrations_1lw_acmd, Priority::Low)
    .game_acmd("game_specials22lw", ssbexo_miiswordsman_rending_serrations_2lw_acmd, Priority::Low)
    .game_acmd("game_specialairs22lw", ssbexo_miiswordsman_rending_serrations_2lw_acmd, Priority::Low)
    .game_acmd("game_specials23lw", ssbexo_miiswordsman_rending_serrations_3lw_acmd, Priority::Low)
    .game_acmd("game_specialairs23lw", ssbexo_miiswordsman_rending_serrations_3lw_acmd, Priority::Low)
    .game_acmd("game_specials24lw", ssbexo_miiswordsman_rending_serrations_4lw_acmd, Priority::Low)
    .game_acmd("game_specialairs24lw", ssbexo_miiswordsman_rending_serrations_4lw_acmd, Priority::Low)
    .game_acmd("game_specials21b", ssbexo_miiswordsman_rending_serrations_1b_acmd, Priority::Low)
    .game_acmd("game_specialairs21b", ssbexo_miiswordsman_rending_serrations_1b_acmd, Priority::Low)
    .game_acmd("game_specials22b", ssbexo_miiswordsman_rending_serrations_2b_acmd, Priority::Low)
    .game_acmd("game_specialairs22b", ssbexo_miiswordsman_rending_serrations_2b_acmd, Priority::Low)
    .game_acmd("game_specials23b", ssbexo_miiswordsman_rending_serrations_3b_acmd, Priority::Low)
    .game_acmd("game_specialairs23b", ssbexo_miiswordsman_rending_serrations_3b_acmd, Priority::Low)
    .game_acmd("game_specials24b", ssbexo_miiswordsman_rending_serrations_4b_acmd, Priority::Low)
    .game_acmd("game_specialairs24b", ssbexo_miiswordsman_rending_serrations_4b_acmd, Priority::Low)
    .effect_acmd("effect_specials21s", ssbexo_miiswordsman_rending_serrations_1s_effect, Priority::Low)
    .effect_acmd("effect_specialairs21s", ssbexo_miiswordsman_rending_serrations_1s_effect, Priority::Low)
    .effect_acmd("effect_specials22s", ssbexo_miiswordsman_rending_serrations_2s_effect, Priority::Low)
    .effect_acmd("effect_specialairs22s", ssbexo_miiswordsman_rending_serrations_2s_effect, Priority::Low)
    .effect_acmd("effect_specials23s", ssbexo_miiswordsman_rending_serrations_3s_effect, Priority::Low)
    .effect_acmd("effect_specialairs23s", ssbexo_miiswordsman_rending_serrations_3s_effect, Priority::Low)
    .effect_acmd("effect_specials24s", ssbexo_miiswordsman_rending_serrations_4s_effect, Priority::Low)
    .effect_acmd("effect_specialairs24s", ssbexo_miiswordsman_rending_serrations_4s_effect, Priority::Low)
    .effect_acmd("effect_specials21hi", ssbexo_miiswordsman_rending_serrations_1hi_effect, Priority::Low)
    .effect_acmd("effect_specialairs21hi", ssbexo_miiswordsman_rending_serrations_1hi_effect, Priority::Low)
    .effect_acmd("effect_specials22hi", ssbexo_miiswordsman_rending_serrations_2hi_effect, Priority::Low)
    .effect_acmd("effect_specialairs22hi", ssbexo_miiswordsman_rending_serrations_2hi_effect, Priority::Low)
    .effect_acmd("effect_specials23hi", ssbexo_miiswordsman_rending_serrations_3hi_effect, Priority::Low)
    .effect_acmd("effect_specialairs23hi", ssbexo_miiswordsman_rending_serrations_3hi_effect, Priority::Low)
    .effect_acmd("effect_specials24hi", ssbexo_miiswordsman_rending_serrations_4hi_effect, Priority::Low)
    .effect_acmd("effect_specialairs24hi", ssbexo_miiswordsman_rending_serrations_4hi_effect, Priority::Low)
    .effect_acmd("effect_specials21lw", ssbexo_miiswordsman_rending_serrations_1lw_effect, Priority::Low)
    .effect_acmd("effect_specialairs21lw", ssbexo_miiswordsman_rending_serrations_1lw_effect, Priority::Low)
    .effect_acmd("effect_specials22lw", ssbexo_miiswordsman_rending_serrations_2lw_effect, Priority::Low)
    .effect_acmd("effect_specialairs22lw", ssbexo_miiswordsman_rending_serrations_2lw_effect, Priority::Low)
    .effect_acmd("effect_specials23lw", ssbexo_miiswordsman_rending_serrations_3lw_effect, Priority::Low)
    .effect_acmd("effect_specialairs23lw", ssbexo_miiswordsman_rending_serrations_3lw_effect, Priority::Low)
    .effect_acmd("effect_specials24lw", ssbexo_miiswordsman_rending_serrations_4lw_effect, Priority::Low)
    .effect_acmd("effect_specialairs24lw", ssbexo_miiswordsman_rending_serrations_4lw_effect, Priority::Low)
    .effect_acmd("effect_specials21b", ssbexo_miiswordsman_rending_serrations_1b_effect, Priority::Low)
    .effect_acmd("effect_specialairs21b", ssbexo_miiswordsman_rending_serrations_1b_effect, Priority::Low)
    .effect_acmd("effect_specials22b", ssbexo_miiswordsman_rending_serrations_2b_effect, Priority::Low)
    .effect_acmd("effect_specialairs22b", ssbexo_miiswordsman_rending_serrations_2b_effect, Priority::Low)
    .effect_acmd("effect_specials23b", ssbexo_miiswordsman_rending_serrations_3b_effect, Priority::Low)
    .effect_acmd("effect_specialairs23b", ssbexo_miiswordsman_rending_serrations_3b_effect, Priority::Low)
    .effect_acmd("effect_specials24b", ssbexo_miiswordsman_rending_serrations_4b_effect, Priority::Low)
    .effect_acmd("effect_specialairs24b", ssbexo_miiswordsman_rending_serrations_4b_effect, Priority::Low)
    .sound_acmd("sound_specials21s", ssbexo_miiswordsman_rending_serrations_1s_sound, Priority::Low)
    .sound_acmd("sound_specialairs21s", ssbexo_miiswordsman_rending_serrations_1s_sound, Priority::Low)
    .sound_acmd("sound_specials22s", ssbexo_miiswordsman_rending_serrations_2s_sound, Priority::Low)
    .sound_acmd("sound_specialairs22s", ssbexo_miiswordsman_rending_serrations_2s_sound, Priority::Low)
    .sound_acmd("sound_specials23s", ssbexo_miiswordsman_rending_serrations_3s_sound, Priority::Low)
    .sound_acmd("sound_specialairs23s", ssbexo_miiswordsman_rending_serrations_3s_sound, Priority::Low)
    .sound_acmd("sound_specials24s", ssbexo_miiswordsman_rending_serrations_4s_sound, Priority::Low)
    .sound_acmd("sound_specialairs24s", ssbexo_miiswordsman_rending_serrations_4s_sound, Priority::Low)
    .sound_acmd("sound_specials21hi", ssbexo_miiswordsman_rending_serrations_1hi_sound, Priority::Low)
    .sound_acmd("sound_specialairs21hi", ssbexo_miiswordsman_rending_serrations_1hi_sound, Priority::Low)
    .sound_acmd("sound_specials22hi", ssbexo_miiswordsman_rending_serrations_2hi_sound, Priority::Low)
    .sound_acmd("sound_specialairs22hi", ssbexo_miiswordsman_rending_serrations_2hi_sound, Priority::Low)
    .sound_acmd("sound_specials23hi", ssbexo_miiswordsman_rending_serrations_3hi_sound, Priority::Low)
    .sound_acmd("sound_specialairs23hi", ssbexo_miiswordsman_rending_serrations_3hi_sound, Priority::Low)
    .sound_acmd("sound_specials24hi", ssbexo_miiswordsman_rending_serrations_4hi_sound, Priority::Low)
    .sound_acmd("sound_specialairs24hi", ssbexo_miiswordsman_rending_serrations_4hi_sound, Priority::Low)
    .sound_acmd("sound_specials21lw", ssbexo_miiswordsman_rending_serrations_1lw_sound, Priority::Low)
    .sound_acmd("sound_specialairs21lw", ssbexo_miiswordsman_rending_serrations_1lw_sound, Priority::Low)
    .sound_acmd("sound_specials22lw", ssbexo_miiswordsman_rending_serrations_2lw_sound, Priority::Low)
    .sound_acmd("sound_specialairs22lw", ssbexo_miiswordsman_rending_serrations_2lw_sound, Priority::Low)
    .sound_acmd("sound_specials23lw", ssbexo_miiswordsman_rending_serrations_3lw_sound, Priority::Low)
    .sound_acmd("sound_specialairs23lw", ssbexo_miiswordsman_rending_serrations_3lw_sound, Priority::Low)
    .sound_acmd("sound_specials24lw", ssbexo_miiswordsman_rending_serrations_4lw_sound, Priority::Low)
    .sound_acmd("sound_specialairs24lw", ssbexo_miiswordsman_rending_serrations_4lw_sound, Priority::Low)
    .sound_acmd("sound_specials21b", ssbexo_miiswordsman_rending_serrations_1b_sound, Priority::Low)
    .sound_acmd("sound_specialairs21b", ssbexo_miiswordsman_rending_serrations_1b_sound, Priority::Low)
    .sound_acmd("sound_specials22b", ssbexo_miiswordsman_rending_serrations_2b_sound, Priority::Low)
    .sound_acmd("sound_specialairs22b", ssbexo_miiswordsman_rending_serrations_2b_sound, Priority::Low)
    .sound_acmd("sound_specials23b", ssbexo_miiswordsman_rending_serrations_3b_sound, Priority::Low)
    .sound_acmd("sound_specialairs23b", ssbexo_miiswordsman_rending_serrations_3b_sound, Priority::Low)
    .sound_acmd("sound_specials24b", ssbexo_miiswordsman_rending_serrations_4b_sound, Priority::Low)
    .sound_acmd("sound_specialairs24b", ssbexo_miiswordsman_rending_serrations_4b_sound, Priority::Low)
    .expression_acmd("expression_specials21s", ssbexo_miiswordsman_rending_serrations_1s_expression, Priority::Low)
    .expression_acmd("expression_specialairs21s", ssbexo_miiswordsman_rending_serrations_1s_expression, Priority::Low)
    .expression_acmd("expression_specials22s", ssbexo_miiswordsman_rending_serrations_2s_expression, Priority::Low)
    .expression_acmd("expression_specialairs22s", ssbexo_miiswordsman_rending_serrations_2s_expression, Priority::Low)
    .expression_acmd("expression_specials23s", ssbexo_miiswordsman_rending_serrations_3s_expression, Priority::Low)
    .expression_acmd("expression_specialairs23s", ssbexo_miiswordsman_rending_serrations_3s_expression, Priority::Low)
    .expression_acmd("expression_specials24s", ssbexo_miiswordsman_rending_serrations_4s_expression, Priority::Low)
    .expression_acmd("expression_specialairs24s", ssbexo_miiswordsman_rending_serrations_4s_expression, Priority::Low)
    .expression_acmd("expression_specials21hi", ssbexo_miiswordsman_rending_serrations_1hi_expression, Priority::Low)
    .expression_acmd("expression_specialairs21hi", ssbexo_miiswordsman_rending_serrations_1hi_expression, Priority::Low)
    .expression_acmd("expression_specials22hi", ssbexo_miiswordsman_rending_serrations_2hi_expression, Priority::Low)
    .expression_acmd("expression_specialairs22hi", ssbexo_miiswordsman_rending_serrations_2hi_expression, Priority::Low)
    .expression_acmd("expression_specials23hi", ssbexo_miiswordsman_rending_serrations_3hi_expression, Priority::Low)
    .expression_acmd("expression_specialairs23hi", ssbexo_miiswordsman_rending_serrations_3hi_expression, Priority::Low)
    .expression_acmd("expression_specials24hi", ssbexo_miiswordsman_rending_serrations_4hi_expression, Priority::Low)
    .expression_acmd("expression_specialairs24hi", ssbexo_miiswordsman_rending_serrations_4hi_expression, Priority::Low)
    .expression_acmd("expression_specials21lw", ssbexo_miiswordsman_rending_serrations_1lw_expression, Priority::Low)
    .expression_acmd("expression_specialairs21lw", ssbexo_miiswordsman_rending_serrations_1lw_expression, Priority::Low)
    .expression_acmd("expression_specials22lw", ssbexo_miiswordsman_rending_serrations_2lw_expression, Priority::Low)
    .expression_acmd("expression_specialairs22lw", ssbexo_miiswordsman_rending_serrations_2lw_expression, Priority::Low)
    .expression_acmd("expression_specials23lw", ssbexo_miiswordsman_rending_serrations_3lw_expression, Priority::Low)
    .expression_acmd("expression_specialairs23lw", ssbexo_miiswordsman_rending_serrations_3lw_expression, Priority::Low)
    .expression_acmd("expression_specials24lw", ssbexo_miiswordsman_rending_serrations_4lw_expression, Priority::Low)
    .expression_acmd("expression_specialairs24lw", ssbexo_miiswordsman_rending_serrations_4lw_expression, Priority::Low)
    .expression_acmd("expression_specials21b", ssbexo_miiswordsman_rending_serrations_1b_expression, Priority::Low)
    .expression_acmd("expression_specialairs21b", ssbexo_miiswordsman_rending_serrations_1b_expression, Priority::Low)
    .expression_acmd("expression_specials22b", ssbexo_miiswordsman_rending_serrations_2b_expression, Priority::Low)
    .expression_acmd("expression_specialairs22b", ssbexo_miiswordsman_rending_serrations_2b_expression, Priority::Low)
    .expression_acmd("expression_specials23b", ssbexo_miiswordsman_rending_serrations_3b_expression, Priority::Low)
    .expression_acmd("expression_specialairs23b", ssbexo_miiswordsman_rending_serrations_3b_expression, Priority::Low)
    .expression_acmd("expression_specials24b", ssbexo_miiswordsman_rending_serrations_4b_expression, Priority::Low)
    .expression_acmd("expression_specialairs24b", ssbexo_miiswordsman_rending_serrations_4b_expression, Priority::Low)
    .install()
    ;
    Agent::new("miiswordsman_lightshuriken")
    .game_acmd("game_fly", ssbexo_miiswordsman_shuriken_of_light_fly_acmd, Priority::Low)
    .install()
    ;
}