use super::*;

//Forward Tilt ACMD
unsafe extern "C" fn ssbexo_ness_forward_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("legl"), 10.0, 361, 100, 0, 35, 3.6, 0.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("kneel"), 10.0, 361, 100, 0, 35, 4.8, 2.7, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 361, 100, 0, 35, 7.5, 0.0, 6.0, 15.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        }
    }
    else {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("legl"), 10.0, 361, 100, 0, 35, 3.6, 0.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("kneel"), 10.0, 361, 100, 0, 35, 4.8, 2.7, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 361, 100, 0, 35, 5.5, 0.0, 6.0, 15.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    wait(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Tilt Effect
unsafe extern "C" fn ssbexo_ness_forward_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 1, 5.3, 2.5, 0, -60, 15, 0.9, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 7.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("ness_psi_atk"), Hash40::new("top"), 0, 6.0, 15.0, 0, -90, 0, 1.3, 0, 0, 0, 0, 0, 360, true);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("ness_psi_atk"), Hash40::new("top"), 0, 6.0, 15.0, 0, -90, 0, 1.1, 0, 0, 0, 0, 0, 360, true);
        }
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Forward Tilt Hi ACMD
unsafe extern "C" fn ssbexo_ness_forward_tilt_hi_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("legl"), 10.0, 361, 100, 0, 35, 3.6, 0.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("kneel"), 10.0, 361, 100, 0, 35, 4.8, 2.7, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 361, 100, 0, 35, 7.5, 0.0, 11.0, 15.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        }
    }
    else {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("legl"), 10.0, 361, 100, 0, 35, 3.6, 0.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("kneel"), 10.0, 361, 100, 0, 35, 4.8, 2.7, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 361, 100, 0, 35, 5.5, 0.0, 11.0, 15.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    wait(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Tilt Hi Effect
unsafe extern "C" fn ssbexo_ness_forward_tilt_hi_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 1, 6.7, 1.8, -25, -45, 45, 0.9, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 7.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("ness_psi_atk"), Hash40::new("top"), 0, 11.0, 15.0, 0, -90, 0, 1.3, 0, 0, 0, 0, 0, 360, true);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("ness_psi_atk"), Hash40::new("top"), 0, 11.0, 15.0, 0, -90, 0, 1.1, 0, 0, 0, 0, 0, 360, true);
        }
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Forward Tilt Lw ACMD
unsafe extern "C" fn ssbexo_ness_forward_tilt_lw_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("legl"), 10.0, 361, 100, 0, 35, 3.6, 0.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("kneel"), 10.0, 361, 100, 0, 35, 4.8, 2.7, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 361, 100, 0, 35, 7.5, 0.0, 3.0, 15.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        }
    }
    else {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("legl"), 10.0, 361, 100, 0, 35, 3.6, 0.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("kneel"), 10.0, 361, 100, 0, 35, 4.8, 2.7, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 361, 100, 0, 35, 5.5, 0.0, 3.0, 15.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    wait(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Tilt Lw Effect
unsafe extern "C" fn ssbexo_ness_forward_tilt_lw_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 2, 4, 1.8, 15, -60, 3, 0.9, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 7.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("ness_psi_atk"), Hash40::new("top"), 0, 3.0, 15.0, 0, -90, 0, 1.3, 0, 0, 0, 0, 0, 360, true);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("ness_psi_atk"), Hash40::new("top"), 0, 3.0, 15.0, 0, -90, 0, 1.1, 0, 0, 0, 0, 0, 360, true);
        }
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Up Tilt ACMD
unsafe extern "C" fn ssbexo_ness_up_tilt_acmd(agent: &mut L2CAgentBase) {
    FT_MOTION_RATE(agent, 0.57);
    frame(agent.lua_state_agent, 7.0);
    FT_MOTION_RATE(agent, 1.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 96, 113, 0, 46, 8.5, 0.0, 17.0, 0.0, Some(0.0), Some(22.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 96, 113, 0, 46, 4.0, 0.0, 9.0, 1.5, Some(0.0), Some(9.0), Some(-1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PSI);
        }
    }
    else {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 96, 113, 0, 46, 6.5, 0.0, 17.0, 0.0, Some(0.0), Some(22.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 96, 113, 0, 46, 4.0, 0.0, 9.0, 1.5, Some(0.0), Some(9.0), Some(-1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PSI);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Up Tilt Effect
unsafe extern "C" fn ssbexo_ness_up_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 2, 9, 2, -90, 0, 0, 0.55, true);
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 9, -2, -90, 0, 0, 0.55, true);
            EFFECT(agent, Hash40::new("ness_psi_atk"), Hash40::new("top"), 0, 19, 0, 0, -90, 0, 2.0, 0, 0, 0, 0, 0, 360, true);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 2, 9, 2, -90, 0, 0, 0.55, true);
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 9, -2, -90, 0, 0, 0.55, true);
            EFFECT(agent, Hash40::new("ness_psi_atk"), Hash40::new("top"), 0, 19, 0, 0, -90, 0, 1.8, 0, 0, 0, 0, 0, 360, true);
        }
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

//Down Tilt ACMD
unsafe extern "C" fn ssbexo_ness_down_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("kneel"), 4.5, 361, 50, 0, 20, 2.5, 1.8, 0.0, 0.0, Some(-3.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("kneel"), 3.0, 361, 50, 0, 20, 2.5, 5.0, 0.0, 0.0, Some(-3.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 361, 50, 0, 20, 5.5, 0.0, 3.0, 12.2, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    else {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("kneel"), 4.5, 361, 50, 0, 20, 2.5, 1.8, 0.0, 0.0, Some(-3.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("kneel"), 3.0, 361, 50, 0, 20, 2.5, 5.0, 0.0, 0.0, Some(-3.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 361, 50, 0, 20, 3.5, 0.0, 3.0, 12.2, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_INPUT);
    }
}

//Down Tilt Effect
unsafe extern "C" fn ssbexo_ness_down_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 6, 1, -2.5, 0, -25, 0, 0.8, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 3.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if is_excute(agent) {
            EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 1.5, 9.2, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 360, true, 0.6);
            EFFECT(agent, Hash40::new("ness_psi_atk"), Hash40::new("top"), 0, 1.5, 12.2, 0, -90, 0, 0.9, 0, 0, 0, 0, 0, 360, true);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 1.5, 9.2, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 360, true, 0.6);
            EFFECT(agent, Hash40::new("ness_psi_atk"), Hash40::new("top"), 0, 1.5, 12.2, 0, -90, 0, 0.7, 0, 0, 0, 0, 0, 360, true);
        }
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    Agent::new("ness")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attacks3", ssbexo_ness_forward_tilt_acmd, Low)
    .effect_acmd("effect_attacks3", ssbexo_ness_forward_tilt_effect, Low)
    .game_acmd("game_attacks3hi", ssbexo_ness_forward_tilt_hi_acmd, Low)
    .effect_acmd("effect_attacks3hi", ssbexo_ness_forward_tilt_hi_effect, Low)
    .game_acmd("game_attacks3lw", ssbexo_ness_forward_tilt_lw_acmd, Low)
    .effect_acmd("effect_attacks3lw", ssbexo_ness_forward_tilt_lw_effect, Low)
    .game_acmd("game_attackhi3", ssbexo_ness_up_tilt_acmd, Low)
    .effect_acmd("effect_attackhi3", ssbexo_ness_up_tilt_effect, Low)
    .game_acmd("game_attacklw3", ssbexo_ness_down_tilt_acmd, Low)
    .effect_acmd("effect_attacklw3", ssbexo_ness_down_tilt_effect, Low)
    .install()
    ;
}