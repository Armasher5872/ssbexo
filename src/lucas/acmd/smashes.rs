use super::*;

//Forward Smash Charge Effect
unsafe extern "C" fn ssbexo_lucas_forward_smash_charge_effect(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 6, 0, 4, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 5.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_bullet"), true, true);
            macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("haver"), 0, 4, 0, 0, 0, 0, 0.6, true);
            macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("haver"), 0, 8, 0, 0, 0, 0, 0.6, true);
        }
        else {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 8, 0, 0, 0, 0, 1, 2, 5, 2, 0, 0, 0, true);
        }
    }
}

//Forward Smash ACMD
unsafe extern "C" fn ssbexo_lucas_forward_smash_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("bat") as i64, hash40("bat_visible") as i64);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCAS_STATUS_ATTACK_S4_FLAG_REFLECT_START);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCAS_STATUS_ATTACK_S4_FLAG_REFLECT_START);
    }
    frame(agent.lua_state_agent, 14.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 88, 0, 20, 3.7, 0.0, 5.6, 7.0, Some(0.0), Some(5.6), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 361, 91, 0, 20, 3.7, 0.0, 5.6, 13.0, Some(0.0), Some(5.6), Some(7.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 88, 0, 50, 3.7, 0.0, 5.6, 7.0, Some(0.0), Some(5.6), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCAS_BAT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 361, 91, 0, 50, 3.7, 0.0, 5.6, 13.0, Some(0.0), Some(5.6), Some(7.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCAS_BAT, *ATTACK_REGION_OBJECT);
        }
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCAS_STATUS_ATTACK_S4_FLAG_REFLECT_END);
    }
}

//Forward Smash Hi ACMD
unsafe extern "C" fn ssbexo_lucas_forward_smash_hi_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("bat") as i64, hash40("bat_visible") as i64);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCAS_STATUS_ATTACK_S4_FLAG_REFLECT_START);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCAS_STATUS_ATTACK_S4_FLAG_REFLECT_START);
    }
    frame(agent.lua_state_agent, 14.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 88, 0, 20, 3.7, 0.0, 9.2, 7.0, Some(0.0), Some(5.6), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 361, 91, 0, 20, 3.7, 0.0, 12.6, 12.0, Some(0.0), Some(9.2), Some(7.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 88, 0, 50, 3.7, 0.0, 9.2, 7.0, Some(0.0), Some(5.6), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCAS_BAT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 361, 91, 0, 50, 3.7, 0.0, 12.6, 12.0, Some(0.0), Some(9.2), Some(7.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCAS_BAT, *ATTACK_REGION_OBJECT);
        }
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCAS_STATUS_ATTACK_S4_FLAG_REFLECT_END);
    }
}

//Forward Smash Lw ACMD
unsafe extern "C" fn ssbexo_lucas_forward_smash_lw_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("bat") as i64, hash40("bat_visible") as i64);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCAS_STATUS_ATTACK_S4_FLAG_REFLECT_START);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCAS_STATUS_ATTACK_S4_FLAG_REFLECT_START);
    }
    frame(agent.lua_state_agent, 14.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 88, 0, 20, 3.7, 0.0, 4.4, 9.8, Some(0.0), Some(6.6), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 361, 91, 0, 20, 3.7, 0.0, 2.6, 15.0, Some(0.0), Some(4.4), Some(9.8), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 88, 0, 50, 3.7, 0.0, 4.4, 9.8, Some(0.0), Some(6.6), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCAS_BAT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 361, 91, 0, 50, 3.7, 0.0, 2.6, 15.0, Some(0.0), Some(4.4), Some(9.8), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCAS_BAT, *ATTACK_REGION_OBJECT);
        }
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCAS_STATUS_ATTACK_S4_FLAG_REFLECT_END);
    }
}

