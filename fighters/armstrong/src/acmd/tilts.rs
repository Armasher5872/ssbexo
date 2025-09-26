use super::*;

//Forward Tilt ACMD
unsafe extern "C" fn ssbexo_armstrong_forward_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 6.0);
        DamageModule::set_reaction_mul(agent.module_accessor, 0.85);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        }
        ATTACK(agent, 0, 0, Hash40::new("hip"), 13.0, 27, 82, 0, 31, 4.8, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 13.0, 27, 82, 0, 31, 5.0, -0.5, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("kneel"), 14.0, 27, 82, 0, 31, 5.5, 5.3, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        AttackModule::set_power_up(agent.module_accessor, 1.0);
        DamageModule::set_reaction_mul(agent.module_accessor, 1.0);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
        WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
        WorkModule::set_float(agent.module_accessor, 1.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
        WorkModule::set_float(agent.module_accessor, 1.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
    }
}

//Forward Tilt Effect
unsafe extern "C" fn ssbexo_armstrong_forward_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 11.8, -10, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 12, 18.5, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true, 0.8);
    }
}

//Forward Tilt Hi Effect
unsafe extern "C" fn ssbexo_armstrong_forward_tilt_hi_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 10.8, -10, -32, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true, 0.8);
    }
}

//Forward Tilt Lw Effect
unsafe extern "C" fn ssbexo_armstrong_forward_tilt_lw_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 12.8, -10, 28, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("footl"), 0, 0, 10, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true, 0.8);
    }
}

//Forward Tilt Sound
unsafe extern "C" fn ssbexo_armstrong_forward_tilt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_ganon_swing_l"));
    }
}

//Up Tilt ACMD
unsafe extern "C" fn ssbexo_armstrong_up_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
        ATTACK(agent, 0, 0, Hash40::new("footl"), 6.0, 35, 80, 80, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 6.0, 35, 80, 80, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("legl"), 6.0, 35, 80, 80, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 19.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 19.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 19.0, false);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 7.0);
        DamageModule::set_reaction_mul(agent.module_accessor, 0.85);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        }
        ATTACK(agent, 0, 0, Hash40::new("footl"), 18.0, 361, 75, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 18.0, 361, 75, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("legl"), 18.0, 361, 75, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        DamageModule::set_reaction_mul(agent.module_accessor, 1.0);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
        WorkModule::set_float(agent.module_accessor, 1.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
    }
    frame(agent.lua_state_agent, 29.0);
    if is_excute(agent) {
        if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 80, 0, 60, 6.0, 0.0, 7.0, 16.0, Some(0.0), Some(22.0), Some(16.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 361, 80, 0, 60, 6.0, 0.0, 4.0, 16.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 32.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        AttackModule::set_power_up(agent.module_accessor, 1.0);
        WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
        WorkModule::set_float(agent.module_accessor, 1.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
    }
}

//Up Tilt Effect
unsafe extern "C" fn ssbexo_armstrong_up_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 10, 0, 0, -40, -90, 1.2, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("edge_gokumon_impact"), Hash40::new("top"), 0, 0, 12, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        LAST_EFFECT_SET_RATE(agent, 0.5);
    }
    frame(agent.lua_state_agent, 29.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        LANDING_EFFECT(agent, Hash40::new("ganon_majinken"), Hash40::new("top"), 12, 0, 0, 90, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

//Up Tilt Sound
unsafe extern "C" fn ssbexo_armstrong_up_tilt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_ganon_swing_l"));
    }
}

//Up Tilt Expression
unsafe extern "C" fn ssbexo_armstrong_up_tilt_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 38, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_explosionm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 37.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 10);
    }
}

//Down Tilt ACMD
unsafe extern "C" fn ssbexo_armstrong_down_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 4.5);
        DamageModule::set_reaction_mul(agent.module_accessor, 0.85);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        }
        ATTACK(agent, 0, 0, Hash40::new("legr"), 14.0, 80, 94, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 14.0, 90, 94, 0, 30, 4.8, 0.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("footr"), 14.0, 100, 94, 0, 30, 4.8, 0.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        AttackModule::set_power_up(agent.module_accessor, 1.0);
        DamageModule::set_reaction_mul(agent.module_accessor, 1.0);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
        WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
        WorkModule::set_float(agent.module_accessor, 1.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
        WorkModule::set_float(agent.module_accessor, 1.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
    }
}

//Down Tilt Effect
unsafe extern "C" fn ssbexo_armstrong_down_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), -1, 9.5, 0, 10, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 4.7, 15, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true, 0.8);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([8, 9, 10, 11, 12, 13, 14, 15].to_vec())
    .game_acmd("game_attacks3", ssbexo_armstrong_forward_tilt_acmd, Low)
    .game_acmd("game_attacks3hi", ssbexo_armstrong_forward_tilt_acmd, Low)
    .game_acmd("game_attacks3lw", ssbexo_armstrong_forward_tilt_acmd, Low)
    .effect_acmd("effect_attacks3", ssbexo_armstrong_forward_tilt_effect, Low)
    .effect_acmd("effect_attacks3hi", ssbexo_armstrong_forward_tilt_hi_effect, Low)
    .effect_acmd("effect_attacks3lw", ssbexo_armstrong_forward_tilt_lw_effect, Low)
    .sound_acmd("sound_attacks3", ssbexo_armstrong_forward_tilt_sound, Low)
    .sound_acmd("sound_attacks3hi", ssbexo_armstrong_forward_tilt_sound, Low)
    .sound_acmd("sound_attacks3lw", ssbexo_armstrong_forward_tilt_sound, Low)
    .game_acmd("game_attackhi3", ssbexo_armstrong_up_tilt_acmd, Low)
    .effect_acmd("effect_attackhi3", ssbexo_armstrong_up_tilt_effect, Low)
    .sound_acmd("sound_attackhi3", ssbexo_armstrong_up_tilt_sound, Low)
    .expression_acmd("expression_attackhi3", ssbexo_armstrong_up_tilt_expression, Low)
    .game_acmd("game_attacklw3", ssbexo_armstrong_down_tilt_acmd, Low)
    .effect_acmd("effect_attacklw3", ssbexo_armstrong_down_tilt_effect, Low)
    .install()
    ;
}