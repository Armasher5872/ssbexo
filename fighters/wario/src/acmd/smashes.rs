use super::*;

//Forward Smash ACMD
unsafe extern "C" fn ssbexo_wario_forward_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("handr"), 17.0, 361, 80, 0, 34, 5.7, 0.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 17.0, 361, 80, 0, 34, 4.3, 0.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("shoulderr"), 17.0, 361, 80, 0, 34, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Smash Effect
unsafe extern "C" fn ssbexo_wario_forward_smash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 8, -12, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("wario_smash_line"), Hash40::new("wario_smash_line"), Hash40::new("top"), 1, 13, -12.5, 15, -10, 0, 1.6, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 1, 7, 12.5, 0, 0, 0, 1.7, 0, 0, 0, 0, 0, 360, true, 1.5);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

//Forward Smash Sound
unsafe extern "C" fn ssbexo_wario_forward_smash_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start"));
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_wario_attack07"));
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_smash_s01"));
    }
}

//Forward Smash Expression
unsafe extern "C" fn ssbexo_wario_forward_smash_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(agent.lua_state_agent, 8.0);
    execute(agent.lua_state_agent, 8.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if is_excute(agent) {
            ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        }
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 1);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(agent.lua_state_agent, 60.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

//Down Smash Charge Effect
unsafe extern "C" fn ssbexo_wario_down_smash_charge_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 5.0);
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("handr"), 5, 0, 0, 0, 0, 0, 1, 3, 3, 3, 0, 0, 0, true);
    }
}

//Down Smash ACMD
unsafe extern "C" fn ssbexo_wario_down_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("handr"), 16.7, 270, 80, 0, 20, 5.7, 2.3, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 16.7, 270, 80, 0, 20, 3.0, -0.5, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("top"), 7.4, 68, 82, 0, 27, 8.0, 0.0, 4.0, 3.0, Some(0.0), Some(0.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(agent, 3, 0, Hash40::new("top"), 7.4, 68, 82, 0, 27, 8.0, 0.0, 4.0, -3.0, Some(0.0), Some(0.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Smash Effect
unsafe extern "C" fn ssbexo_wario_down_smash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 17.0);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wario_shock"), Hash40::new("top"), 3, 0, -4, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
            EFFECT(agent, Hash40::new("wario_ground_crack"), Hash40::new("top"), 3, 0, -7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            let wario_quake = EffectModule::req_follow(agent.module_accessor, Hash40::new("wario_quake"), Hash40::new("top"), &Vector3f{x: 5.0, y: 0.0, z: -2.5}, &Vector3f::zero(), 0.5, true, 0, 0, 0, 0, 0, true, true);
            EffectModule::set_scale(agent.module_accessor, wario_quake as u32, &Vector3f{x: 1.0, y: 0.5, z: 1.35});
            EffectModule::set_rate(agent.module_accessor, wario_quake as u32, 2.0);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wario_shock"), Hash40::new("top"), -3, 0, -4, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
            EFFECT(agent, Hash40::new("wario_ground_crack"), Hash40::new("top"), -3, 0, -7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            let wario_quake = EffectModule::req_follow(agent.module_accessor, Hash40::new("wario_quake"), Hash40::new("top"), &Vector3f{x: -5.0, y: 0.0, z: -2.5}, &Vector3f::zero(), 0.5, true, 0, 0, 0, 0, 0, true, true);
            EffectModule::set_scale(agent.module_accessor, wario_quake as u32, &Vector3f{x: 1.0, y: 0.5, z: 1.35});
            EffectModule::set_rate(agent.module_accessor, wario_quake as u32, 2.0);
        }
    }
}

//Down Smash Sound
unsafe extern "C" fn ssbexo_wario_down_smash_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start"));
        PLAY_SE(agent, Hash40::new("se_wario_attackair_l01"));
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        let rand = sv_math::randf(hash40("fighter"), 100.0);
        if rand > 66.0 {
            PLAY_SE(agent, Hash40::new("vc_wario_attack05"));
        }
        else if rand <= 66.0 && rand > 33.0 {
            let final_sfx = SoundModule::play_se(agent.module_accessor, Hash40::new("vc_wario_final03"), true, false, false, false, smash::app::enSEType(0));
            SoundModule::set_se_vol(agent.module_accessor, final_sfx as i32, 0.75, 0);
        }
        else {
            PLAY_SE(agent, Hash40::new("vc_wario_attack06"));
        }
        PLAY_SE(agent, Hash40::new("se_wario_smash_l01"));
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_special_s06"));
    }
}

//Down Smash Expression
unsafe extern "C" fn ssbexo_wario_down_smash_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 6.0);
    execute(agent.lua_state_agent, 6.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if is_excute(agent) {
            ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        }
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
    frame(agent.lua_state_agent, 70.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attacks4", ssbexo_wario_forward_smash_acmd, Low)
    .effect_acmd("effect_attacks4", ssbexo_wario_forward_smash_effect, Low)
    .sound_acmd("sound_attacks4", ssbexo_wario_forward_smash_sound, Low)
    .expression_acmd("expression_attacks4", ssbexo_wario_forward_smash_expression, Low)
    .effect_acmd("effect_attacklw4charge", ssbexo_wario_down_smash_charge_effect, Low)
    .game_acmd("game_attacklw4", ssbexo_wario_down_smash_acmd, Low)
    .effect_acmd("effect_attacklw4", ssbexo_wario_down_smash_effect, Low)
    .sound_acmd("sound_attacklw4", ssbexo_wario_down_smash_sound, Low)
    .expression_acmd("expression_attacklw4", ssbexo_wario_down_smash_expression, Low)
    .install()
    ;
}