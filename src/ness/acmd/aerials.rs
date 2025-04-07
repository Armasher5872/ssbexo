use super::*;

//Nair ACMD
unsafe extern "C" fn ssbexo_ness_nair_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 7.0, 65, 60, 0, 35, 6.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("handl"), 7.0, 65, 60, 0, 35, 6.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 2, 0, Hash40::new("hip"), 7.0, 65, 60, 0, 35, 7.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        }
    }
    else {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 7.0, 65, 60, 0, 35, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("handl"), 7.0, 65, 60, 0, 35, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 2, 0, Hash40::new("hip"), 7.0, 65, 60, 0, 35, 5.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    frame(agent.lua_state_agent, 13.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            macros::ATTACK(agent, 0, 1, Hash40::new("handr"), 5.0, 80, 50, 0, 20, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 1, Hash40::new("handl"), 5.0, 80, 50, 0, 20, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 2, 1, Hash40::new("hip"), 5.0, 80, 50, 0, 20, 7.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        }
    }
    else {
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            macros::ATTACK(agent, 0, 1, Hash40::new("handr"), 5.0, 80, 50, 0, 20, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 1, Hash40::new("handl"), 5.0, 80, 50, 0, 20, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 2, 1, Hash40::new("hip"), 5.0, 80, 50, 0, 20, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Nair Effect
unsafe extern "C" fn ssbexo_ness_nair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.7, 4, 40, 0, -13, 0.7, true);
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("ness_psi_atk"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.2, true);
            sv_animcmd::EFFECT_FOLLOW_NO_SCALE(agent.lua_state_agent);
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("ness_psi_atk"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.2, true);
            sv_animcmd::EFFECT_FOLLOW_NO_SCALE(agent.lua_state_agent);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.7, 4, 40, 0, -13, 0.7, true);
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("ness_psi_atk"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true);
            sv_animcmd::EFFECT_FOLLOW_NO_SCALE(agent.lua_state_agent);
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("ness_psi_atk"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.0, true);
            sv_animcmd::EFFECT_FOLLOW_NO_SCALE(agent.lua_state_agent);
        }
    }
    frame(agent.lua_state_agent, 11.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("ness_psi_atk"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.2, true);
            sv_animcmd::EFFECT_FOLLOW_NO_SCALE(agent.lua_state_agent);
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("ness_psi_atk"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.2, true);
            sv_animcmd::EFFECT_FOLLOW_NO_SCALE(agent.lua_state_agent);
        }
    }
    else {
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("ness_psi_atk"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true);
            sv_animcmd::EFFECT_FOLLOW_NO_SCALE(agent.lua_state_agent);
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("ness_psi_atk"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.0, true);
            sv_animcmd::EFFECT_FOLLOW_NO_SCALE(agent.lua_state_agent);
        }
    }
}

