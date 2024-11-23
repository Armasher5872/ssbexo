use super::*;

//Forward Smash Charge Effect
unsafe extern "C" fn ssbexo_mewtwo_forward_smash_charge_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 1, 0, 1.3, 0, 0, 0, 0.25, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, false);
    }
    wait(agent.lua_state_agent, 5.0);
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            macros::EFFECT_FLIP(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sys_smash_flash_s"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, 7, 7, 7, 0, 0, 0, true, *EF_FLIP_YZ);
        }
        wait(agent.lua_state_agent, 5.0);
    }
}

//Forward Smash Effect
unsafe extern "C" fn ssbexo_mewtwo_forward_smash_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_pk_attack_b"), Hash40::new("mewtwo_pk_attack_b"), Hash40::new("top"), 0, 10, 16.5, 0, 0, 0, 1, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_deathscythe_trace"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_deathscythe_aura"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_shadowball_hold"), false, false);
    }
}

//Forward Smash Hi Effect
unsafe extern "C" fn ssbexo_mewtwo_forward_smash_hi_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_pk_attack_b"), Hash40::new("mewtwo_pk_attack_b"), Hash40::new("top"), 0, 13.5, 16.5, -30, 0, 0, 1, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_deathscythe_trace"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_deathscythe_aura"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_shadowball_hold"), false, false);
    }
}

//Forward Smash Lw Effect
unsafe extern "C" fn ssbexo_mewtwo_forward_smash_lw_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_pk_attack_b"), Hash40::new("mewtwo_pk_attack_b"), Hash40::new("top"), 0, 6.5, 16.5, 30, 0, 0, 1, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_deathscythe_trace"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_deathscythe_aura"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_shadowball_hold"), false, false);
    }
}

//Up Smash Charge Effect
unsafe extern "C" fn ssbexo_mewtwo_up_smash_charge_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_aura"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), -1, 0, 1.3, 0, 0, 0, 0.7, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 10, 0, 5, 0, 0, 0, false);
    }
    wait(agent.lua_state_agent, 5.0);
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            macros::EFFECT_FLIP(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sys_smash_flash_s"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, 7, 7, 7, 0, 0, 0, true, *EF_FLIP_YZ);
        }
        wait(agent.lua_state_agent, 5.0);
    }
}

//Up Smash ACMD
unsafe extern "C" fn ssbexo_mewtwo_up_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 368, 100, 90, 0, 5.0, 0.0, 4.0, -12.0, Some(0.0), Some(4.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_vec_target_pos(agent.module_accessor, 2, Hash40::new("top"), &Vector2f{x: 0.0, y: 24.0}, 7, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 10.0);
    for _ in 0..3 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 170, 100, 18, 0, 7.0, 0.0, 22.0, -5.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 170, 100, 18, 0, 7.0, 0.0, 22.0, 5.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 95, 100, 40, 0, 7.0, 0.0, 14.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 270, 100, 5, 0, 7.0, 0.0, 23.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 80, 111, 0, 60, 7.0, 0.0, 22.0, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 80, 111, 0, 60, 7.0, 0.0, 22.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 90, 111, 0, 60, 7.0, 0.0, 23.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 10.0, 90, 111, 0, 60, 7.0, 0.0, 14.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Up Smash Effect
unsafe extern "C" fn ssbexo_mewtwo_up_smash_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_aura"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), -1, 0, 1.3, 0, 0, 0, 0.7, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_pk_hand"), false, false);
    }
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mewtwo_pk_attack_a"), Hash40::new("top"), 2.5, 24, 0.5, 0, 180, 0, 1, true);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mewtwo_pk_attack_a"), Hash40::new("top"), -2.5, 24, 0.5, 0, 0, 0, 1, true);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_deathscythe_trace"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_deathscythe_aura"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_shadowball_hold"), false, false);
    }
}

//Down Smash Charge Effect
unsafe extern "C" fn ssbexo_mewtwo_down_smash_charge_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_aura"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), -1, 0, 1.3, 0, 0, 0, 0.7, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 10, 10, 10, 0, 0, 0, false);
    }
    wait(agent.lua_state_agent, 5.0);
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            macros::EFFECT_FLIP(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sys_smash_flash_s"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, 7, 7, 7, 0, 0, 0, true, *EF_FLIP_YZ);
        }
        wait(agent.lua_state_agent, 5.0);
    }
}

//Down Smash ACMD
unsafe extern "C" fn ssbexo_mewtwo_down_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 55, 118, 0, 20, 7.5, 0.0, 4.0, 13.7, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 55, 118, 0, 20, 11.0, 0.0, 4.0, 13.7, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Smash Effect
unsafe extern "C" fn ssbexo_mewtwo_down_smash_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 15, -6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_aura"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), -1.5, 0, 2, 0, 0, 0, 0.7, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_pk_hand"), true, true);
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0, 1, 3, 0, 0, 0, 1.3, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_pk_attack_c"), Hash40::new("mewtwo_pk_attack_c"), Hash40::new("top"), 0, 3, 13, 0, 0, 0, 1, true, *EF_FLIP_YZ);
        macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 13, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_deathscythe_trace"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_deathscythe_aura"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_shadowball_hold"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_pk_hand"), false, false);
    }
}

pub fn install() {
    Agent::new("mewtwo")
    .effect_acmd("effect_attacks4charge", ssbexo_mewtwo_forward_smash_charge_effect, Priority::Low)
    .effect_acmd("effect_attacks4", ssbexo_mewtwo_forward_smash_effect, Priority::Low)
    .effect_acmd("effect_attacks4hi", ssbexo_mewtwo_forward_smash_hi_effect, Priority::Low)
    .effect_acmd("effect_attacks4lw", ssbexo_mewtwo_forward_smash_lw_effect, Priority::Low)
    .effect_acmd("effect_attackhi4charge", ssbexo_mewtwo_up_smash_charge_effect, Priority::Low)
    .game_acmd("game_attackhi4", ssbexo_mewtwo_up_smash_acmd, Priority::Low)
    .effect_acmd("effect_attackhi4", ssbexo_mewtwo_up_smash_effect, Priority::Low)
    .effect_acmd("effect_attacklw4charge", ssbexo_mewtwo_down_smash_charge_effect, Priority::Low)
    .game_acmd("game_attacklw4", ssbexo_mewtwo_down_smash_acmd, Priority::Low)
    .effect_acmd("effect_attacklw4", ssbexo_mewtwo_down_smash_effect, Priority::Low)
    .install()
    ;
}