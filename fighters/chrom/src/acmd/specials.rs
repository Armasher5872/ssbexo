use super::*;

//Neutral Special Start Effect
unsafe extern "C" fn ssbexo_chrom_neutral_special_start_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), 3, 0, -10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//Neutral Special Start Sound
unsafe extern "C" fn ssbexo_chrom_neutral_special_start_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_chrom_special_n06"));
    }
}

//Neutral Special Start Expression
unsafe extern "C" fn ssbexo_chrom_neutral_special_start_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

//Neutral Special Loop Effect
unsafe extern "C" fn ssbexo_chrom_neutral_special_loop_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_volcano_hold_1"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_volcano_hold_2"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_sword_purple"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        FLASH(agent, 1, 1, 1, 0.863);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        FLASH(agent, 0.392, 0, 0.941, 0);
        FLASH_FRM(agent, 90, 0.392, 0, 0.941, 0.706);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 0.8, 8, 0, 7, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 6.0);
    for _ in 0..3 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword1"), 0, -0.0, 8, 0, 0, 0, 1, 2, 2, 8, 0, 0, 0, true);
        }
        wait(agent.lua_state_agent, 5.0);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_refraction_2"), Hash40::new("sword1"), 0, 0, 1.25, 0, 0, 0, 1, true);
    }
    if sv_animcmd::get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("chrom_level_2"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("chrom_level_2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
    }
    for _ in 0..4 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword1"), 0, -0.0, 8, 0, 0, 0, 1, 2, 2, 8, 0, 0, 0, true);
        }
        wait(agent.lua_state_agent, 5.0);
    }
    frame(agent.lua_state_agent, 41.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_refraction_2"), Hash40::new("sword1"), 0, 0, 1.25, 0, 0, 0, 1, true);
    }
    if sv_animcmd::get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("chrom_level_3"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("chrom_level_3"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
    }
    for _ in 0..4 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword1"), 0, -0.0, 8, 0, 0, 0, 1, 2, 2, 8, 0, 0, 0, true);
        }
        wait(agent.lua_state_agent, 5.0);
    }
}

//Neutral Special Loop Sound
unsafe extern "C" fn ssbexo_chrom_neutral_special_loop_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_chrom_special_n01"));
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_chrom_final_02"));
    }
}

//Neutral Special Loop Expression
unsafe extern "C" fn ssbexo_chrom_neutral_special_loop_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        agent.clear_lua_stack();
        lua_args!(agent, 0, 2, 130, 2, 1, 0, 12, 30, 30, 100);
        sv_animcmd::AREA_WIND_2ND_arg10(agent.lua_state_agent);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Neutral Special End ACMD