//Forward Smash Effect
unsafe extern "C" fn ssbexo_lucas_forward_smash_effect(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("haver"), 0, 4, 0, 0, 0, 0, 0.6, true);
            macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("haver"), 0, 8, 0, 0, 0, 0, 0.6, true);
        }
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0.0, 8, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("lucas_smash_arc"), Hash40::new("lucas_smash_arc"), Hash40::new("top"), 1, 6, 3.5, 0, -20, 10, 1, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), -2, 6.5, 14, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 360, true, 0.6);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_bullet"), true, true);
    }
}

//Forward Smash Hi Effect
unsafe extern "C" fn ssbexo_lucas_forward_smash_hi_effect(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("haver"), 0, 4, 0, 0, 0, 0, 0.6, true);
            macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("haver"), 0, 8, 0, 0, 0, 0, 0.6, true);
        }
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("lucas_smash_arc"), Hash40::new("lucas_smash_arc"), Hash40::new("top"), 1, 6, 3.5, -30, -20, 10, 1, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), -2, 8.5, 14, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 360, true, 0.6);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_bullet"), true, true);
    }
}

//Forward Smash Lw Effect
unsafe extern "C" fn ssbexo_lucas_forward_smash_lw_effect(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("haver"), 0, 4, 0, 0, 0, 0, 0.6, true);
            macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("haver"), 0, 8, 0, 0, 0, 0, 0.6, true);
        }
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("lucas_smash_arc"), Hash40::new("lucas_smash_arc"), Hash40::new("top"), 1, 6, 3.5, 20, -20, 10, 1, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), -2, 4.5, 14, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 360, true, 0.6);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_bullet"), true, true);
    }
}

//Up Smash Charge Effect
unsafe extern "C" fn ssbexo_lucas_up_smash_charge_effect(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        frame(agent.lua_state_agent, 1.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
            if macros::is_excute(agent) {
                macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_bullet"), true, true);
                macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("handr"), 0, 4, 0, 0, 0, 0, 0.6, true);
                macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("handl"), 0, 8, 0, 0, 0, 0, 0.6, true);
                macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 5, 0, 4, 0, 0, 0, false);
            }
        }
        else {
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("lucas_psi_hold"), Hash40::new("lucas_psi_hold"), Hash40::new("top"), -4, 3.5, 1, 0, 0, 0, 1, true, *EF_FLIP_YZ);
                macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 5, 0, 4, 0, 0, 0, false);
            }
        }
        wait(agent.lua_state_agent, 5.0);
    }
}

