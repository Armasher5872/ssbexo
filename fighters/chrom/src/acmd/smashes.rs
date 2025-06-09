use super::*;

//Forward Smash ACMD
unsafe extern "C" fn ssbexo_chrom_forward_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 18.0, 361, 70, 0, 50, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword1"), 18.0, 361, 70, 0, 50, 3.5, 0.0, 0.0, 0.7, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword1"), 18.0, 361, 70, 0, 50, 3.5, 0.0, 0.0, 7.2, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword1"), 18.0, 361, 70, 0, 50, 3.5, 0.0, 0.0, 10.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Smash Effect
unsafe extern "C" fn ssbexo_chrom_forward_smash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 4, 16, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 0, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 5, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 10, 90, 0, 0, 0.5, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_chrom_sword1"), Hash40::new("tex_chrom_sword2"), 6, Hash40::new("sword1"), 0, 0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("chrom_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_attack_hi4_slash"), false, false);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("sword1"), 0, 0, 0, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("sword1"), 0, 0, 5, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("sword1"), 0, 0, 10, 90, 0, 0, 0.5, true);
    }
}

//Forward Smash Sound
unsafe extern "C" fn ssbexo_chrom_forward_smash_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_chrom_attack05"));
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_chrom_smash_s01"));
    }
}

//Forward Smash Expression
unsafe extern "C" fn ssbexo_chrom_forward_smash_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 9.0);
    sv_animcmd::execute(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
    frame(agent.lua_state_agent, 55.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

//Up Smash ACMD
unsafe extern "C" fn ssbexo_chrom_up_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 16.0, 90, 80, 0, 40, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword1"), 16.0, 90, 80, 0, 40, 3.5, 0.0, 0.0, 0.7, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword1"), 16.0, 90, 80, 0, 40, 3.5, 0.0, 0.0, 7.2, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword1"), 16.0, 90, 80, 0, 40, 3.5, 0.0, 0.0, 10.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Up Smash Effect
unsafe extern "C" fn ssbexo_chrom_up_smash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 4, 16, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 0, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 5, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 10, 90, 0, 0, 0.5, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_chrom_sword1"), Hash40::new("tex_chrom_sword2"), 4, Hash40::new("sword1"), 0, 0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("chrom_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_attack_hi4_slash"), false, false);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("sword1"), 0, 0, 0, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("sword1"), 0, 0, 5, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("sword1"), 0, 0, 10, 90, 0, 0, 0.5, true);
    }
}

//Up Smash Sound
unsafe extern "C" fn ssbexo_chrom_up_smash_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_chrom_attack06"));
    }
}

