use super::*;

//Forward Tilt ACMD
unsafe extern "C" fn ssbexo_purin_forward_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 361, 70, 0, 60, 7.0, 0.0, 3.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 361, 70, 0, 60, 7.0, 0.0, 6.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 361, 70, 0, 60, 7.0, 0.0, 8.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Tilt Effect
unsafe extern "C" fn ssbexo_purin_forward_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 3.5, 7, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.75, 0.8);
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 8, -2, 0, 0, 90, 0.7, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.75, 0.8);
        macros::EFFECT_FOLLOW(agent, Hash40::new("rosetta_ring_erase"), Hash40::new("top"), 0, 4, 6, 0, 0, 25, 1, false);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("rosetta_ring_erase"), -1);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 5.0, 9, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.75, 0.8);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 6.5, 11, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.75, 0.8);
        macros::EFFECT_FOLLOW(agent, Hash40::new("rosetta_ring_erase"), Hash40::new("top"), 0, 7, 8, 0, 0, 25, 1, false);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("rosetta_ring_erase"), -1);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 8.0, 9, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.75, 0.8);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 9.5, 7, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.75, 0.8);
        macros::EFFECT_FOLLOW(agent, Hash40::new("rosetta_ring_erase"), Hash40::new("top"), 0, 10, 6, 0, 0, 25, 1, false);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("rosetta_ring_erase"), -1);
    }
}

//Forward Tilt Expression
unsafe extern "C" fn ssbexo_purin_forward_tilt_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_beamm"), 0);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

//Up Tilt ACMD
unsafe extern "C" fn ssbexo_purin_up_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 90, 100, 0, 40, 6.0, 0.0, 12.5, 5.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 90, 100, 0, 40, 6.0, 0.0, 12.5, 5.0, Some(0.0), Some(4.5), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 90, 100, 0, 40, 6.0, 0.0, 12.5, -2.0, Some(0.0), Some(4.5), Some(-4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Up Tilt Effect
unsafe extern "C" fn ssbexo_purin_up_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 12.0, 0, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.75, 0.8);
        macros::EFFECT_FOLLOW(agent, Hash40::new("rosetta_ring_erase"), Hash40::new("top"), 0, 10, 0, 0, 0, 25, 1, false);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("rosetta_ring_erase"), -1);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 11.0, 1.5, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.75, 0.8);
        macros::EFFECT_FOLLOW(agent, Hash40::new("rosetta_ring_erase"), Hash40::new("top"), 0, 9, 1.5, 0, 0, 25, 1, false);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("rosetta_ring_erase"), -1);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 11.0, -1.5, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.75, 0.8);
        macros::EFFECT_FOLLOW(agent, Hash40::new("rosetta_ring_erase"), Hash40::new("top"), 0, 9, -1.5, 0, 0, 25, 1, false);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("rosetta_ring_erase"), -1);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 10.0, 3.0, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.75, 0.8);
        macros::EFFECT_FOLLOW(agent, Hash40::new("rosetta_ring_erase"), Hash40::new("top"), 0, 8, 3.0, 0, 0, 25, 1, false);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("rosetta_ring_erase"), -1);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 10.0, -3.0, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.75, 0.8);
        macros::EFFECT_FOLLOW(agent, Hash40::new("rosetta_ring_erase"), Hash40::new("top"), 0, 8, -3.0, 0, 0, 25, 1, false);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("rosetta_ring_erase"), -1);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 8.0, 4.0, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.75, 0.8);
        macros::EFFECT_FOLLOW(agent, Hash40::new("rosetta_ring_erase"), Hash40::new("top"), 0, 6, 4.0, 0, 0, 25, 1, false);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("rosetta_ring_erase"), -1);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 8.0, -4.0, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.75, 0.8);
        macros::EFFECT_FOLLOW(agent, Hash40::new("rosetta_ring_erase"), Hash40::new("top"), 0, 6, -4.0, 0, 0, 25, 1, false);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("rosetta_ring_erase"), -1);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 7.0, 4.5, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.75, 0.8);
        macros::EFFECT_FOLLOW(agent, Hash40::new("rosetta_ring_erase"), Hash40::new("top"), 0, 5, 4.5, 0, 0, 25, 1, false);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("rosetta_ring_erase"), -1);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 7.0, -4.5, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.75, 0.8);
        macros::EFFECT_FOLLOW(agent, Hash40::new("rosetta_ring_erase"), Hash40::new("top"), 0, 5, -4.5, 0, 0, 25, 1, false);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("rosetta_ring_erase"), -1);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 6.0, 4.0, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.75, 0.8);
        macros::EFFECT_FOLLOW(agent, Hash40::new("rosetta_ring_erase"), Hash40::new("top"), 0, 4, 4.0, 0, 0, 25, 1, false);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("rosetta_ring_erase"), -1);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 6.0, -4.0, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.75, 0.8);
        macros::EFFECT_FOLLOW(agent, Hash40::new("rosetta_ring_erase"), Hash40::new("top"), 0, 4, -4.0, 0, 0, 25, 1, false);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("rosetta_ring_erase"), -1);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 4.0, 4.0, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.75, 0.8);
        macros::EFFECT_FOLLOW(agent, Hash40::new("rosetta_ring_erase"), Hash40::new("top"), 0, 2, 4.0, 0, 0, 25, 1, false);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("rosetta_ring_erase"), -1);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 4.0, -4.0, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.75, 0.8);
        macros::EFFECT_FOLLOW(agent, Hash40::new("rosetta_ring_erase"), Hash40::new("top"), 0, 2, -4.0, 0, 0, 25, 1, false);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("rosetta_ring_erase"), -1);
    }
}