//Up Smash ACMD
unsafe extern "C" fn ssbexo_lucas_up_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
    }
    frame(agent.lua_state_agent, 5.0);
    execute(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        macros::WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_INVINCIBLE);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 110, 90, 110, 0, 6.0, 0.0, 5.0, -8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 110, 90, 110, 0, 6.0, 0.0, 5.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 30.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            macros::ATTACK(agent, 0, 0, Hash40::new("throw"), 21.0, 95, 77, 0, 18, 11.0, 0.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        }
    }
    else {
        if macros::is_excute(agent) {
            HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            macros::ATTACK(agent, 0, 0, Hash40::new("throw"), 21.0, 95, 77, 0, 48, 11.0, 0.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    frame(agent.lua_state_agent, 33.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 1, 0, Hash40::new("throw"), 21.0, 95, 77, 0, 18, 10.0, 0.0, -2.5, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
            AttackModule::clear(agent.module_accessor, 0, false);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 1, 0, Hash40::new("throw"), 21.0, 95, 77, 0, 48, 10.0, 0.0, -2.5, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            AttackModule::clear(agent.module_accessor, 0, false);
        }
    }
    frame(agent.lua_state_agent, 38.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("throw"), 21.0, 95, 77, 0, 18, 9.0, 0.0, 2.5, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
            AttackModule::clear(agent.module_accessor, 1, false);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("throw"), 21.0, 95, 77, 0, 48, 9.0, 0.0, 2.5, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            AttackModule::clear(agent.module_accessor, 1, false);
        }
    }
    frame(agent.lua_state_agent, 43.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 3, 0, Hash40::new("throw"), 21.0, 95, 77, 0, 18, 9.0, 0.0, -1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
            AttackModule::clear(agent.module_accessor, 2, false);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 3, 0, Hash40::new("throw"), 21.0, 95, 77, 0, 48, 9.0, 0.0, -1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            AttackModule::clear(agent.module_accessor, 2, false);
        }
    }
    frame(agent.lua_state_agent, 48.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("throw"), 21.0, 95, 77, 0, 18, 6.0, 0.0, 3.5, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
            AttackModule::clear(agent.module_accessor, 3, false);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("throw"), 21.0, 95, 77, 0, 48, 6.0, 0.0, 3.5, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            AttackModule::clear(agent.module_accessor, 3, false);
        }
    }
    frame(agent.lua_state_agent, 53.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("throw"), 21.0, 95, 77, 0, 18, 4.5, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("throw"), 21.0, 95, 77, 0, 48, 4.5, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    frame(agent.lua_state_agent, 55.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Up Smash Effect
unsafe extern "C" fn ssbexo_lucas_up_smash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.6, true);
            macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.6, true);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(agent.lua_state_agent, 20.0);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("lucas_psi_hold"), Hash40::new("lucas_psi_hold"), Hash40::new("haver"), 0, 0, 0.3, 0, 0, 0, 1, true, *EF_FLIP_YZ);
        }
    }
    frame(agent.lua_state_agent, 23.0);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_psi_hold"), true, true);
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("lucas_psi_catch"), Hash40::new("lucas_psi_catch"), Hash40::new("top"), 0, 15, -0.5, 0, 0, 0, 1.5, true, *EF_FLIP_YZ);
        }
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 26.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_bullet"), true, true);
            macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1.0, true);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("lucas_psi_atk_hi"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(agent.lua_state_agent, 30.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_bullet"), true, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("throw"), 0, 0, 0, -90, 0, 0, 4.0, true);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_psi_catch"), false, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0, 13, 0, -90, 0, 0, 1.55, true);
        }
    }
    frame(agent.lua_state_agent, 38.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_bullet"), true, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("throw"), 0, 0, 0, -90, 0, 0, 3.5, true);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_psi_atk"), true, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0, 25, 0, -90, 0, 0, 1.5, true);
        }
    }
    frame(agent.lua_state_agent, 48.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_bullet"), true, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("throw"), 0, 0, 0, -90, 0, 0, 3.0, true);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_psi_atk"), true, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0, 37, 0, -90, 0, 0, 1.1, true);
        }
    }
    frame(agent.lua_state_agent, 54.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_bullet"), false, false);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_psi_atk"), false, false);
        }
    }
}

//Down Smash Charge Effect
unsafe extern "C" fn ssbexo_lucas_down_smash_charge_effect(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        frame(agent.lua_state_agent, 5.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
            if macros::is_excute(agent) {
                macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_bullet"), false, false);
                macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.6, true);
                macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 4, 0, 6, 0, 0, 0, false);
            }
        }
        else {
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("lucas_psi_hold"), Hash40::new("lucas_psi_hold"), Hash40::new("havel"), 0.5, 0.5, 1.3, 0, 0, 0, 1, true, *EF_FLIP_YZ);
                macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 4, 0, 6, 0, 0, 0, false);
            }
        }
        wait(agent.lua_state_agent, 7.0);
    }
}

