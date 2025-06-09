use super::*;

//Jab 1 ACMD
unsafe extern "C" fn ssbexo_wario_jab_1_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 361, 20, 0, 40, 3.2, 0.0, 6.3, 7.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 20, 0, 40, 3.2, 0.0, 5.6, 9.2, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 361, 20, 0, 38, 3.8, 0.0, 4.0, 12.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_wario_dash_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 50, 97, 0, 60, 4.0, 0.0, 5.0, 5.8, Some(0.0), Some(9.7), Some(5.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 50, 97, 0, 60, 4.0, 0.0, 6.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
}

//Dash Attack Effect
unsafe extern "C" fn ssbexo_wario_dash_attack_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("wario_attack_dash"), Hash40::new("top"), 0, 4, 16, 0, 0, 0, 1, true);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 9, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.6);
    }
}

//Dash Attack Sound
unsafe extern "C" fn ssbexo_wario_dash_attack_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_attackdash"));
    }
}

//Dash Attack Expression
unsafe extern "C" fn ssbexo_wario_dash_attack_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 9);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Dash Attack Loop ACMD
unsafe extern "C" fn ssbexo_wario_dash_attack_loop_acmd(agent: &mut L2CAgentBase) {
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_HIT) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 50, 97, 0, 60, 4.0, 0.0, 5.0, 5.8, Some(0.0), Some(9.7), Some(5.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 50, 97, 0, 60, 4.0, 0.0, 6.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
    }
}

//Dash Attack Loop Effect
unsafe extern "C" fn ssbexo_wario_dash_attack_loop_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("wario_attack_dash"), Hash40::new("top"), 0, 4, 16, 0, 0, 0, 1, true);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 9, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.6);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 9, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
}

//Dash Attack Loop Sound
unsafe extern "C" fn ssbexo_wario_dash_attack_loop_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_attackdash"));
    }
}

//Dash Attack Loop Expression
unsafe extern "C" fn ssbexo_wario_dash_attack_loop_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 9);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Dash Attack Jump Squat ACMD
unsafe extern "C" fn ssbexo_wario_dash_attack_jump_squat_acmd(agent: &mut L2CAgentBase) {
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_HIT) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 50, 97, 0, 60, 4.0, 0.0, 5.0, 5.8, Some(0.0), Some(9.7), Some(5.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 50, 97, 0, 60, 4.0, 0.0, 6.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        ADD_SPEED_NO_LIMIT(agent, 0, 2.0);
        SA_SET(agent, *SITUATION_KIND_AIR);
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
}

//Dash Attack Jump Squat Effect
unsafe extern "C" fn ssbexo_wario_dash_attack_jump_squat_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("wario_attack_dash"), Hash40::new("top"), 0, 4, 16, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
}

//Dash Attack Jump Squat Sound
unsafe extern "C" fn ssbexo_wario_dash_attack_jump_squat_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_attackdash"));
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_wario_rnd_jump"));
        PLAY_SE(agent, Hash40::new("se_wario_jump01"));
    }
}

//Dash Attack Jump Squat Expression
unsafe extern "C" fn ssbexo_wario_dash_attack_jump_squat_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 9);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Dash Attack Air Loop ACMD
unsafe extern "C" fn ssbexo_wario_dash_attack_air_loop_acmd(agent: &mut L2CAgentBase) {
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_HIT) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 50, 97, 0, 60, 4.0, 0.0, 5.0, 5.8, Some(0.0), Some(9.7), Some(5.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 50, 97, 0, 60, 4.0, 0.0, 6.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
    }
}

//Dash Attack Air Loop Effect
unsafe extern "C" fn ssbexo_wario_dash_attack_air_loop_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("wario_attack_dash"), Hash40::new("top"), 0, 4, 16, 0, 0, 0, 1, true);
    }
}

//Dash Attack Air Loop Sound
unsafe extern "C" fn ssbexo_wario_dash_attack_air_loop_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_attackdash"));
    }
}

//Dash Attack Air Loop Expression
unsafe extern "C" fn ssbexo_wario_dash_attack_air_loop_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Dash Attack Landing ACMD
unsafe extern "C" fn ssbexo_wario_dash_attack_landing_acmd(agent: &mut L2CAgentBase) {
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_HIT) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 50, 97, 0, 60, 4.0, 0.0, 5.0, 5.8, Some(0.0), Some(9.7), Some(5.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 50, 97, 0, 60, 4.0, 0.0, 6.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
    }
}

