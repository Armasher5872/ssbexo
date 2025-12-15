use super::*;

//Forward Tilt ACMD
unsafe extern "C" fn ssbexo_wario_forward_tilt_acmd(agent: &mut L2CAgentBase) {
    FT_MOTION_RATE(agent, 10.0/12.0);
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        HIT_NODE(agent, Hash40::new("handl"), *HIT_STATUS_XLU);
        ATTACK(agent, 0, 0, Hash40::new("handl"), 13.0, 44, 86, 0, 20, 5.7, 2.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 11.0, 44, 86, 0, 20, 3.0, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("handl"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Up Tilt ACMD
unsafe extern "C" fn ssbexo_wario_up_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("handl"), *HIT_STATUS_XLU);
        ATTACK(agent, 0, 0, Hash40::new("handl"), 7.1, 74, 64, 0, 46, 5.7, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 7.1, 74, 64, 0, 46, 4.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("shoulderl"), 7.1, 74, 64, 0, 46, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("handl"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Up Tilt Effect
unsafe extern "C" fn ssbexo_wario_up_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
	if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
	    LAST_EFFECT_SET_RATE(agent, 0.4);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_c"), Hash40::new("sys_attack_arc_c"), Hash40::new("top"), 2, 11, 2, -12, 16, 95, 0.9, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 10.0);
	if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true, 0.9);
        LAST_EFFECT_SET_RATE(agent, 2.0);
    }
}

//Up Tilt Sound
unsafe extern "C" fn ssbexo_wario_up_tilt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_l"));
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        let rand = sv_math::randf(hash40("fighter"), 100.0);
        if rand > 80.0 {
            PLAY_SE(agent, Hash40::new("vc_wario_attack04"));
        }
        else if rand <= 80.0 && rand > 60.0 {
            PLAY_SE(agent, Hash40::new("vc_wario_attack03"));
        }
        else if rand <= 60.0 && rand > 40.0 {
            PLAY_SE(agent, Hash40::new("vc_wario_attack01"));
        }
        else if rand <= 40.0 && rand > 20.0 {
            PLAY_SE(agent, Hash40::new("vc_wario_007"));
        }
    }
}

//Up Tilt Expression
unsafe extern "C" fn ssbexo_wario_up_tilt_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

//Down Tilt ACMD
unsafe extern "C" fn ssbexo_wario_down_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("handl"), *HIT_STATUS_XLU);
        ATTACK(agent, 0, 0, Hash40::new("handl"), 8.6, 44, 56, 0, 22, 5.7, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 8.6, 44, 56, 0, 22, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("shoulderl"), 8.6, 44, 56, 0, 22, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("handl"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Tilt Effect
unsafe extern "C" fn ssbexo_wario_down_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 3, 7, 1.5, 0, 0, 0, 1, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Down Tilt Sound
unsafe extern "C" fn ssbexo_wario_down_tilt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_swing_l"));
        let rand = sv_math::randf(hash40("fighter"), 100.0);
        if rand > 50.0 {
            PLAY_SE(agent, Hash40::new("vc_wario_attack02"));
        }
        else {
            PLAY_SE(agent, Hash40::new("vc_wario_attack03"));
        }
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attacks3", ssbexo_wario_forward_tilt_acmd, Low)
    .game_acmd("game_attacks3hi", ssbexo_wario_forward_tilt_acmd, Low)
    .game_acmd("game_attacks3lw", ssbexo_wario_forward_tilt_acmd, Low)
    .game_acmd("game_attackhi3", ssbexo_wario_up_tilt_acmd, Low)
    .effect_acmd("effect_attackhi3", ssbexo_wario_up_tilt_effect, Low)
    .sound_acmd("sound_attackhi3", ssbexo_wario_up_tilt_sound, Low)
    .expression_acmd("expression_attackhi3", ssbexo_wario_up_tilt_expression, Low)
    .game_acmd("game_attacklw3", ssbexo_wario_down_tilt_acmd, Low)
    .effect_acmd("effect_attacklw3", ssbexo_wario_down_tilt_effect, Low)
    .sound_acmd("sound_attacklw3", ssbexo_wario_down_tilt_sound, Low)
    .install()
    ;
}