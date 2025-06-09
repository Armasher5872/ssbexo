use super::*;

//Forward Tilt ACMD
unsafe extern "C" fn ssbexo_mewtwo_forward_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 361, 80, 0, 70, 6.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 361, 80, 0, 70, 6.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(17.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 361, 80, 0, 70, 6.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 361, 80, 0, 70, 6.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(23.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 361, 80, 0, 70, 6.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(26.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 361, 80, 0, 70, 6.0, 0.0, 11.0, 8.0, Some(0.0), Some(11.0), Some(29.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Tilt Effect
unsafe extern "C" fn ssbexo_mewtwo_forward_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("top"), 0, 10, 10.5, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("top"), 0, 12, 10.5, 20, 0, 0, 1.0, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_aura"), Hash40::new("top"), 0, 12, 10.5, 20, 0, 0, 1.0, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_mball_beam"), Hash40::new("top"), 0, 10, 10.5, -190, 0, 0, 1.25, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 1.0);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("top"), 0, 12, 14.5, 20, 0, 0, 1.0, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_aura"), Hash40::new("top"), 0, 12, 14.5, 20, 0, 0, 1.0, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("top"), 0, 12, 18.5, 20, 0, 0, 1.0, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_aura"), Hash40::new("top"), 0, 12, 18.5, 20, 0, 0, 1.0, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_deathscythe_trace"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_deathscythe_aura"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_shadowball_hold"), false, false);
    }
}

//Forward Tilt Sound
unsafe extern "C" fn ssbexo_mewtwo_forward_tilt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mewtwo_attackhard_s01"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_mewtwo_rnd_attack"));
    }
}

//Forward Tilt Expression
unsafe extern "C" fn ssbexo_mewtwo_forward_tilt_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

//Up Tilt ACMD
unsafe extern "C" fn ssbexo_mewtwo_up_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("s_tail3"), 10.0, 110, 55, 0, 55, 4.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        ATTACK(agent, 1, 0, Hash40::new("s_tail5"), 9.0, 110, 55, 0, 55, 3.9, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        ATTACK(agent, 2, 0, Hash40::new("s_tail7"), 8.0, 110, 55, 0, 55, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 3, 0, Hash40::new("top"), 10.0, 110, 55, 0, 55, 6.0, 0.0, 10.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 3, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Up Tilt Effect
unsafe extern "C" fn ssbexo_mewtwo_up_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_2"), 4, Hash40::new("s_tail6"), 0.0, 0.0, 0.0, Hash40::new("s_tail7"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail7"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail5"), 0.0, 0.0, 0.0, Hash40::new("s_tail6"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail6"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail4"), 0.0, 0.0, 0.0, Hash40::new("s_tail5"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail5"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail3"), 0.0, 0.0, 0.0, Hash40::new("s_tail4"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail4"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail2"), 0.0, 0.0, 0.0, Hash40::new("s_tail3"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail3"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail1"), 0.0, 0.0, 0.0, Hash40::new("s_tail2"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail2"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail7"), 0, 0, 0, 0, 0, 0, 1.0, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail5"), 0, 0, 0, 0, 0, 0, 0.8, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail3"), 0, 0, 0, 0, 0, 0, 0.6, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail1"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_deathscythe_trace"), false, false);
    }
}

//Down Tilt ACMD
unsafe extern "C" fn ssbexo_mewtwo_down_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("s_tail3"), 9.0, 80, 40, 0, 60, 4.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        ATTACK(agent, 1, 0, Hash40::new("s_tail5"), 8.5, 90, 45, 0, 65, 3.8, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        ATTACK(agent, 2, 0, Hash40::new("s_tail7"), 8.0, 100, 50, 0, 60, 3.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        ATK_SET_SHIELD_SETOFF_MUL_arg4(agent, 0, 1, 2, 0.1);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Tilt Effect
unsafe extern "C" fn ssbexo_mewtwo_down_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_2"), 4, Hash40::new("s_tail6"), 0.0, 0.0, 0.0, Hash40::new("s_tail7"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail7"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail5"), 0.0, 0.0, 0.0, Hash40::new("s_tail6"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail6"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail4"), 0.0, 0.0, 0.0, Hash40::new("s_tail5"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail5"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail3"), 0.0, 0.0, 0.0, Hash40::new("s_tail4"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail4"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail2"), 0.0, 0.0, 0.0, Hash40::new("s_tail3"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail3"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail1"), 0.0, 0.0, 0.0, Hash40::new("s_tail2"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail2"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail7"), 0, 0, 0, 0, 0, 0, 1.0, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail5"), 0, 0, 0, 0, 0, 0, 0.8, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail3"), 0, 0, 0, 0, 0, 0, 0.6, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail1"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        AFTER_IMAGE_OFF(agent, 4);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_deathscythe_trace"), false, false);
    }
}

pub fn install() {
    Agent::new("mewtwo")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attacks3", ssbexo_mewtwo_forward_tilt_acmd, Low)
    .effect_acmd("effect_attacks3", ssbexo_mewtwo_forward_tilt_effect, Low)
    .sound_acmd("sound_attacks3", ssbexo_mewtwo_forward_tilt_sound, Low)
    .expression_acmd("expression_attacks3", ssbexo_mewtwo_forward_tilt_expression, Low)
    .game_acmd("game_attacks3hi", ssbexo_mewtwo_forward_tilt_acmd, Low)
    .effect_acmd("effect_attacks3hi", ssbexo_mewtwo_forward_tilt_effect, Low)
    .sound_acmd("sound_attacks3hi", ssbexo_mewtwo_forward_tilt_sound, Low)
    .expression_acmd("expression_attacks3hi", ssbexo_mewtwo_forward_tilt_expression, Low)
    .game_acmd("game_attacks3lw", ssbexo_mewtwo_forward_tilt_acmd, Low)
    .effect_acmd("effect_attacks3lw", ssbexo_mewtwo_forward_tilt_effect, Low)
    .sound_acmd("sound_attacks3lw", ssbexo_mewtwo_forward_tilt_sound, Low)
    .expression_acmd("expression_attacks3lw", ssbexo_mewtwo_forward_tilt_expression, Low)
    .game_acmd("game_attackhi3", ssbexo_mewtwo_up_tilt_acmd, Low)
    .effect_acmd("effect_attackhi3", ssbexo_mewtwo_up_tilt_effect, Low)
    .game_acmd("game_attacklw3", ssbexo_mewtwo_down_tilt_acmd, Low)
    .effect_acmd("effect_attacklw3", ssbexo_mewtwo_down_tilt_effect, Low)
    .install()
    ;
}