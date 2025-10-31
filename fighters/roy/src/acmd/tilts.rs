use super::*;

//Forward Tilt ACMD
unsafe extern "C" fn ssbexo_roy_forward_tilt_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        JostleModule::set_team(agent.module_accessor, 1);
    }
    MotionModule::set_rate(agent.module_accessor, 0.625);
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
        ATTACK(agent, 0, 0, Hash40::new("armr"), 12.0, 361, 60, 0, 40, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword1"), 12.0, 361, 60, 0, 40, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword2"), 8.0, 361, 60, 0, 40, 4.0, 0.0, 0.0, 6.0, Some(0.0), Some(0.0), Some(10.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        JostleModule::set_team(agent.module_accessor, 0);
    }
}

//Forward Tilt Effect
unsafe extern "C" fn ssbexo_roy_forward_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("haver"), 0, -2.5, 0, -90, 0, 0, 0.55, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.64, 0.0);
        LAST_EFFECT_SET_RATE(agent, 1.7);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
        EFFECT_OFF_KIND(agent, Hash40::new("roy_fire"), false, false);
    }
}

//Forward Tilt Sound
unsafe extern "C" fn ssbexo_roy_forward_tilt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_roy_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_roy_attackl_s01"));
    }
}

//Forward Tilt Expression
unsafe extern "C" fn ssbexo_roy_forward_tilt_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, 0);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, 0);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_nohitm"), 0);
    }
}

//Up Tilt ACMD
unsafe extern "C" fn ssbexo_roy_up_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 98, 103, 0, 35, 4.2, 0.0, 16.0, 0.0, Some(0.0), Some(16.0), Some(0.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword1"), 12.0, 98, 103, 0, 35, 2.8, 0.0, 0.0, 1.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 98, 103, 0, 35, 1.5, 0.0, 18.0, 6.0, Some(0.0), Some(10.0), Some(6.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 7.0, 65, 100, 0, 30, 2.0, 0.0, 16.0, 10.0, Some(0.0), Some(10.0), Some(10.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("sword1"), 7.0, 65, 100, 0, 30, 3.5, 0.0, 0.0, 7.2, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        AttackModule::clear(agent.module_accessor, 3, false);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Up Tilt Effect
unsafe extern "C" fn ssbexo_roy_up_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 7, Hash40::new("sword1"), 0, 0, -0.8, Hash40::new("sword1"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 10);
    }
}

pub fn install() {
    Agent::new("roy")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attacks3", ssbexo_roy_forward_tilt_acmd, Low)
    .game_acmd("game_attacks3hi", ssbexo_roy_forward_tilt_acmd, Low)
    .game_acmd("game_attacks3lw", ssbexo_roy_forward_tilt_acmd, Low)
    .effect_acmd("effect_attacks3", ssbexo_roy_forward_tilt_effect, Low)
    .effect_acmd("effect_attacks3hi", ssbexo_roy_forward_tilt_effect, Low)
    .effect_acmd("effect_attacks3lw", ssbexo_roy_forward_tilt_effect, Low)
    .sound_acmd("sound_attacks3", ssbexo_roy_forward_tilt_sound, Low)
    .sound_acmd("sound_attacks3hi", ssbexo_roy_forward_tilt_sound, Low)
    .sound_acmd("sound_attacks3lw", ssbexo_roy_forward_tilt_sound, Low)
    .expression_acmd("expression_attacks3", ssbexo_roy_forward_tilt_expression, Low)
    .expression_acmd("expression_attacks3hi", ssbexo_roy_forward_tilt_expression, Low)
    .expression_acmd("expression_attacks3lw", ssbexo_roy_forward_tilt_expression, Low)
    .game_acmd("game_attackhi3", ssbexo_roy_up_tilt_acmd, Low)
    .effect_acmd("effect_attackhi3", ssbexo_roy_up_tilt_effect, Low)
    .install()
    ;
}