//Up Tilt Sound
unsafe extern "C" fn ssbexo_purin_up_tilt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_purin_rnd_attack02"));
        macros::PLAY_SE(agent, Hash40::new("se_purin_swing_l"));
    }
}

//Up Tilt Expression
unsafe extern "C" fn ssbexo_purin_up_tilt_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_beamm"), 0);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

//Down Tilt ACMD
unsafe extern "C" fn ssbexo_purin_down_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("footl"), 10.0, 20, 68, 0, 50, 3.0, 0.0, -8.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("footl"), 10.0, 20, 68, 0, 50, 3.0, 0.0, -4.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("footl"), 10.0, 20, 68, 0, 50, 3.0, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 10.0, 20, 68, 0, 50, 5.0, 0.0, 2.5, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Tilt Effect
unsafe extern "C" fn ssbexo_purin_down_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 4, 0, -3, 0, 0, 0.6, true, *EF_FLIP_YZ);
        macros::LAST_PARTICLE_SET_COLOR(agent, 1, 1, 0.5);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 2.0, 12, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.75, 0.8);
        macros::EFFECT_FOLLOW(agent, Hash40::new("rosetta_ring_erase"), Hash40::new("top"), 0, 2, 12, 0, 0, 25, 1, false);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("rosetta_ring_erase"), -1);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 4.5, 10.5, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.6);
    }
}

//Down Tilt Expression
unsafe extern "C" fn ssbexo_purin_down_tilt_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_beamm"), 0);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 12);
    }
}

pub fn install() {
    Agent::new("purin")
    .game_acmd("game_attacks3", ssbexo_purin_forward_tilt_acmd)
    .game_acmd("game_attacks3hi", ssbexo_purin_forward_tilt_acmd)
    .game_acmd("game_attacks3lw", ssbexo_purin_forward_tilt_acmd)
    .effect_acmd("effect_attacks3", ssbexo_purin_forward_tilt_effect)
    .effect_acmd("effect_attacks3hi", ssbexo_purin_forward_tilt_effect)
    .effect_acmd("effect_attacks3lw", ssbexo_purin_forward_tilt_effect)
    .expression_acmd("expression_attacks3", ssbexo_purin_forward_tilt_expression)
    .expression_acmd("expression_attacks3hi", ssbexo_purin_forward_tilt_expression)
    .expression_acmd("expression_attacks3lw", ssbexo_purin_forward_tilt_expression)
    .game_acmd("game_attackhi3", ssbexo_purin_up_tilt_acmd)
    .effect_acmd("effect_attackhi3", ssbexo_purin_up_tilt_effect)
    .sound_acmd("sound_attackhi3", ssbexo_purin_up_tilt_sound)
    .expression_acmd("expression_attackhi3", ssbexo_purin_up_tilt_expression)
    .game_acmd("game_attacklw3", ssbexo_purin_down_tilt_acmd)
    .effect_acmd("effect_attacklw3", ssbexo_purin_down_tilt_effect)
    .expression_acmd("expression_attacklw3", ssbexo_purin_down_tilt_expression)
    .install()
    ;
}