unsafe extern "C" fn ssbexo_chrom_neutral_special_end_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        KineticModule::set_consider_ground_friction(agent.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(agent.lua_state_agent, 8.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CHARGE_MAX) {
        for _ in 0..4 {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 367, 20, 0, 2, 5.0, 0.0, 6.0, 11.0, None, None, None, 0.5, 1.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
                ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 367, 20, 0, 2, 4.0, 0.0, 6.0, 16.0, None, None, None, 0.5, 1.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
                ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 367, 20, 0, 2, 4.5, 0.0, 6.0, 6.0, None, None, None, 0.5, 1.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            }
            wait(agent.lua_state_agent, 1.0);
            if is_excute(agent) {
                AttackModule::clear_all(agent.module_accessor);
            }
            wait(agent.lua_state_agent, 2.0);
        }
    }
    else {
        for _ in 0..4 {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 20, 0, 2, 5.0, 0.0, 6.0, 11.0, None, None, None, 0.5, 1.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
                ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 367, 20, 0, 2, 4.0, 0.0, 6.0, 16.0, None, None, None, 0.5, 1.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
                ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 367, 20, 0, 2, 4.5, 0.0, 6.0, 6.0, None, None, None, 0.5, 1.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            }
            wait(agent.lua_state_agent, 1.0);
            if is_excute(agent) {
                AttackModule::clear_all(agent.module_accessor);
            }
            wait(agent.lua_state_agent, 2.0);
        }
    }
    frame(agent.lua_state_agent, 20.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CHARGE_MAX) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 361, 120, 0, 50, 4.5, 0.0, 6.0, 5.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 361, 120, 0, 50, 5.0, 0.0, 7.0, 11.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 361, 120, 0, 50, 3.0, 0.0, 8.0, 18.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        }
    }
    else {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 4.275, 361, 120, 0, 50, 4.5, 0.0, 6.0, 5.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 4.275, 361, 120, 0, 50, 5.0, 0.0, 7.0, 11.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("top"), 4.275, 361, 120, 0, 50, 3.0, 0.0, 8.0, 18.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        }
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Neutral Special End Effect
unsafe extern "C" fn ssbexo_chrom_neutral_special_end_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_sword_green"), Hash40::new("sword1"), -0.0, 0, 0, 0, 0, 0, 1, true);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 0, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 5, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 10, 90, 0, 0, 0.5, true);
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("haver"), 0, -3.5, 0, -90, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        LAST_PARTICLE_SET_COLOR(agent, 0.4, 1, 0.3);
        LAST_EFFECT_SET_RATE(agent, 2);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("haver"), 0, -3.5, 0, -90, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        LAST_PARTICLE_SET_COLOR(agent, 0.4, 1, 0.3);
        LAST_EFFECT_SET_RATE(agent, 2);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 0, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 5, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 10, 90, 0, 0, 0.5, true);
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("haver"), 0, -3.5, 0, -90, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        LAST_PARTICLE_SET_COLOR(agent, 0.4, 1, 0.3);
        LAST_EFFECT_SET_RATE(agent, 2);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("haver"), 0, -3.5, 0, -90, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        LAST_PARTICLE_SET_COLOR(agent, 0.4, 1, 0.3);
        LAST_EFFECT_SET_RATE(agent, 2);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 0, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 5, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 10, 90, 0, 0, 0.5, true);
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -7, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("haver"), 0, -3.5, 0, -90, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_PARTICLE_SET_COLOR(agent, 0.4, 1, 0.3);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_sword_green"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("sword1"), 0, 0, 0, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("sword1"), 0, 0, 5, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("sword1"), 0, 0, 10, 90, 0, 0, 0.5, true);
    }
}

//Neutral Special End Sound
unsafe extern "C" fn ssbexo_chrom_neutral_special_end_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_chrom_special_s01"));
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_chrom_final_04"));
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_chrom_final_04"));
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_chrom_final_04"));
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_chrom_final_04"));
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_chrom_special_n02"));
    }
}

//Neutral Special End Expression
unsafe extern "C" fn ssbexo_chrom_neutral_special_end_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 10, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_piercess"), 4);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_piercel"), 0);
    }
}

//Side Special ACMD
unsafe extern "C" fn ssbexo_chrom_side_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CHROM_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_CHROM_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK);
    }
}

//Grounded Side Special Effect
unsafe extern "C" fn ssbexo_chrom_grounded_side_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.5);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_sword2"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FLW_POS(agent, Hash40::new("chrom_final_speedline"), Hash40::new("top"), 0, 4, -2, 0, 0, 0, 1, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_final_speedline"), false, true);
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 7.5, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    wait(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 7.5, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Side Special Effect
unsafe extern "C" fn ssbexo_chrom_aerial_side_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_sword2"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FLW_POS(agent, Hash40::new("chrom_final_speedline"), Hash40::new("top"), 0, 4, -2, 0, 0, 0, 1, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_final_speedline"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_sword2"), false, true);
    }
}