//Fair ACMD
unsafe extern "C" fn ssbexo_ness_fair_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 367, 60, 0, 16, 6.0, 0.0, 6.8, 10.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 367, 60, 0, 16, 4.0, 0.0, 5.8, 6.0, Some(0.0), Some(5.8), Some(6.0), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            ShieldModule::set_status(agent.module_accessor, *FIGHTER_NESS_SHIELD_KIND_PSI_ATTACK_AIR_F_GUARD, ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_NESS_SHIELD_GROUP_KIND_PSI_ATTACK_AIR_F_GUARD);
        }
    }
    else {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 367, 60, 0, 16, 6.0, 0.0, 4.8, 10.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 367, 60, 0, 16, 4.0, 0.0, 3.8, 6.0, Some(0.0), Some(3.8), Some(3.0), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        ShieldModule::set_status(agent.module_accessor, *FIGHTER_NESS_SHIELD_KIND_PSI_ATTACK_AIR_F_GUARD, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_NESS_SHIELD_GROUP_KIND_PSI_ATTACK_AIR_F_GUARD);
    }
    frame(agent.lua_state_agent, 20.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.5, 361, 128, 0, 32, 10.0, 0.0, 4.8, 9.6, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            ShieldModule::set_status(agent.module_accessor, *FIGHTER_NESS_SHIELD_KIND_PSI_ATTACK_AIR_F_2_GUARD, ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_NESS_SHIELD_GROUP_KIND_PSI_ATTACK_AIR_F_2_GUARD);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.5, 361, 128, 0, 32, 8.0, 0.0, 4.8, 9.6, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        ShieldModule::set_status(agent.module_accessor, *FIGHTER_NESS_SHIELD_KIND_PSI_ATTACK_AIR_F_2_GUARD, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_NESS_SHIELD_GROUP_KIND_PSI_ATTACK_AIR_F_2_GUARD);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Fair Effect
unsafe extern "C" fn ssbexo_ness_fair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("ness_psi_hold"), Hash40::new("ness_psi_hold"), Hash40::new("havel"), -2, 1, 1, 0, 0, 0, 1, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ness_psi_hold"), true, true);
    }
    for _ in 0..3 {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
            if macros::is_excute(agent) {
                agent.clear_lua_stack();
                lua_args!(agent, Hash40::new("ness_psi_rush"), Hash40::new("top"), 0, 5, 10, 90, 0, 90, 1.5, 1, 1, 1, 0, 0, 0, true);
                sv_animcmd::EFFECT_FOLLOW_RND(agent.lua_state_agent);
            }
        }
        else {
            if macros::is_excute(agent) {
                agent.clear_lua_stack();
                lua_args!(agent, Hash40::new("ness_psi_rush"), Hash40::new("top"), 0, 5, 10, 90, 0, 90, 1.3, 1, 1, 1, 0, 0, 0, true);
                sv_animcmd::EFFECT_FOLLOW_RND(agent.lua_state_agent);
            }
        }
        wait(agent.lua_state_agent, 4.0);
    }
    frame(agent.lua_state_agent, 20.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("ness_psi_rush"), Hash40::new("top"), 0, 4, 10.2, 90, 0, 90, 2.2, 0, 0, 0, 0, 360, 0, true);
            sv_animcmd::EFFECT_FOLLOW_RND(agent.lua_state_agent);
        }
    }
    else {
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("ness_psi_rush"), Hash40::new("top"), 0, 4, 10.2, 90, 0, 90, 2.0, 0, 0, 0, 0, 360, 0, true);
            sv_animcmd::EFFECT_FOLLOW_RND(agent.lua_state_agent);
        }
    }
}

//Bair ACMD
unsafe extern "C" fn ssbexo_ness_bair_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 15.0, 361, 100, 0, 16, 6.0, -1.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 15.0, 361, 100, 0, 16, 7.0, 3.7, 2.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    else {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 15.0, 361, 100, 0, 16, 4.0, -1.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 15.0, 361, 100, 0, 16, 5.0, 3.7, 2.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    wait(agent.lua_state_agent, 2.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 8.0, 361, 100, 0, 0, 6.5, -1.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 8.0, 361, 100, 0, 0, 7.5, 3.7, 2.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 8.0, 361, 100, 0, 0, 4.5, -1.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 8.0, 361, 100, 0, 0, 5.5, 3.7, 2.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Bair Effect
unsafe extern "C" fn ssbexo_ness_bair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -2, 3.7, 3, 0, 180, 0, 1, true, *EF_FLIP_YZ);
        macros::LAST_PARTICLE_SET_COLOR(agent, 0.2, 1, 0.8);
    }
    frame(agent.lua_state_agent, 10.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("ness_psi_atk"), Hash40::new("ness_psi_atk"), Hash40::new("top"), -3, 4, -9, 0, 180, 0, 1.7, true, *EF_FLIP_YZ);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("ness_psi_atk"), Hash40::new("ness_psi_atk"), Hash40::new("top"), -3, 4, -9, 0, 180, 0, 1.5, true, *EF_FLIP_YZ);
        }
    }
}