//Dash Attack Landing Effect
unsafe extern "C" fn ssbexo_wario_dash_attack_landing_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("wario_attack_dash"), Hash40::new("top"), 0, 4, 16, 0, 0, 0, 1, true);
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Dash Attack Landing Sound
unsafe extern "C" fn ssbexo_wario_dash_attack_landing_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_attackdash"));
        STOP_SE(agent, Hash40::new("se_common_011"));
        PLAY_LANDING_SE(agent, Hash40::new("se_wario_landing02"));
    }
}

//Dash Attack Landing Expression
unsafe extern "C" fn ssbexo_wario_dash_attack_landing_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Dash Attack End ACMD
unsafe extern "C" fn ssbexo_wario_dash_attack_end_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Dash Attack End Effect
unsafe extern "C" fn ssbexo_wario_dash_attack_end_effect(_agent: &mut L2CAgentBase) {}

//Dash Attack End Sound
unsafe extern "C" fn ssbexo_wario_dash_attack_end_sound(_agent: &mut L2CAgentBase) {}

//Dash Attack End Expression
unsafe extern "C" fn ssbexo_wario_dash_attack_end_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attack11", ssbexo_wario_jab_1_acmd, Low)
    .game_acmd("game_attackdash", ssbexo_wario_dash_attack_acmd, Low)
    .effect_acmd("effect_attackdash", ssbexo_wario_dash_attack_effect, Low)
    .sound_acmd("sound_attackdash", ssbexo_wario_dash_attack_sound, Low)
    .expression_acmd("expression_attackdash", ssbexo_wario_dash_attack_expression, Low)
    .game_acmd("game_attackdashloop", ssbexo_wario_dash_attack_loop_acmd, Low)
    .effect_acmd("effect_attackdashloop", ssbexo_wario_dash_attack_loop_effect, Low)
    .sound_acmd("sound_attackdashloop", ssbexo_wario_dash_attack_loop_sound, Low)
    .expression_acmd("expression_attackdashloop", ssbexo_wario_dash_attack_loop_expression, Low)
    .game_acmd("game_attackdashjumpsquat", ssbexo_wario_dash_attack_jump_squat_acmd, Low)
    .effect_acmd("effect_attackdashjumpsquat", ssbexo_wario_dash_attack_jump_squat_effect, Low)
    .sound_acmd("sound_attackdashjumpsquat", ssbexo_wario_dash_attack_jump_squat_sound, Low)
    .expression_acmd("expression_attackdashjumpsquat", ssbexo_wario_dash_attack_jump_squat_expression, Low)
    .game_acmd("game_attackdashairloop", ssbexo_wario_dash_attack_air_loop_acmd, Low)
    .effect_acmd("effect_attackdashairloop", ssbexo_wario_dash_attack_air_loop_effect, Low)
    .sound_acmd("sound_attackdashairloop", ssbexo_wario_dash_attack_air_loop_sound, Low)
    .expression_acmd("expression_attackdashairloop", ssbexo_wario_dash_attack_air_loop_expression, Low)
    .game_acmd("game_attackdashlanding", ssbexo_wario_dash_attack_landing_acmd, Low)
    .effect_acmd("effect_attackdashlanding", ssbexo_wario_dash_attack_landing_effect, Low)
    .sound_acmd("sound_attackdashlanding", ssbexo_wario_dash_attack_landing_sound, Low)
    .expression_acmd("expression_attackdashlanding", ssbexo_wario_dash_attack_landing_expression, Low)
    .game_acmd("game_attackdashend", ssbexo_wario_dash_attack_end_acmd, Low)
    .effect_acmd("effect_attackdashend", ssbexo_wario_dash_attack_end_effect, Low)
    .sound_acmd("sound_attackdashend", ssbexo_wario_dash_attack_end_sound, Low)
    .expression_acmd("expression_attackdashend", ssbexo_wario_dash_attack_end_expression, Low)
    .game_acmd("game_attackdashairend", ssbexo_wario_dash_attack_end_acmd, Low)
    .effect_acmd("effect_attackdashairend", ssbexo_wario_dash_attack_end_effect, Low)
    .sound_acmd("sound_attackdashairend", ssbexo_wario_dash_attack_end_sound, Low)
    .expression_acmd("expression_attackdashairend", ssbexo_wario_dash_attack_end_expression, Low)
    .install()
    ;
}