//Side Special Sound
unsafe extern "C" fn ssbexo_chrom_side_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_chrom_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_chrom_dash_start"));
        SET_PLAY_INHIVIT(agent, Hash40::new("se_chrom_dash_start"), 20);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_chrom_dash_stop"));
        SET_PLAY_INHIVIT(agent, Hash40::new("se_chrom_dash_stop"), 20);
    }
}

//Grounded Side Special Expression
unsafe extern "C" fn ssbexo_chrom_grounded_side_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Aerial Side Special Expression
unsafe extern "C" fn ssbexo_chrom_aerial_side_special_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Side Special Attack ACMD
unsafe extern "C" fn ssbexo_chrom_side_special_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 13.0, 110, 30, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword1"), 13.0, 110, 30, 0, 60, 5.0, 0.0, 0.0, 0.7, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword1"), 13.0, 110, 30, 0, 60, 5.0, 0.0, 0.0, 7.2, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword1"), 13.0, 110, 30, 0, 60, 5.0, 0.0, 0.0, 10.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 12.0, false);
        ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 0, 1, 2, 3, 0.0);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Grounded Side Special Attack Effect
unsafe extern "C" fn ssbexo_chrom_grounded_side_special_attack_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_sword2"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.5);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 0, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 5, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 10, 90, 0, 0, 0.5, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_chrom_sword1"), Hash40::new("tex_chrom_sword2"), 9, Hash40::new("sword1"), 0, 0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("chrom_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_sword2"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_attack_hi4_slash"), false, false);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 90, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Side Special Attack Effect
unsafe extern "C" fn ssbexo_chrom_aerial_side_special_attack_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_sword2"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_chrom_sword1"), Hash40::new("tex_chrom_sword2"), 9, Hash40::new("sword1"), 0, 0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("chrom_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_sword2"), false, true);
    }
}

//Side Special Attack Sound
unsafe extern "C" fn ssbexo_chrom_side_special_attack_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_chrom_rnd_attack"));
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_chrom_final_landing02"));
    }
}

//Grounded Side Special Attack Expression
unsafe extern "C" fn ssbexo_chrom_grounded_side_special_attack_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slash_critical"), 0);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_R, 3);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_landl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

//Aerial Side Special Attack Expression
unsafe extern "C" fn ssbexo_chrom_aerial_side_special_attack_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slash_critical"), 0);
    }
}

//Up Special ACMD
unsafe extern "C" fn ssbexo_chrom_up_special_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        SET_SPEED_EX(agent, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP);
    }
}

//Grounded Up Special Effect
unsafe extern "C" fn ssbexo_chrom_grounded_up_special_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("top"), 0, 0, -7, 90, 0, 0, 1.0, true);
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_tenku_jump"), Hash40::new("top"), 0, 0, -6, 0, 0, 0, 1, false);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("sword1"), 0, 0, 0, 90, 0, 0, 0.8, true);
        EFFECT_DETACH_KIND(agent, Hash40::new("chrom_attack_hi4_slash_finish"), -1);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_tenku_line"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.95);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
}

//Aerial Up Special Effect
unsafe extern "C" fn ssbexo_chrom_aerial_up_special_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("top"), 0, 0, -7, 90, 0, 0, 1.0, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_tenku_jump"), Hash40::new("top"), 0, 0, -6, 0, 0, 0, 1, false);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("sword1"), 0, 0, 0, 90, 0, 0, 0.8, true);
        EFFECT_DETACH_KIND(agent, Hash40::new("chrom_attack_hi4_slash_finish"), -1);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_tenku_line"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.95);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
}

//Up Special Sound
unsafe extern "C" fn ssbexo_chrom_up_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_chrom_special_h03"));
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_chrom_rnd_special_h"));
    }
}

//Grounded Up Special Expression
unsafe extern "C" fn ssbexo_chrom_grounded_up_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Aerial Up Special Expression
unsafe extern "C" fn ssbexo_chrom_aerial_up_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Up Special Hold ACMD
unsafe extern "C" fn ssbexo_chrom_up_special_hold_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, -20);
        KineticModule::clear_speed_all(agent.module_accessor);
        ADD_SPEED_NO_LIMIT(agent, 0, -6);
    }
}

