use super::*;

//Forward Tilt ACMD
unsafe extern "C" fn ssbexo_mewtwo_forward_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("s_tail2"), 12.0, 361, 75, 0, 70, 5.0, 1.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 1, 0, Hash40::new("s_tail4"), 11.0, 361, 75, 0, 70, 4.6, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 2, 0, Hash40::new("s_tail6"), 10.0, 361, 75, 0, 70, 4.2, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Tilt Effect
unsafe extern "C" fn ssbexo_mewtwo_forward_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_2"), 4, Hash40::new("s_tail6"), 0.0, 0.0, 0.0, Hash40::new("s_tail7"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail7"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail5"), 0.0, 0.0, 0.0, Hash40::new("s_tail6"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail6"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail4"), 0.0, 0.0, 0.0, Hash40::new("s_tail5"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail5"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail3"), 0.0, 0.0, 0.0, Hash40::new("s_tail4"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail4"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail2"), 0.0, 0.0, 0.0, Hash40::new("s_tail3"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail3"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail1"), 0.0, 0.0, 0.0, Hash40::new("s_tail2"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail2"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail7"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail5"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail3"), 0, 0, 0, 0, 0, 0, 0.6, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail1"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_deathscythe_trace"), false, false);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
}

//Up Tilt ACMD
unsafe extern "C" fn ssbexo_mewtwo_up_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 95, 55, 0, 70, 6.0, 0.0, 2.0, -12.0, Some(0.0), Some(2.0), Some(12.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 95, 55, 0, 70, 6.0, 0.0, 11.7, -6.0, Some(0.0), Some(11.7), Some(7.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 95, 55, 0, 70, 6.0, 0.0, 14.7, -3.0, Some(0.0), Some(14.7), Some(3.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Up Tilt Effect
unsafe extern "C" fn ssbexo_mewtwo_up_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_appeal_aura"), Hash40::new("top"), 0, -1.5, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_dead"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 0.8, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 1.0);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 1.8, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 20, 0, 15, 0, 0, 0, false);
    }
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_shield_smoke"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_hit_death"), false, false);
    }
}

//Down Tilt ACMD
unsafe extern "C" fn ssbexo_mewtwo_down_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 80, 82, 0, 60, 4.3, 0.0, 3.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.5, 70, 84, 0, 65, 3.8, 0.0, 1.5, 13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 2, 0, Hash40::new("s_tail7"), 4.0, 60, 70, 0, 60, 3.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        AttackModule::set_attack_height_all(agent.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("s_tail3"), 5.0, 80, 82, 0, 60, 4.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 1, 0, Hash40::new("s_tail5"), 4.5, 70, 84, 0, 65, 3.8, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 2, 0, Hash40::new("s_tail7"), 4.0, 60, 70, 0, 60, 3.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        AttackModule::set_attack_height_all(agent.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Tilt Effect
unsafe extern "C" fn ssbexo_mewtwo_down_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_2"), 4, Hash40::new("s_tail6"), 0.0, 0.0, 0.0, Hash40::new("s_tail7"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail7"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail5"), 0.0, 0.0, 0.0, Hash40::new("s_tail6"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail6"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail4"), 0.0, 0.0, 0.0, Hash40::new("s_tail5"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail5"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail3"), 0.0, 0.0, 0.0, Hash40::new("s_tail4"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail4"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail2"), 0.0, 0.0, 0.0, Hash40::new("s_tail3"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail3"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail1"), 0.0, 0.0, 0.0, Hash40::new("s_tail2"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail2"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail7"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail5"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail3"), 0, 0, 0, 0, 0, 0, 0.6, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail1"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_deathscythe_trace"), false, false);
        macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
}

pub fn install() {
    Agent::new("mewtwo")
    .game_acmd("game_attacks3", ssbexo_mewtwo_forward_tilt_acmd)
    .game_acmd("game_attacks3hi", ssbexo_mewtwo_forward_tilt_acmd)
    .game_acmd("game_attacks3lw", ssbexo_mewtwo_forward_tilt_acmd)
    .effect_acmd("effect_attacks3", ssbexo_mewtwo_forward_tilt_effect)
    .effect_acmd("effect_attacks3hi", ssbexo_mewtwo_forward_tilt_effect)
    .effect_acmd("effect_attacks3lw", ssbexo_mewtwo_forward_tilt_effect)
    .game_acmd("game_attackhi3", ssbexo_mewtwo_up_tilt_acmd)
    .effect_acmd("effect_attackhi3", ssbexo_mewtwo_up_tilt_effect)
    .game_acmd("game_attacklw3", ssbexo_mewtwo_down_tilt_acmd)
    .effect_acmd("effect_attacklw3", ssbexo_mewtwo_down_tilt_effect)
    .install()
    ;
}