//Down Smash ACMD
unsafe extern "C" fn ssbexo_lucas_down_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 20.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 17.0, 361, 90, 0, 13, 8.0, 0.0, 2.5, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -8, 0.0, 8, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 17.0, 361, 90, 0, 43, 8.0, 0.0, 2.5, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -8, 0.0, 8, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::set_target_category(agent.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
    }
    frame(agent.lua_state_agent, 29.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 90, 0, 0, 9.8, 3.3, 2.5, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -6, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 90, 0, 30, 9.8, 3.3, 2.5, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -6, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::set_target_category(agent.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
    }
    frame(agent.lua_state_agent, 39.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 361, 90, 0, 0, 12.0, -5.6, 2.5, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 361, 90, 0, 20, 12.0, -5.6, 2.5, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -4, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Smash Effect
unsafe extern "C" fn ssbexo_lucas_down_smash_effect(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.6, true);
        }
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 8.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_bullet"), false, false);
            macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.8, true);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("lucas_psi_hold"), Hash40::new("lucas_psi_hold"), Hash40::new("havel"), 0.5, 0.5, 1.3, 0, 0, 0, 1, true, *EF_FLIP_YZ);
        }
    }
    frame(agent.lua_state_agent, 20.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_bullet"), false, false);
            macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bomb_max"), Hash40::new("top"), 0, 2.5, 8, 0, 0, 0, 1.25, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_pkfr_bullet_ed"), Hash40::new("top"), 0, 2.5, 8, 0, 0, 0, 1.25, true);
            macros::LAST_EFFECT_SET_RATE(agent, 0.33);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_psi_hold"), false, false);
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("lucas_psi_atk_lw"), Hash40::new("lucas_psi_atk_lw"), Hash40::new("top"), 0, 2.5, 8, 0, 0, 0, 1.25, true, *EF_FLIP_YZ);
        }
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.7, 2, 2, 2, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 29.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_bomb_max"), false, false);
            macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_bullet_ed"), false, false);
            macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bomb_max"), Hash40::new("top"), 0, 2.5, 8, 0, 0, 0, 1.3, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_pkfr_bullet_ed"), Hash40::new("top"), 0, 2.5, 8, 0, 0, 0, 1.3, true);
            macros::LAST_EFFECT_SET_RATE(agent, 0.33);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("lucas_psi_atk_lw"), Hash40::new("lucas_psi_atk_lw"), Hash40::new("top"), 0, 2.5, 8, 0, 0, 0, 1.3, true, *EF_FLIP_YZ);
        }
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.7, 2, 2, 2, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 39.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_bomb_max"), false, false);
            macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_bullet_ed"), false, false);
            macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bomb_max"), Hash40::new("top"), 0, 2.5, 8, 0, 0, 0, 1.45, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_pkfr_bullet_ed"), Hash40::new("top"), 0, 2.5, 8, 0, 0, 0, 1.45, true);
            macros::LAST_EFFECT_SET_RATE(agent, 0.33);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("lucas_psi_atk_lw"), Hash40::new("lucas_psi_atk_lw"), Hash40::new("top"), 0, 2.5, 8, 0, 0, 0, 1.45, true, *EF_FLIP_YZ);
        }
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 2, 2, 2, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_bomb_max"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_bullet_ed"), false, false);
    }
}

pub fn install() {
    Agent::new("lucas")
    .effect_acmd("effect_attacks4charge", ssbexo_lucas_forward_smash_charge_effect, Priority::Low)
    .game_acmd("game_attacks4", ssbexo_lucas_forward_smash_acmd, Priority::Low)
    .game_acmd("game_attacks4hi", ssbexo_lucas_forward_smash_hi_acmd, Priority::Low)
    .game_acmd("game_attacks4lw", ssbexo_lucas_forward_smash_lw_acmd, Priority::Low)
    .effect_acmd("effect_attacks4", ssbexo_lucas_forward_smash_effect, Priority::Low)
    .effect_acmd("effect_attacks4hi", ssbexo_lucas_forward_smash_hi_effect, Priority::Low)
    .effect_acmd("effect_attacks4lw", ssbexo_lucas_forward_smash_lw_effect, Priority::Low)
    .effect_acmd("effect_attackhi4charge", ssbexo_lucas_up_smash_charge_effect, Priority::Low)
    .game_acmd("game_attackhi4", ssbexo_lucas_up_smash_acmd, Priority::Low)
    .effect_acmd("effect_attackhi4", ssbexo_lucas_up_smash_effect, Priority::Low)
    .effect_acmd("effect_attacklw4charge", ssbexo_lucas_down_smash_charge_effect, Priority::Low)
    .game_acmd("game_attacklw4", ssbexo_lucas_down_smash_acmd, Priority::Low)
    .effect_acmd("effect_attacklw4", ssbexo_lucas_down_smash_effect, Priority::Low)
    .install()
    ;
}