//Up Special Hold Effect
unsafe extern "C" fn ssbexo_chrom_up_special_hold_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 0, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 5, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 10, 90, 0, 0, 0.5, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_chrom_sword1"), Hash40::new("tex_chrom_sword2"), 9, Hash40::new("sword1"), 0, 0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("chrom_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
}

//Up Special Hold Sound
unsafe extern "C" fn ssbexo_chrom_up_special_hold_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_chrom_special_h04"));
    }
}

//Up Special Hold Expression
unsafe extern "C" fn ssbexo_chrom_up_special_hold_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Up Special Drop ACMD
unsafe extern "C" fn ssbexo_chrom_up_special_drop_acmd(agent: &mut L2CAgentBase) {
    let get_stick_x = ControlModule::get_stick_x(agent.module_accessor);
    let lr = PostureModule::lr(agent.module_accessor);
    let stick_x = get_stick_x*lr;
    if is_excute(agent) {
        camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, -20);
        KineticModule::clear_speed_all(agent.module_accessor);
        ADD_SPEED_NO_LIMIT(agent, 3.0*stick_x, -6);
        ATTACK(agent, 0, 0, Hash40::new("armr"), 10.0, 270, 50, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_pierce"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword1"), 10.0, 270, 50, 0, 60, 5.0, 0.0, 0.0, 0.7, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_pierce"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword1"), 10.0, 270, 50, 0, 60, 5.0, 0.0, 0.0, 7.2, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_pierce"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword1"), 10.0, 270, 50, 0, 60, 5.0, 0.0, 0.0, 10.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_pierce"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
}

//Up Special Drop Effect
unsafe extern "C" fn ssbexo_chrom_up_special_drop_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_tenku_line"), Hash40::new("top"), -5, 0, 0, -180, 0, 0, 1.8, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.85);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    for _ in 0..3 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 0, 90, 0, 0, 0.5, true);
            EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 5, 90, 0, 0, 0.5, true);
            EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 10, 90, 0, 0, 0.5, true);
        }
        wait(agent.lua_state_agent, 5.0);
    }
}

//Up Special Drop Sound
unsafe extern "C" fn ssbexo_chrom_up_special_drop_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_chrom_special_h05"));
    }
}

//Up Special Drop Expression
unsafe extern "C" fn ssbexo_chrom_up_special_drop_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
}

//Up Special Land ACMD
unsafe extern "C" fn ssbexo_chrom_up_special_land_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 60, 115, 0, 70, 10.0, 0.0, 6.0, 11.8, Some(0.0), Some(11.0), Some(11.8), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Up Special Land Effect
unsafe extern "C" fn ssbexo_chrom_up_special_land_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_tenku_sword_down"), false, false);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("chrom_tenku_landing"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.9);
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 10, 0, 0, 0, 180, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_sword"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("sword1"), 0, 0, 0, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("sword1"), 0, 0, 5, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("sword1"), 0, 0, 10, 90, 0, 0, 0.5, true);
    }
}

//Up Special Land Sound
unsafe extern "C" fn ssbexo_chrom_up_special_land_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_chrom_special_h05"));
        PLAY_SE(agent, Hash40::new("se_chrom_special_h06"));
    }
}

//Up Special Land Expression
unsafe extern "C" fn ssbexo_chrom_up_special_land_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

