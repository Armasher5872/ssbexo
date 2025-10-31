use super::*;

//Forward Smash ACMD
unsafe extern "C" fn ssbexo_donkey_forward_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 22.0, 46, 88, 0, 21, 5.7, 0.0, 6.4, 24.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 21.0, 361, 86, 0, 18, 3.7, 0.0, 6.4, 18.0, Some(0.0), Some(6.4), Some(10.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 27.0);
    if is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

//Up Smash ACMD
unsafe extern "C" fn ssbexo_donkey_up_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 19.0, 90, 84, 0, 40, 8.8, 0.0, 28.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 19.0, 90, 84, 0, 40, 7.0, 0.0, 18.0, 5.0, Some(0.0), Some(18.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

//Up Smash Effect
unsafe extern "C" fn ssbexo_donkey_up_smash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("donkey_smash_slap"), Hash40::new("donkey_smash_slap"), Hash40::new("top"), 0, 22, 5, -20, 150, 80, 1.4, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 2.6);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("donkey_smash_slap"), Hash40::new("donkey_smash_slap"), Hash40::new("top"), 0, 22, -5, -150, -150, 77, 1.4, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 2.6);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 30, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 360, false);
    }
}

//Down Smash ACMD
unsafe extern "C" fn ssbexo_donkey_down_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 270, 30, 45, 0, 6.0, 0.0, 3.0, -10.0, Some(0.0), Some(3.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 300, 20, 100, 0, 6.0, 0.0, 3.0, -24.0, Some(0.0), Some(3.0), Some(24.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 78, 30, 0, 100, 6.0, 0.0, 3.0, -24.0, Some(0.0), Some(3.0), Some(24.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 16.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 16.0, false);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 84, 85, 0, 65, 6.0, 0.0, 3.0, -10.0, Some(0.0), Some(3.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 84, 70, 0, 45, 6.0, 0.0, 3.0, -24.0, Some(0.0), Some(3.0), Some(24.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 78, 30, 0, 100, 6.0, 0.0, 3.0, -24.0, Some(0.0), Some(3.0), Some(24.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Smash Effect
unsafe extern "C" fn ssbexo_donkey_down_smash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("donkey_handslap"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("donkey_handslap"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 43.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}

//Down Smash Sound
unsafe extern "C" fn ssbexo_donkey_down_smash_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start"));
        PLAY_SE(agent, Hash40::new("vc_donkey_attack07"));
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_donkey_special_l01"));
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_donkey_special_l02"));
    }
}

//Down Smash Expression
unsafe extern "C" fn ssbexo_donkey_down_smash_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 7.0);
    sv_animcmd::execute(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 38.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
}

pub fn install() {
    Agent::new("donkey")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attacks4", ssbexo_donkey_forward_smash_acmd, Low)
    .game_acmd("game_attackhi4", ssbexo_donkey_up_smash_acmd, Low)
    .effect_acmd("effect_attackhi4", ssbexo_donkey_up_smash_effect, Low)
    .game_acmd("game_attacklw4", ssbexo_donkey_down_smash_acmd, Low)
    .effect_acmd("effect_attacklw4", ssbexo_donkey_down_smash_effect, Low)
    .sound_acmd("sound_attacklw4", ssbexo_donkey_down_smash_sound, Low)
    .expression_acmd("expression_attacklw4", ssbexo_donkey_down_smash_expression, Low)
    .install()
    ;
}