//Uair ACMD
unsafe extern "C" fn ssbexo_ness_uair_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 2.5, 366, 100, 30, 0, 7.5, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("handr"), 2.5, 366, 100, 30, 0, 5.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    else {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 2.5, 366, 100, 30, 0, 5.5, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("handr"), 2.5, 366, 100, 30, 0, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }   
    }
    frame(agent.lua_state_agent, 15.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 5.0, 73, 180, 0, 40, 8.5, 3.0, 0.0, 0.0, Some(5.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("handr"), 5.0, 73, 180, 0, 40, 5.5, -3.0, 0.0, 1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 5.0, 73, 180, 0, 40, 6.5, 3.0, 0.0, 0.0, Some(5.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("handr"), 5.0, 73, 180, 0, 40, 3.5, -3.0, 0.0, 1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 0.85);
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Uair Effect
unsafe extern "C" fn ssbexo_ness_uair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("ness_psi_rush"), Hash40::new("haver"), -0.5, 2.6, 2.9, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE);
            sv_animcmd::EFFECT_ATTR(agent.lua_state_agent);
        }
    }
    else {
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("ness_psi_rush"), Hash40::new("haver"), -0.5, 2.6, 2.9, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE);
            sv_animcmd::EFFECT_ATTR(agent.lua_state_agent);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("ness_psi_rush"), Hash40::new("haver"), -0.5, 2.6, 2.9, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE);
            sv_animcmd::EFFECT_ATTR(agent.lua_state_agent);
        }
    }
    else {
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("ness_psi_rush"), Hash40::new("haver"), -0.5, 2.6, 2.9, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE);
            sv_animcmd::EFFECT_ATTR(agent.lua_state_agent);
        }
    }
    frame(agent.lua_state_agent, 10.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("ness_psi_rush"), Hash40::new("haver"), -0.5, 2.6, 2.9, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE);
            sv_animcmd::EFFECT_ATTR(agent.lua_state_agent);
        }
    }
    else {
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("ness_psi_rush"), Hash40::new("haver"), -0.5, 2.6, 2.9, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE);
            sv_animcmd::EFFECT_ATTR(agent.lua_state_agent);
        }   
    }
    frame(agent.lua_state_agent, 12.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("ness_psi_rush"), Hash40::new("haver"), -0.5, 2.6, 2.9, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE);
            sv_animcmd::EFFECT_ATTR(agent.lua_state_agent);
        }
    }
    else {
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("ness_psi_rush"), Hash40::new("haver"), -0.5, 2.6, 2.9, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE);
            sv_animcmd::EFFECT_ATTR(agent.lua_state_agent);
        }   
    }
    frame(agent.lua_state_agent, 14.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("ness_psi_rush"), Hash40::new("haver"), -0.5, 2.6, 2.9, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE);
            sv_animcmd::EFFECT_ATTR(agent.lua_state_agent);
        }
    }
    else {
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("ness_psi_rush"), Hash40::new("haver"), -0.5, 2.6, 2.9, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE);
            sv_animcmd::EFFECT_ATTR(agent.lua_state_agent);
        }   
    }
}

//Dair ACMD
unsafe extern "C" fn ssbexo_ness_dair_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.895);
    frame(agent.lua_state_agent, 20.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 270, 74, 0, 37, 8.0, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PSI);
        }
    }
    else {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 270, 74, 0, 37, 6.0, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PSI);
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 45, 74, 0, 30, 8.0, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PSI);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 45, 74, 0, 30, 6.0, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PSI);
        }
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Dair Effect
unsafe extern "C" fn ssbexo_ness_dair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, -4, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 20.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("ness_psi_atk"), Hash40::new("top"), 0, -4, 0, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
            sv_animcmd::EFFECT_ATTR(agent.lua_state_agent);
        }
    }
    else {
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("ness_psi_atk"), Hash40::new("top"), 0, -4, 0, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
            sv_animcmd::EFFECT_ATTR(agent.lua_state_agent);
        }
    }
}

pub fn install() {
    Agent::new("ness")
    .game_acmd("game_attackairn", ssbexo_ness_nair_acmd, Priority::Low)
    .effect_acmd("effect_attackairn", ssbexo_ness_nair_effect, Priority::Low)
    .game_acmd("game_attackairf", ssbexo_ness_fair_acmd, Priority::Low)
    .effect_acmd("effect_attackairf", ssbexo_ness_fair_effect, Priority::Low)
    .game_acmd("game_attackairb", ssbexo_ness_bair_acmd, Priority::Low)
    .effect_acmd("effect_attackairb", ssbexo_ness_bair_effect, Priority::Low)
    .game_acmd("game_attackairhi", ssbexo_ness_uair_acmd, Priority::Low)
    .effect_acmd("effect_attackairhi", ssbexo_ness_uair_effect, Priority::Low)
    .game_acmd("game_attackairlw", ssbexo_ness_dair_acmd, Priority::Low)
    .effect_acmd("effect_attackairlw", ssbexo_ness_dair_effect, Priority::Low)
    .install()
    ;
}