//Down Special Hit ACMD
unsafe extern "C" fn ssbexo_chrom_down_special_hit_acmd(agent: &mut L2CAgentBase) {
    let hit_count = WorkModule::get_int(agent.module_accessor, *FIGHTER_CHROM_INSTANCE_WORK_ID_INT_SPECIAL_LW_HIT_COUNT);
    let fixed_knockback = 100+(25*hit_count);
    let distance = 13.0+(10.0*hit_count as f32);
    let hit_sound = match hit_count {3 => *ATTACK_SOUND_LEVEL_L, 2 => *ATTACK_SOUND_LEVEL_M, _ => *ATTACK_SOUND_LEVEL_S};
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 150, 50, 60, 0, 7.5, 0.0, 8.0, 18.0, Some(0.0), Some(8.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), hit_sound, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 150, 50, fixed_knockback, 0, 7.7, 0.0, 8.0, distance, Some(0.0), Some(8.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), hit_sound, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 30.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 24.0, false);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
        if is_excute(agent) {
            AttackModule::set_optional_hit_sound(agent.module_accessor, 0, Hash40::new("se_chrom_criticalhit"));
            AttackModule::set_optional_hit_sound(agent.module_accessor, 1, Hash40::new("se_chrom_criticalhit"));
        }
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Grounded Down Special Hit Effect
unsafe extern "C" fn ssbexo_chrom_grounded_down_special_hit_effect(agent: &mut L2CAgentBase) {
    let hit_count = WorkModule::get_int(agent.module_accessor, *FIGHTER_CHROM_INSTANCE_WORK_ID_INT_SPECIAL_LW_HIT_COUNT);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_guard_mark"), true, true);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("chrom_counter_success"), Hash40::new("top"), 0, 14.8, -1, 0, 90, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 14.8, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_chrom_sword1"), Hash40::new("tex_chrom_sword2"), 9, Hash40::new("sword1"), 0, 0, 1.7, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("chrom_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    match hit_count {
        3 => {
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 14, 90, 0, 0, 1.0, true);
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 21, 90, 0, 0, 1.0, true);
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 28, 90, 0, 0, 1.0, true);
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 35, 90, 0, 0, 1.0, true);
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 42, 90, 0, 0, 1.0, true);
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 49, 90, 0, 0, 1.0, true);
            }
        }
        2 => {
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 14, 90, 0, 0, 0.8, true);
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 21, 90, 0, 0, 0.8, true);
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 28, 90, 0, 0, 0.8, true);
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 35, 90, 0, 0, 0.8, true);
            }
        }
        1 => {
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 14, 90, 0, 0, 0.6, true);
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 21, 90, 0, 0, 0.6, true);
            }
        }
        _ => {
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 14, 90, 0, 0, 0.5, true);
            }
        }
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        FLASH(agent, 1, 1, 1, 0);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
}

//Aerial Down Special Hit Effect
unsafe extern "C" fn ssbexo_chrom_aerial_down_special_hit_effect(agent: &mut L2CAgentBase) {
    let hit_count = WorkModule::get_int(agent.module_accessor, *FIGHTER_CHROM_INSTANCE_WORK_ID_INT_SPECIAL_LW_HIT_COUNT);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_guard_mark"), true, true);
        EFFECT(agent, Hash40::new("chrom_counter_success"), Hash40::new("top"), 0, 14.8, -1, 0, 90, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 14.8, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_chrom_sword1"), Hash40::new("tex_chrom_sword2"), 9, Hash40::new("sword1"), 0, 0, 1.7, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("chrom_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    match hit_count {
        3 => {
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 14, 90, 0, 0, 1.0, true);
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 21, 90, 0, 0, 1.0, true);
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 28, 90, 0, 0, 1.0, true);
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 35, 90, 0, 0, 1.0, true);
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 42, 90, 0, 0, 1.0, true);
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 49, 90, 0, 0, 1.0, true);
            }
        }
        2 => {
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 14, 90, 0, 0, 0.8, true);
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 21, 90, 0, 0, 0.8, true);
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 28, 90, 0, 0, 0.8, true);
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 35, 90, 0, 0, 0.8, true);
            }
        }
        1 => {
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 14, 90, 0, 0, 0.6, true);
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 21, 90, 0, 0, 0.6, true);
            }
        }
        _ => {
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("top"), 0, 7, 14, 90, 0, 0, 0.5, true);
            }
        }
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        FLASH(agent, 1, 1, 1, 0);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
}

