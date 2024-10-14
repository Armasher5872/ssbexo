use super::*;

//Neutral Special Hi ACMD
unsafe extern "C" fn ssbexo_rockman_neutral_special_hi_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_ALREADY_EXIST_METALBLADE) {
        if macros::is_excute(agent) {
            ArticleModule::generate_article_enable(agent.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_METALBLADE, false, -1);
        }
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_STICK_CHECK_FRAME_END);
    }
    frame(agent.lua_state_agent, 16.0);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_ALREADY_EXIST_METALBLADE) {
        if macros::is_excute(agent) {
            ArticleModule::shoot_exist(agent.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_METALBLADE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
    }
}

//Grounded Neutral Special Hi Effect
unsafe extern "C" fn ssbexo_rockman_grounded_neutral_special_hi_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_ALREADY_EXIST_METALBLADE) {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("rockman_metalblade_start"), Hash40::new("handl"), 0, 0, 0, 0, -0.0, 0, 0.8, true);
        }
    }
    frame(agent.lua_state_agent, 8.0);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("rockman_metalblade_cutter"), Hash40::new("havel"), 0, 0, 0, 0, -0.0, -90, 1, true);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("rockman_metalblade_cutter"), Hash40::new("havel"), 0, 0, 0, 0, -0.0, 90, 1, true);
        }
    }
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("havel"), 0, -2, 4, 0, -0.0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("rockman_leafshield_arc"), Hash40::new("top"), -1, 10, 2.5, 0, -60, -105, 0.87, false, 0.05);
        macros::LAST_EFFECT_SET_RATE(agent, 1.7);
    }
    frame(agent.lua_state_agent, 16.0);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_ALREADY_EXIST_METALBLADE) {
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

//Aerial Neutral Special Hi Effect
unsafe extern "C" fn ssbexo_rockman_aerial_neutral_special_hi_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_ALREADY_EXIST_METALBLADE) {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("rockman_metalblade_start"), Hash40::new("handl"), 0, 0, 0, 0, -0.0, 0, 0.8, true);
        }
    }
    frame(agent.lua_state_agent, 8.0);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("rockman_metalblade_cutter"), Hash40::new("havel"), 0, 0, 0, 0, -0.0, -90, 1, true);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("rockman_metalblade_cutter"), Hash40::new("havel"), 0, 0, 0, 0, -0.0, 90, 1, true);
        }
    }
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("havel"), 0, -2, 4, 0, -0.0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("rockman_leafshield_arc"), Hash40::new("top"), -1, 10, 2.5, 0, -60, -105, 0.87, false, 0.05);
        macros::LAST_EFFECT_SET_RATE(agent, 1.7);
    }
}

//Neutral Special Hi Expression
unsafe extern "C" fn ssbexo_rockman_neutral_special_hi_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_LEFT, *FIGHTER_ROCKMAN_ARMFORM_HAND, 0);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_RIGHT, *FIGHTER_ROCKMAN_ARMFORM_HAND, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Grounded Down Special Effect
unsafe extern "C" fn ssbexo_rockman_grounded_down_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_v"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Down Special Effect
unsafe extern "C" fn ssbexo_rockman_aerial_down_special_effect(_agent: &mut L2CAgentBase) {}

//Leafshield Start ACMD
unsafe extern "C" fn ssbexo_rockman_leafshield_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("leafshield1"), 8.0, 55, 50, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("leafshield2"), 8.0, 55, 50, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 2, 0, Hash40::new("leafshield3"), 8.0, 55, 50, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 3, 0, Hash40::new("leafshield4"), 8.0, 55, 50, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
}

