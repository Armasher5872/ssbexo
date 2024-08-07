use super::*;

//Jab 1 ACMD
unsafe extern "C" fn ssbexo_ness_jab_1_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 25, 0, 30, 1.6, 0.0, 5.0, 4.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 25, 0, 30, 1.8, 0.0, 5.0, 6.5, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 180, 15, 0, 20, 2.2, 0.0, 5.0, 9.2, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 361, 15, 0, 20, 2.2, 0.0, 5.0, 9.2, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 180, 25, 0, 20, 6.0, 0.0, 5.0, 13.2, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_MIDDLE), false);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 25, 0, 30, 1.6, 0.0, 5.0, 4.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 25, 0, 30, 1.8, 0.0, 5.0, 6.5, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 180, 15, 0, 20, 2.2, 0.0, 5.0, 9.2, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 361, 15, 0, 20, 2.2, 0.0, 5.0, 9.2, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 180, 25, 0, 20, 4.0, 0.0, 5.0, 13.2, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_MIDDLE), false);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
}

//Jab 1 Effect
unsafe extern "C" fn ssbexo_ness_jab_1_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0.5, 4.9, -6.5, 0, -2, 0, 0.95, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 3.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 5, 10.5, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, true, 0.6);
            macros::EFFECT(agent, Hash40::new("ness_psi_atk"), Hash40::new("top"), 0, 5, 12.0, 0, -90, 0, 1.1, 0, 0, 0, 0, 0, 360, true);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 5, 10.5, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, true, 0.6);
            macros::EFFECT(agent, Hash40::new("ness_psi_atk"), Hash40::new("top"), 0, 5, 12.0, 0, -90, 0, 0.9, 0, 0, 0, 0, 0, 360, true);
        }
    }
}

//Jab 2 ACMD
unsafe extern "C" fn ssbexo_ness_jab_2_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 361, 25, 0, 30, 2.2, 0.0, 5.5, 5.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 361, 25, 0, 30, 2.2, 0.0, 5.5, 8.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.5, 361, 20, 0, 20, 3.0, 0.0, 5.5, 13.1, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 1.5, 180, 30, 0, 20, 6.0, 0.0, 5.5, 15.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 361, 25, 0, 30, 2.2, 0.0, 5.5, 5.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 361, 25, 0, 30, 2.2, 0.0, 5.5, 8.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.5, 361, 20, 0, 20, 3.0, 0.0, 5.5, 13.1, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 1.5, 180, 30, 0, 20, 4.0, 0.0, 5.5, 15.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Jab 2 Effect
unsafe extern "C" fn ssbexo_ness_jab_2_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 5.5, -4.5, 0, 7, 0, 0.95, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 3.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 5.5, 12.5, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, true, 0.5);
            macros::EFFECT(agent, Hash40::new("ness_psi_atk"), Hash40::new("top"), 0, 5.5, 14.5, 0, -90, 0, 1.2, 0, 0, 0, 0, 0, 360, true);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 5.5, 12.5, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, true, 0.5);
            macros::EFFECT(agent, Hash40::new("ness_psi_atk"), Hash40::new("top"), 0, 5.5, 14.5, 0, -90, 0, 1.0, 0, 0, 0, 0, 0, 360, true);
        }
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_attack_line"), true, true);
    }
}

//Jab 3 ACMD
unsafe extern "C" fn ssbexo_ness_jab_3_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 100, 0, 40, 3.3, 0.0, 6.0, 5.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 361, 100, 0, 40, 3.8, 0.0, 6.0, 11.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 361, 100, 0, 40, 6.3, 0.0, 6.0, 15.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 100, 0, 40, 3.3, 0.0, 6.0, 5.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 361, 100, 0, 40, 3.8, 0.0, 6.0, 11.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 361, 100, 0, 40, 4.3, 0.0, 6.0, 15.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Jab 3 Effect
unsafe extern "C" fn ssbexo_ness_jab_3_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), -3, 5, -7.5, -4, 9, 0, 1.1, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 7.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 6, 12, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, true, 0.5);
            macros::EFFECT(agent, Hash40::new("ness_psi_atk"), Hash40::new("top"), 0, 6.0, 15.0, 0, -90, 0, 1.3, 0, 0, 0, 0, 0, 360, true);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 6, 12, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, true, 0.5);
            macros::EFFECT(agent, Hash40::new("ness_psi_atk"), Hash40::new("top"), 0, 6.0, 15.0, 0, -90, 0, 1.1, 0, 0, 0, 0, 0, 360, true);
        }
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_attack_line"), true, true);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_ness_dash_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 0, 100, 57, 0, 8.0, 0.0, 5.5, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 80, 70, 0, 60, 8.0, 0.0, 5.5, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 45, 100, 0, 80, 8.0, 0.0, 4.3, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 0, 100, 57, 0, 6.0, 0.0, 5.5, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 80, 70, 0, 60, 6.0, 0.0, 5.5, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 45, 100, 0, 80, 6.0, 0.0, 4.3, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 4.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 20, 100, 70, 0, 8.0, 0.0, 4.8, 16.0, Some(0.0), Some(4.8), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 20, 100, 70, 0, 6.0, 0.0, 4.8, 16.0, Some(0.0), Some(4.8), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 5.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 100, 113, 0, 85, 8.0, 0.0, 4.8, 21.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 100, 113, 0, 85, 6.0, 0.0, 4.8, 21.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Dash Attack Effect
unsafe extern "C" fn ssbexo_ness_dash_attack_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("ness_psi_hold"), Hash40::new("handr"), 1, 1, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 8.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("ness_psi_hold"), true, true);
            macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(agent, Hash40::new("ness_psi_rush"), Hash40::new("top"), 0, 5, 9, 0, 0, 0, 1.7, 2, 2, 2, 0, 0, 0, true);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("ness_psi_hold"), true, true);
            macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(agent, Hash40::new("ness_psi_rush"), Hash40::new("top"), 0, 5, 9, 0, 0, 0, 1.5, 2, 2, 2, 0, 0, 0, true);
        }
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("ness_psi_rush"), Hash40::new("top"), 0, 5, 16, 0, 0, 0, 1.6, 2, 2, 2, 0, 0, 0, true);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("ness_psi_rush"), Hash40::new("top"), 0, 5, 16, 0, 0, 0, 1.4, 2, 2, 2, 0, 0, 0, true);
        }
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 22.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("ness_psi_rush"), Hash40::new("top"), 0, 4.8, 21, 0, 0, 0, 1.8, 2, 2, 2, 0, 0, 0, true);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("ness_psi_rush"), Hash40::new("top"), 0, 4.8, 21, 0, 0, 0, 1.6, 2, 2, 2, 0, 0, 0, true);
        }
    }
}

pub fn install() {
    Agent::new("ness")
    .game_acmd("game_attack11", ssbexo_ness_jab_1_acmd, Priority::Low)
    .effect_acmd("effect_attack11", ssbexo_ness_jab_1_effect, Priority::Low)
    .game_acmd("game_attack12", ssbexo_ness_jab_2_acmd, Priority::Low)
    .effect_acmd("effect_attack12", ssbexo_ness_jab_2_effect, Priority::Low)
    .game_acmd("game_attack13", ssbexo_ness_jab_3_acmd, Priority::Low)
    .effect_acmd("effect_attack13", ssbexo_ness_jab_3_effect, Priority::Low)
    .game_acmd("game_attackdash", ssbexo_ness_dash_attack_acmd, Priority::Low)
    .effect_acmd("effect_attackdash", ssbexo_ness_dash_attack_effect, Priority::Low)
    .install()
    ;
}