//Down Special Hit Sound
unsafe extern "C" fn ssbexo_chrom_down_special_hit_sound(agent: &mut L2CAgentBase) {
    let hit_count = WorkModule::get_int(agent.module_accessor, *FIGHTER_CHROM_INSTANCE_WORK_ID_INT_SPECIAL_LW_HIT_COUNT);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_chrom_rnd_special_l"));
    }
    frame(agent.lua_state_agent, 1.0);
    match hit_count {
        3 => {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("se_chrom_criticalhit"));
            }
        }
        2 => {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("se_chrom_special_l02"));
            }
        }
        _ => {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("se_chrom_special_n07"));
            }
        }
    }
    wait(agent.lua_state_agent, 3.0);
    match hit_count {
        3 => {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("se_chrom_smash_h01"));
            }
        }
        2 => {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("se_chrom_special_l03"));
            }
        }
        _ => {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("se_chrom_attackl_h01"));
            }
        }
    }
}

pub fn install() {
    Agent::new("chrom")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .effect_acmd("effect_specialnstart", ssbexo_chrom_neutral_special_start_effect, Low)
    .effect_acmd("effect_specialairnstart", ssbexo_chrom_neutral_special_start_effect, Low)
    .sound_acmd("sound_specialnstart", ssbexo_chrom_neutral_special_start_sound, Low)
    .sound_acmd("sound_specialairnstart", ssbexo_chrom_neutral_special_start_sound, Low)
    .expression_acmd("expression_specialnstart", ssbexo_chrom_neutral_special_start_expression, Low)
    .expression_acmd("expression_specialairnstart", ssbexo_chrom_neutral_special_start_expression, Low)
    .effect_acmd("effect_specialnloop", ssbexo_chrom_neutral_special_loop_effect, Low)
    .effect_acmd("effect_specialairnloop", ssbexo_chrom_neutral_special_loop_effect, Low)
    .sound_acmd("sound_specialnloop", ssbexo_chrom_neutral_special_loop_sound, Low)
    .sound_acmd("sound_specialairnloop", ssbexo_chrom_neutral_special_loop_sound, Low)
    .expression_acmd("expression_specialnloop", ssbexo_chrom_neutral_special_loop_expression, Low)
    .expression_acmd("expression_specialairnloop", ssbexo_chrom_neutral_special_loop_expression, Low)
    .game_acmd("game_specialnendmax", ssbexo_chrom_neutral_special_end_acmd, Low)
    .game_acmd("game_specialairnendmax", ssbexo_chrom_neutral_special_end_acmd, Low)
    .effect_acmd("effect_specialnendmax", ssbexo_chrom_neutral_special_end_effect, Low)
    .effect_acmd("effect_specialairnendmax", ssbexo_chrom_neutral_special_end_effect, Low)
    .sound_acmd("sound_specialnendmax", ssbexo_chrom_neutral_special_end_sound, Low)
    .sound_acmd("sound_specialairnendmax", ssbexo_chrom_neutral_special_end_sound, Low)
    .expression_acmd("expression_specialnendmax", ssbexo_chrom_neutral_special_end_expression, Low)
    .expression_acmd("expression_specialairnendmax", ssbexo_chrom_neutral_special_end_expression, Low)
    .game_acmd("game_specials", ssbexo_chrom_side_special_acmd, Low)
    .effect_acmd("effect_specials", ssbexo_chrom_grounded_side_special_effect, Low)
    .sound_acmd("sound_specials", ssbexo_chrom_side_special_sound, Low)
    .expression_acmd("expression_specials", ssbexo_chrom_grounded_side_special_expression, Low)
    .game_acmd("game_specialairs", ssbexo_chrom_side_special_acmd, Low)
    .effect_acmd("effect_specialairs", ssbexo_chrom_aerial_side_special_effect, Low)
    .sound_acmd("sound_specialairs", ssbexo_chrom_side_special_sound, Low)
    .expression_acmd("expression_specialairs", ssbexo_chrom_aerial_side_special_expression, Low)
    .game_acmd("game_specialsattack", ssbexo_chrom_side_special_attack_acmd, Low)
    .effect_acmd("effect_specialsattack", ssbexo_chrom_grounded_side_special_attack_effect, Low)
    .sound_acmd("sound_specialsattack", ssbexo_chrom_side_special_attack_sound, Low)
    .expression_acmd("expression_specialsattack", ssbexo_chrom_grounded_side_special_attack_expression, Low)
    .game_acmd("game_specialairsattack", ssbexo_chrom_side_special_attack_acmd, Low)
    .effect_acmd("effect_specialairsattack", ssbexo_chrom_aerial_side_special_attack_effect, Low)
    .sound_acmd("sound_specialairsattack", ssbexo_chrom_side_special_attack_sound, Low)
    .expression_acmd("expression_specialairsattack", ssbexo_chrom_aerial_side_special_attack_expression, Low)
    .game_acmd("game_specialhi", ssbexo_chrom_up_special_acmd, Low)
    .effect_acmd("effect_specialhi", ssbexo_chrom_grounded_up_special_effect, Low)
    .sound_acmd("sound_specialhi", ssbexo_chrom_up_special_sound, Low)
    .expression_acmd("expression_specialhi", ssbexo_chrom_grounded_up_special_expression, Low)
    .game_acmd("game_specialairhi", ssbexo_chrom_up_special_acmd, Low)
    .effect_acmd("effect_specialairhi", ssbexo_chrom_aerial_up_special_effect, Low)
    .sound_acmd("sound_specialairhi", ssbexo_chrom_up_special_sound, Low)
    .expression_acmd("expression_specialairhi", ssbexo_chrom_aerial_up_special_expression, Low)
    .game_acmd("game_specialhihold", ssbexo_chrom_up_special_hold_acmd, Low)
    .effect_acmd("effect_specialhihold", ssbexo_chrom_up_special_hold_effect, Low)
    .sound_acmd("sound_specialhihold", ssbexo_chrom_up_special_hold_sound, Low)
    .expression_acmd("expression_specialhihold", ssbexo_chrom_up_special_hold_expression, Low)
    .game_acmd("game_specialhidrop", ssbexo_chrom_up_special_drop_acmd, Low)
    .effect_acmd("effect_specialhidrop", ssbexo_chrom_up_special_drop_effect, Low)
    .sound_acmd("sound_specialhidrop", ssbexo_chrom_up_special_drop_sound, Low)
    .expression_acmd("expression_specialhidrop", ssbexo_chrom_up_special_drop_expression, Low)
    .game_acmd("game_specialhiland", ssbexo_chrom_up_special_land_acmd, Low)
    .effect_acmd("effect_specialhiland", ssbexo_chrom_up_special_land_effect, Low)
    .sound_acmd("sound_specialhiland", ssbexo_chrom_up_special_land_sound, Low)
    .expression_acmd("expression_specialhiland", ssbexo_chrom_up_special_land_expression, Low)
    .game_acmd("game_speciallwhit", ssbexo_chrom_down_special_hit_acmd, Low)
    .game_acmd("game_specialairlwhit", ssbexo_chrom_down_special_hit_acmd, Low)
    .effect_acmd("effect_speciallwhit", ssbexo_chrom_grounded_down_special_hit_effect, Low)
    .effect_acmd("effect_specialairlwhit", ssbexo_chrom_aerial_down_special_hit_effect, Low)
    .sound_acmd("sound_speciallwhit", ssbexo_chrom_down_special_hit_sound, Low)
    .sound_acmd("sound_specialairlwhit", ssbexo_chrom_down_special_hit_sound, Low)
    .install()
    ;
}