//Leafshield Start Effect
unsafe extern "C" fn ssbexo_rockman_leafshield_start_effect(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("leafshield1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
            smash::app::sv_animcmd::EFFECT_FOLLOW_RND(agent.lua_state_agent);
            agent.pop_lua_stack(1);
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("leafshield2"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
            smash::app::sv_animcmd::EFFECT_FOLLOW_RND(agent.lua_state_agent);
            agent.pop_lua_stack(1);
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("leafshield3"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
            smash::app::sv_animcmd::EFFECT_FOLLOW_RND(agent.lua_state_agent);
            agent.pop_lua_stack(1);
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("leafshield4"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
            smash::app::sv_animcmd::EFFECT_FOLLOW_RND(agent.lua_state_agent);
            agent.pop_lua_stack(1);
        }
        wait(agent.lua_state_agent, 10.0);
    }
}

//Leafshield Shield Effect
unsafe extern "C" fn ssbexo_rockman_leafshield_shield_effect(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("leafshield1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
            smash::app::sv_animcmd::EFFECT_FOLLOW_RND(agent.lua_state_agent);
            agent.pop_lua_stack(1);
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("leafshield2"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
            smash::app::sv_animcmd::EFFECT_FOLLOW_RND(agent.lua_state_agent);
            agent.pop_lua_stack(1);
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("leafshield3"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
            smash::app::sv_animcmd::EFFECT_FOLLOW_RND(agent.lua_state_agent);
            agent.pop_lua_stack(1);
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("leafshield4"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
            smash::app::sv_animcmd::EFFECT_FOLLOW_RND(agent.lua_state_agent);
            agent.pop_lua_stack(1);
        }
        wait(agent.lua_state_agent, 10.0);
    }
}

//Leafshield Fly Effect
unsafe extern "C" fn ssbexo_rockman_leafshield_fly_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("rockman_leafshield_fly"), Hash40::new("top"), 0, 0, 0, 0, -0.0, 0, 1, true);
    }
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("leafshield1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
            smash::app::sv_animcmd::EFFECT_FOLLOW_RND(agent.lua_state_agent);
            agent.pop_lua_stack(1);
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("leafshield2"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
            smash::app::sv_animcmd::EFFECT_FOLLOW_RND(agent.lua_state_agent);
            agent.pop_lua_stack(1);
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("leafshield3"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
            smash::app::sv_animcmd::EFFECT_FOLLOW_RND(agent.lua_state_agent);
            agent.pop_lua_stack(1);
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("leafshield4"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
            smash::app::sv_animcmd::EFFECT_FOLLOW_RND(agent.lua_state_agent);
            agent.pop_lua_stack(1);
        }
        wait(agent.lua_state_agent, 10.0);
    }
}

pub fn install() {
    Agent::new("rockman")
    .game_acmd("game_specialnhi", ssbexo_rockman_neutral_special_hi_acmd, Priority::Low)
    .game_acmd("game_specialairnhi", ssbexo_rockman_neutral_special_hi_acmd, Priority::Low)
    .effect_acmd("effect_specialnhi", ssbexo_rockman_grounded_neutral_special_hi_effect, Priority::Low)
    .effect_acmd("effect_specialairnhi", ssbexo_rockman_aerial_neutral_special_hi_effect, Priority::Low)
    .expression_acmd("expression_specialnhi", ssbexo_rockman_neutral_special_hi_expression, Priority::Low)
    .expression_acmd("expression_specialairnhi", ssbexo_rockman_neutral_special_hi_expression, Priority::Low)
    .effect_acmd("effect_speciallw", ssbexo_rockman_grounded_down_special_effect, Priority::Low)
    .effect_acmd("effect_specialairlw", ssbexo_rockman_aerial_down_special_effect, Priority::Low)
    .install()
    ;
    Agent::new("rockman_leafshield")
    .game_acmd("game_start", ssbexo_rockman_leafshield_acmd, Priority::Low)
    .game_acmd("game_startreverse", ssbexo_rockman_leafshield_acmd, Priority::Low)
    .game_acmd("game_shield", ssbexo_rockman_leafshield_acmd, Priority::Low)
    .game_acmd("game_shieldreverse", ssbexo_rockman_leafshield_acmd, Priority::Low)
    .game_acmd("game_fly", ssbexo_rockman_leafshield_acmd, Priority::Low)
    .game_acmd("game_flyreverse", ssbexo_rockman_leafshield_acmd, Priority::Low)
    .effect_acmd("effect_start", ssbexo_rockman_leafshield_start_effect, Priority::Low)
    .effect_acmd("effect_startreverse", ssbexo_rockman_leafshield_start_effect, Priority::Low)
    .effect_acmd("effect_shield", ssbexo_rockman_leafshield_shield_effect, Priority::Low)
    .effect_acmd("effect_shieldreverse", ssbexo_rockman_leafshield_shield_effect, Priority::Low)
    .effect_acmd("effect_fly", ssbexo_rockman_leafshield_fly_effect, Priority::Low)
    .effect_acmd("effect_flyreverse", ssbexo_rockman_leafshield_fly_effect, Priority::Low)
    .install()
    ;
}