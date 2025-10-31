use super::*;

//Forward Smash Charge Effect
unsafe extern "C" fn ssbexo_roy_forward_smash_charge_effect(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("roy_fire"), false, false);
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 5.0);
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword1"), 0, -0.0, 8, 0, 0, 0, 1, 2, 2, 4, 0, 0, 0, true);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
}

//Forward Smash ACMD
unsafe extern "C" fn ssbexo_roy_forward_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 12.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 14.0);
    FT_MOTION_RATE(agent, 1.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 361, 75, 0, 60, 3.3, 0.0, 7.7, 9.1, Some(0.0), Some(7.7), Some(7.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword1"), 17.0, 361, 75, 0, 60, 4.0, 2.0, 1.0, 1.5, Some(14.0), Some(1.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("sword1"), 12.0, 361, 70, 0, 30, 3.5, 2.0, 1.0, 7.5, Some(9.5), Some(1.0), Some(7.5), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            ATTACK(agent, 3, 0, Hash40::new("sword1"), 12.0, 361, 70, 0, 30, 3.5, 2.0, 1.0, 7.5, Some(2.0), Some(1.0), Some(19.5), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            ATTACK(agent, 4, 0, Hash40::new("sword1"), 12.0, 361, 70, 0, 30, 3.5, 2.0, 1.0, 7.5, Some(2.5), Some(1.0), Some(24.5), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
    }
    else {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 361, 75, 0, 60, 3.3, 0.0, 7.7, 9.1, Some(0.0), Some(7.7), Some(7.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword1"), 17.0, 361, 75, 0, 60, 4.0, 2.0, 1.0, 1.5, Some(14.0), Some(1.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("sword1"), 12.0, 361, 70, 0, 30, 3.5, 2.0, 1.0, 7.5, Some(9.5), Some(1.0), Some(7.5), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Smash Effect
unsafe extern "C" fn ssbexo_roy_forward_smash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 4, 16, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 12.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
        if is_excute(agent) {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 5, Hash40::new("sword1"), 0, 0, -0.8, Hash40::new("sword1"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 5, Hash40::new("sword1"), 0, 0, -0.8, Hash40::new("sword1"), -0.0, -0.0, 26.0, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 7, 0, 0, 0, 1, true);
        }
    }
    else {
        if is_excute(agent) {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 5, Hash40::new("sword1"), 0, 0, -0.8, Hash40::new("sword1"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 6);
        EFFECT_OFF_KIND(agent, Hash40::new("roy_fire"), false, false);
    }
}

//Up Smash Charge Effect
unsafe extern "C" fn ssbexo_roy_up_smash_charge_effect(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("roy_fire"), false, false);
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 5.0);
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword1"), 0, -0.0, 8, 0, 0, 0, 1, 2, 2, 4, 0, 0, 0, true);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
}

//Down Smash Charge Effect
unsafe extern "C" fn ssbexo_roy_down_smash_charge_effect(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("roy_fire"), false, false);
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 5.0);
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword1"), 0, -0.0, 8, 0, 0, 0, 1, 2, 2, 4, 0, 0, 0, true);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
}

//Down Smash ACMD
unsafe extern "C" fn ssbexo_roy_down_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 17.0, 361, 85, 0, 42, 3.3, 0.0, 4.5, 13.0, Some(0.0), Some(6.0), Some(7.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword1"), 10.0, 361, 90, 0, 15, 3.5, 0.0, 0.0, 2.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword1"), 10.0, 361, 90, 0, 15, 2.8, 0.0, 0.0, 7.2, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 17.0, 361, 85, 0, 42, 3.3, 0.0, 4.1, -11.0, Some(0.0), Some(4.5), Some(-8.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword1"), 10.0, 361, 90, 0, 15, 3.5, 0.0, 0.0, 2.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword1"), 10.0, 361, 90, 0, 15, 2.8, -1.6, 0.0, 7.2, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Smash Effect
unsafe extern "C" fn ssbexo_roy_down_smash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), -0.0, -0.0, 7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 10, Hash40::new("sword1"), 0, 0, -0.8, Hash40::new("sword1"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
        EFFECT_FOLLOW(agent, Hash40::new("roy_sword_light"), Hash40::new("sword1"), 0, 0, 10.55, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.4);
        EFFECT_FOLLOW(agent, Hash40::new("roy_sword_light"), Hash40::new("sword1"), 0, 0, 7, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.4);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
        EFFECT_OFF_KIND(agent, Hash40::new("roy_sword_light"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("roy_sword"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("roy_fire"), false, false);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Down Smash Sound
unsafe extern "C" fn ssbexo_roy_down_smash_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_roy_rnd_attack_smash_l"));
        PLAY_SE(agent, Hash40::new("se_roy_smash_l01"));
    }
    wait(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_roy_smash_l01"));
    }
}

//Down Smash Expression
unsafe extern "C" fn ssbexo_roy_down_smash_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 4.0);
    sv_animcmd::execute(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashll"), 0);
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 1);
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 2);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashll"), 0);
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 1);
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 2);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 16);
    }
}

pub fn install() {
    Agent::new("roy")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .effect_acmd("effect_attacks4charge", ssbexo_roy_forward_smash_charge_effect, Low)
    .game_acmd("game_attacks4", ssbexo_roy_forward_smash_acmd, Low)
    .effect_acmd("effect_attacks4", ssbexo_roy_forward_smash_effect, Low)
    .effect_acmd("effect_attackhi4charge", ssbexo_roy_up_smash_charge_effect, Low)
    .effect_acmd("effect_attacklw4charge", ssbexo_roy_down_smash_charge_effect, Low)
    .game_acmd("game_attacklw4", ssbexo_roy_down_smash_acmd, Low)
    .effect_acmd("effect_attacklw4", ssbexo_roy_down_smash_effect, Low)
    .sound_acmd("sound_attacklw4", ssbexo_roy_down_smash_sound, Low)
    .expression_acmd("expression_attacklw4", ssbexo_roy_down_smash_expression, Low)
    .install()
    ;
}