//Up Smash Expression
unsafe extern "C" fn ssbexo_chrom_up_smash_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_Z));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 9.0);
    sv_animcmd::execute(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

//Down Smash ACMD
unsafe extern "C" fn ssbexo_chrom_down_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 13.0, 25, 50, 0, 60, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword1"), 13.0, 25, 50, 0, 60, 3.5, 0.0, 0.0, 0.7, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword1"), 13.0, 25, 50, 0, 60, 3.5, 0.0, 0.0, 7.2, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword1"), 13.0, 25, 50, 0, 60, 3.5, 0.0, 0.0, 10.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 12.0, 85, 50, 0, 60, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword1"), 12.0, 85, 50, 0, 60, 3.5, 0.0, 0.0, 0.7, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword1"), 12.0, 85, 50, 0, 60, 3.5, 0.0, 0.0, 7.2, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword1"), 12.0, 85, 50, 0, 60, 3.5, 0.0, 0.0, 10.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 0, 1, 2, 3, 0.0);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Smash Effect
unsafe extern "C" fn ssbexo_chrom_down_smash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 4, 16, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_sword"), Hash40::new("sword1"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 0, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 5, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 10, 90, 0, 0, 0.5, true);
        EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("haver"), 0, -2.5, 0, -90, 0, 0, 0.55, 0, 0, 0, 0, 0, 0, true);
        LAST_PARTICLE_SET_COLOR(agent, 0.4, 0.6, 1);
        LAST_EFFECT_SET_RATE(agent, 1.7);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_sword"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_attack_hi4_slash"), false, false);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("sword1"), 0, 0, 0, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("sword1"), 0, 0, 5, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("sword1"), 0, 0, 10, 90, 0, 0, 0.5, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        if sv_animcmd::get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -15, 0, 3.0, 0, 0, 120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        else {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -15, 0, 3.0, 0, 0, -120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        if sv_animcmd::get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -13, 0, 3.0, 0, 0, 120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        else {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -13, 0, 3.0, 0, 0, -120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }   
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        if sv_animcmd::get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -11, 0, 3.0, 0, 0, 120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        else {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -11, 0, 3.0, 0, 0, -120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }   
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        if sv_animcmd::get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -10, 0, 3.0, 0, 0, 120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        else {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -10, 0, 3.0, 0, 0, -120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }   
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        if sv_animcmd::get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -9, 0, 3.0, 0, 0, 120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        else {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -9, 0, 3.0, 0, 0, -120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }   
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        if sv_animcmd::get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -8, 0, 3.0, 0, 0, 120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        else {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -8, 0, 3.0, 0, 0, -120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }   
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        if sv_animcmd::get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -6, 0, 3.0, 0, 0, 120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        else {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -6, 0, 3.0, 0, 0, -120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }   
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 0, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 5, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, 10, 90, 0, 0, 0.5, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_chrom_sword1"), Hash40::new("tex_chrom_sword2"), 4, Hash40::new("sword1"), 0, 0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("chrom_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        if sv_animcmd::get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -4, 0, 3.0, 0, 0, 120, 1.0, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -2, 0, 3.0, 0, 0, 120, 1.0, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), 0, 0, 3.0, 0, 0, 120, 1.0, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), 2, 0, 3.0, 0, 0, 120, 1.0, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), 4, 0, 3.0, 0, 0, 120, 1.0, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), 6, 0, 3.0, 0, 0, 120, 1.0, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), 8, 0, 3.0, 0, 0, 120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        else {
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -4, 0, 3.0, 0, 0, -120, 1.0, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -2, 0, 3.0, 0, 0, -120, 1.0, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), 0, 0, 3.0, 0, 0, -120, 1.0, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), 2, 0, 3.0, 0, 0, -120, 1.0, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), 4, 0, 3.0, 0, 0, -120, 1.0, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), 6, 0, 3.0, 0, 0, -120, 1.0, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), 8, 0, 3.0, 0, 0, -120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(agent.lua_state_agent, 29.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_attack_hi4_slash"), false, false);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("sword1"), 0, 0, 0, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("sword1"), 0, 0, 5, 90, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_attack_hi4_slash_finish"), Hash40::new("sword1"), 0, 0, 10, 90, 0, 0, 0.5, true);
    }
}

//Down Smash Sound
unsafe extern "C" fn ssbexo_chrom_down_smash_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_chrom_attack07"));
        PLAY_SE(agent, Hash40::new("se_chrom_smash_l01"));
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_chrom_smash_l01"));
    }
}

//Down Smash Expression
unsafe extern "C" fn ssbexo_chrom_down_smash_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 6.0);
    sv_animcmd::execute(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_piercel"), 0);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 16);
    }
}

pub fn install() {
    Agent::new("chrom")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attacks4", ssbexo_chrom_forward_smash_acmd, Low)
    .effect_acmd("effect_attacks4", ssbexo_chrom_forward_smash_effect, Low)
    .sound_acmd("sound_attacks4", ssbexo_chrom_forward_smash_sound, Low)
    .expression_acmd("expression_attacks4", ssbexo_chrom_forward_smash_expression, Low)
    .game_acmd("game_attackhi4", ssbexo_chrom_up_smash_acmd, Low)
    .effect_acmd("effect_attackhi4", ssbexo_chrom_up_smash_effect, Low)
    .sound_acmd("sound_attackhi4", ssbexo_chrom_up_smash_sound, Low)
    .expression_acmd("expression_attackhi4", ssbexo_chrom_up_smash_expression, Low)
    .game_acmd("game_attacklw4", ssbexo_chrom_down_smash_acmd, Low)
    .effect_acmd("effect_attacklw4", ssbexo_chrom_down_smash_effect, Low)
    .sound_acmd("sound_attacklw4", ssbexo_chrom_down_smash_sound, Low)
    .expression_acmd("expression_attacklw4", ssbexo_chrom_down_smash_expression, Low)
    .install()
    ;
}