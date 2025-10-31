use super::*;

//Forward Smash Effect
unsafe extern "C" fn ssbexo_captain_forward_smash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("captain_fn_flash"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.5, true, 1.0);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_smash_line"), Hash40::new("top"), 0, 10, -16, 0, 0, 0, 1.2, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

//Forward Smash Hi Effect
unsafe extern "C" fn ssbexo_captain_forward_smash_hi_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("captain_fn_flash"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.5, true, 1.0);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_smash_line"), Hash40::new("top"), 0, 2, -16, -25, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

//Forward Smash Lw Effect
unsafe extern "C" fn ssbexo_captain_forward_smash_lw_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("captain_fn_flash"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.5, true, 1.0);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_smash_line"), Hash40::new("top"), 0, 16, -12, 25, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("captain_smash_line"), true, true);
    }
}

//Up Smash ACMD
unsafe extern "C" fn ssbexo_captain_up_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 110, 90, 150, 0, 5.7, 0.0, 7.5, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 96, 90, 80, 0, 6.0, 0.0, 17.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 3, 0, Hash40::new("top"), 12.0, 80, 90, 10, 5, 4.8, 0.0, 21.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 80, 90, 0, 0, 4.8, 0.0, 28.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_vec_target_pos(agent.module_accessor, 2, Hash40::new("top"), &Vector2f{x: 5.0, y: 29.0}, 7, false);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 83, 80, 0, 70, 6.0, 0.0, 29.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 85, 81, 0, 70, 5.5, 0.0, 21.0, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("top"), 13.0, 70, 88, 0, 70, 5.0, 0.0, 16.0, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Up Smash Effect
unsafe extern "C" fn ssbexo_captain_up_smash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("toer"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("captain_fn_flash"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 0.5, true, 1.0);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("captain_fn_flash"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 0.5, true, 1.0);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_smash_arc"), Hash40::new("top"), 0, 18, 2, 68, 0, 151, 1, true);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_smash_arc"), Hash40::new("top"), 0, 17, 1.5, 73, -15, 165, 1.05, true);
    }
    frame(agent.lua_state_agent, 33.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

//Down Smash Charge Effect
unsafe extern "C" fn ssbexo_captain_down_smash_charge_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 12, 0, 4, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 5.0);
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, false);
    }
}

//Down Smash ACMD
unsafe extern "C" fn ssbexo_captain_down_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneel"), 14.0, 28, 90, 0, 30, 4.5, 4.9, -0.9, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 14.0, 28, 90, 0, 30, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("legl"), 14.0, 28, 90, 0, 30, 3.7, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneel"), 14.0, 28, 90, 0, 30, 4.5, 4.9, -0.9, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 14.0, 28, 90, 0, 30, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("legl"), 14.0, 28, 90, 0, 30, 3.7, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Smash Effect
unsafe extern "C" fn ssbexo_captain_down_smash_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("captain_fn_flash"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 0.5, true, 1.0);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

//Down Smash Sound
unsafe extern "C" fn ssbexo_captain_down_smash_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start"));
        PLAY_SE(agent, Hash40::new("vc_captain_attack07"));
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_smashswing_03"));
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_swing_l"));
        PLAY_SE(agent, Hash40::new("se_captain_leather_creak01"));
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_smashswing_03"));
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_swing_l"));
        PLAY_SE(agent, Hash40::new("se_captain_leather_creak01"));
    }
}

//Down Smash Expression
unsafe extern "C" fn ssbexo_captain_down_smash_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(agent.lua_state_agent, 3.0);
    execute(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

pub fn install() {
    Agent::new("captain")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .effect_acmd("effect_attacks4", ssbexo_captain_forward_smash_effect, Low)
    .effect_acmd("effect_attacks4hi", ssbexo_captain_forward_smash_hi_effect, Low)
    .effect_acmd("effect_attacks4lw", ssbexo_captain_forward_smash_lw_effect, Low)
    .game_acmd("game_attackhi4", ssbexo_captain_up_smash_acmd, Low)
    .effect_acmd("effect_attackhi4", ssbexo_captain_up_smash_effect, Low)
    .effect_acmd("effect_attacklw4charge", ssbexo_captain_down_smash_charge_effect, Low)
    .game_acmd("game_attacklw4", ssbexo_captain_down_smash_acmd, Low)
    .effect_acmd("effect_attacklw4", ssbexo_captain_down_smash_effect, Low)
    .sound_acmd("sound_attacklw4", ssbexo_captain_down_smash_sound, Low)
    .expression_acmd("expression_attacklw4", ssbexo_captain_down_smash_expression, Low)
    .install()
    ;
}