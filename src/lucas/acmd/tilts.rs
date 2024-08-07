use super::*;

//Forward Tilt ACMD
unsafe extern "C" fn ssbexo_lucas_forward_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 0.0, 3.0);
    }
    frame(agent.lua_state_agent, 7.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 7.5, 361, 60, 0, 30, 4.2, 0.0, 0.0, 0.0, Some(-0.7), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 361, 80, 0, 45, 7.6, 0.0, 5.0, 9.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 7.5, 361, 60, 0, 30, 2.2, 0.0, 0.0, 0.0, Some(-0.7), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 361, 80, 0, 45, 5.6, 0.0, 5.0, 9.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::set_size(agent.module_accessor, 1, 4.7);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 2.5, 3.0);
    }
}

//Forward Tilt Hi ACMD
unsafe extern "C" fn ssbexo_lucas_forward_tilt_hi_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 0.0, 3.0);
    }
    frame(agent.lua_state_agent, 7.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 7.5, 361, 60, 0, 30, 4.2, 0.0, 0.0, 0.0, Some(-0.7), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 361, 80, 0, 45, 7.6, 0.0, 9.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 7.5, 361, 60, 0, 30, 2.2, 0.0, 0.0, 0.0, Some(-0.7), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 361, 80, 0, 45, 5.6, 0.0, 9.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::set_size(agent.module_accessor, 1, 4.7);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 2.5, 3.0);
    }
}

//Forward Tilt Lw ACMD
unsafe extern "C" fn ssbexo_lucas_forward_tilt_lw_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 0.0, 3.0);
    }
    frame(agent.lua_state_agent, 7.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 7.5, 361, 60, 0, 30, 4.2, 0.0, 0.0, 0.0, Some(-0.7), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 361, 80, 0, 45, 7.6, 0.0, 1.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 7.5, 361, 60, 0, 30, 2.2, 0.0, 0.0, 0.0, Some(-0.7), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 361, 80, 0, 45, 5.6, 0.0, 1.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::set_size(agent.module_accessor, 1, 4.7);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 2.5, 3.0);
    }
}

//Forward Tilt Effect
unsafe extern "C" fn ssbexo_lucas_forward_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_hold"), Hash40::new("haver"), -0.2, 0, 0, 0, 0, 0, 0.8, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 7.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0, 5, 9, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0, 5, 9, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_psi_atk"), false, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_psi_hold"), false, false);
    }
}

//Forward Tilt Hi Effect
unsafe extern "C" fn ssbexo_lucas_forward_tilt_hi_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_hold"), Hash40::new("haver"), -0.2, 0, 0, 0, 0, 0, 0.8, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -3, 0, 0, 16, 26, 169, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 7.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0, 8.7, 8.5, -20, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0, 8.7, 8.5, -20, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_psi_atk"), false, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_psi_hold"), false, false);
    }
}

//Forward Tilt Lw Effect
unsafe extern "C" fn ssbexo_lucas_forward_tilt_lw_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_hold"), Hash40::new("haver"), -0.2, 0, 0, 0, 0, 0, 0.8, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 7.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0, 2, 8.5, 20, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0, 2, 8.5, 20, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_psi_atk"), false, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_psi_hold"), false, false);
    }
}

//Up Tilt ACMD
unsafe extern "C" fn ssbexo_lucas_up_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 120, 100, 60, 0, 3.5, 0.0, 6.0, 6.5, None, None, None, 0.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 120, 100, 60, 0, 3.5, 0.0, 6.0, -6.5, None, None, None, 0.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 7.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 8.0, 93, 100, 0, 50, 7.5, 5.0, 0.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 8.0, 93, 100, 0, 50, 7.0, 0.0, 0.0, 0.0, None, None, None, 0.66, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 8.0, 93, 100, 0, 50, 5.5, 5.0, 0.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 8.0, 93, 100, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.66, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    frame(agent.lua_state_agent, 11.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 5.0, 90, 100, 0, 50, 7.2, 5.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            AttackModule::clear(agent.module_accessor, 1, false);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 5.0, 90, 100, 0, 50, 5.2, 5.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            AttackModule::clear(agent.module_accessor, 1, false);
        }
    }
    frame(agent.lua_state_agent, 15.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 5.0, 90, 100, 0, 50, 5.5, 6.5, 0.5, -0.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 5.0, 90, 100, 0, 50, 3.5, 6.5, 0.5, -0.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Up Tilt Effect
unsafe extern "C" fn ssbexo_lucas_up_tilt_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 7.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_atk"), Hash40::new("kneer"), 4, 0, 0, 0, 90, 0, 1.1, true);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_atk"), Hash40::new("kneer"), 4, 0, 0, 0, 90, 0, 0.9, true);
        }
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_psi_atk"), false, false);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
}

//Down Tilt ACMD
unsafe extern "C" fn ssbexo_lucas_down_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 85, 45, 0, 20, 2.8, 0.0, 3.5, 4.5, Some(0.0), Some(3.6), Some(6.0), 0.7, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 90, 45, 0, 10, 2.8, 0.0, 2.0, 10.5, None, None, None, 0.7, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 90, 45, 0, 10, 4.8, 0.0, 2.0, 15.5, None, None, None, 0.7, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 85, 45, 0, 20, 2.8, 0.0, 3.5, 4.5, Some(0.0), Some(3.6), Some(6.0), 0.7, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 90, 45, 0, 10, 2.8, 0.0, 2.0, 10.5, None, None, None, 0.7, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Tilt Effect
unsafe extern "C" fn ssbexo_lucas_down_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 2, 2.7, 0, 20, 0, 0.85, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_RATE(agent, 1.5);
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(agent, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0, 2, 12.5, 20, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 2, 2.7, 0, 20, 0, 0.85, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_RATE(agent, 1.5);
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

pub fn install() {
    Agent::new("lucas")
    .game_acmd("game_attacks3", ssbexo_lucas_forward_tilt_acmd, Priority::Low)
    .game_acmd("game_attacks3hi", ssbexo_lucas_forward_tilt_hi_acmd, Priority::Low)
    .game_acmd("game_attacks3lw", ssbexo_lucas_forward_tilt_lw_acmd, Priority::Low)
    .effect_acmd("effect_attacks3", ssbexo_lucas_forward_tilt_effect, Priority::Low)
    .effect_acmd("effect_attacks3hi", ssbexo_lucas_forward_tilt_hi_effect, Priority::Low)
    .effect_acmd("effect_attacks3lw", ssbexo_lucas_forward_tilt_lw_effect, Priority::Low)
    .game_acmd("game_attackhi3", ssbexo_lucas_up_tilt_acmd, Priority::Low)
    .effect_acmd("effect_attackhi3", ssbexo_lucas_up_tilt_effect, Priority::Low)
    .game_acmd("game_attacklw3", ssbexo_lucas_down_tilt_acmd, Priority::Low)
    .effect_acmd("effect_attacklw3", ssbexo_lucas_down_tilt_effect, Priority::Low)
    .install()
    ;
}