use super::*;

//Forward Tilt ACMD
unsafe extern "C" fn ssbexo_mariod_forward_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 11.0, 25, 70, 0, 50, 3.5, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("toel"), 11.0, 25, 70, 0, 50, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Tilt Effect
unsafe extern "C" fn ssbexo_mariod_forward_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), -3, 5, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 5.5, 8.5, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true, 0.8);
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("footl"), 1.0, 0, 0, 0, 0, 0, 0.3, true, *EF_FLIP_YZ);
    }
}

//Forward Tilt Sound
unsafe extern "C" fn ssbexo_mariod_forward_tilt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_m"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_mariod_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark2"));
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark1"));
    }
}

//Forward Tilt Expression
unsafe extern "C" fn ssbexo_mariod_forward_tilt_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

//Up Tilt ACMD
unsafe extern "C" fn ssbexo_mariod_up_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("head"), 8.0, 102, 130, 0, 15, 3.5, -0.5, -0.8, 0.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("arml"), 8.0, 102, 130, 0, 15, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("arml"), 8.0, 102, 130, 0, 15, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Up Tilt Effect
unsafe extern "C" fn ssbexo_mariod_up_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_atkhi3_arc"), Hash40::new("mariod_atkhi3_arc"), Hash40::new("top"), 2, 12, 1, 0, 4, 103, 1, true, *EF_FLIP_YZ);
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("handl"), 1.0, 0, 0, 0, 0, 0, 0.3, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

//Up Tilt Sound
unsafe extern "C" fn ssbexo_mariod_up_tilt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_m"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_mariod_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark2"));
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark1"));
    }
}

//Down Tilt ACMD
unsafe extern "C" fn ssbexo_mariod_down_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 9.0, 140, 55, 0, 35, 3.5, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("toel"), 9.0, 140, 55, 0, 35, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Tilt Effect
unsafe extern "C" fn ssbexo_mariod_down_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 3, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), -2, 3, 4, -1, 12, -172, 0.95, true, *EF_FLIP_YZ);
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("footl"), 1.0, 0, 0, 0, 0, 0, 0.3, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_attack_arc"), false, true);
    }
}

//Down Tilt Sound
unsafe extern "C" fn ssbexo_mariod_down_tilt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_m"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_mariod_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark2"));
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark1"));
    }
}

pub fn install() {
    Agent::new("mariod")
    .game_acmd("game_attacks3", ssbexo_mariod_forward_tilt_acmd, Priority::Low)
    .effect_acmd("effect_attacks3", ssbexo_mariod_forward_tilt_effect, Priority::Low)
    .sound_acmd("sound_attacks3", ssbexo_mariod_forward_tilt_sound, Priority::Low)
    .expression_acmd("expression_attacks3", ssbexo_mariod_forward_tilt_expression, Priority::Low)
    .game_acmd("game_attacks3hi", ssbexo_mariod_forward_tilt_acmd, Priority::Low)
    .effect_acmd("effect_attacks3hi", ssbexo_mariod_forward_tilt_effect, Priority::Low)
    .sound_acmd("sound_attacks3hi", ssbexo_mariod_forward_tilt_sound, Priority::Low)
    .expression_acmd("expression_attacks3hi", ssbexo_mariod_forward_tilt_expression, Priority::Low)
    .game_acmd("game_attacks3lw", ssbexo_mariod_forward_tilt_acmd, Priority::Low)
    .effect_acmd("effect_attacks3lw", ssbexo_mariod_forward_tilt_effect, Priority::Low)
    .sound_acmd("sound_attacks3lw", ssbexo_mariod_forward_tilt_sound, Priority::Low)
    .expression_acmd("expression_attacks3lw", ssbexo_mariod_forward_tilt_expression, Priority::Low)
    .game_acmd("game_attackhi3", ssbexo_mariod_up_tilt_acmd, Priority::Low)
    .effect_acmd("effect_attackhi3", ssbexo_mariod_up_tilt_effect, Priority::Low)
    .sound_acmd("sound_attackhi3", ssbexo_mariod_up_tilt_sound, Priority::Low)
    .game_acmd("game_attacklw3", ssbexo_mariod_down_tilt_acmd, Priority::Low)
    .effect_acmd("effect_attacklw3", ssbexo_mariod_down_tilt_effect, Priority::Low)
    .sound_acmd("sound_attacklw3", ssbexo_mariod_down_tilt_sound, Priority::Low)
    .install()
    ;
}