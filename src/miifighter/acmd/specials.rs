use super::*;

//Rising Tiger Knee Start ACMD
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_start_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 367, 20, 20, 0, 7.0, 0.0, 10.0, 3.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
}

//Grounded Rising Tiger Knee Start Effect
unsafe extern "C" fn ssbexo_miifighter_grounded_rising_tiger_knee_start_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 10, 8, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_arc_c"), Hash40::new("sys_attack_arc_c"), Hash40::new("top"), 0, 11.5, 3, -62, 9, 90, 0.8, true, *EF_FLIP_YZ, 0.3);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}

//Aerial Rising Tiger Knee Start Effect
unsafe extern "C" fn ssbexo_miifighter_aerial_rising_tiger_knee_start_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 10, 8, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_arc_c"), Hash40::new("sys_attack_arc_c"), Hash40::new("top"), 0, 11.5, 3, -62, 9, 90, 0.8, true, *EF_FLIP_YZ, 0.3);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}

//Rising Tiger Knee Start Sound
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_start_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_miifighter_rnd_jump"));
    }
}

//Grounded Rising Tiger Knee Start Expression
unsafe extern "C" fn ssbexo_miifighter_grounded_rising_tiger_knee_start_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
}

//Aerial Rising Tiger Knee Start Expression
unsafe extern "C" fn ssbexo_miifighter_aerial_rising_tiger_knee_start_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
}

//Rising Tiger Knee Rise ACMD
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_rise_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::CORRECT(agent, *GROUND_CORRECT_KIND_AIR);
        macros::SA_SET(agent, *SITUATION_KIND_AIR);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 367, 80, 70, 0, 7.0, 0.0, 10.0, 3.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 290, 20, 20, 0, 7.0, 0.0, 10.0, 3.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("legr"), 1.0, 367, 100, 10, 0, 4.0, 3.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 1.0, 367, 100, 10, 0, 5.0, 5.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
}

//Rising Tiger Knee Rise Effect
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_rise_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_machstamp"), Hash40::new("top"), 0, 7, 0, -30, 0, 0, 0.85, true);
    }
}

//Rising Tiger Knee Rise Sound
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_rise_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_miifighter_swing_ll"));
        macros::PLAY_SE(agent, Hash40::new("vc_mii_attack08"));
    }
}

//Rising Tiger Knee Rise Expression
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_rise_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

//Rising Tiger Knee Dive ACMD
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_dive_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("legr"), 1.0, 367, 100, 120, 0, 4.0, 3.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 1.0, 367, 100, 120, 0, 5.0, 5.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
}

//Rising Tiger Knee Dive Effect
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_dive_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_machstamp"), false, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_machstamp"), Hash40::new("top"), 0, 7, 0, -30, 0, 0, 0.85, true);
    }
}

//Rising Tiger Knee Dive Expression
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_dive_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

//Rising Tiger Knee Land ACMD
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_land_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 65, 100, 0, 50, 4.0, 0.0, 5.5, 10.0, Some(0.0), Some(5.5), Some(-10.0), 2.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Rising Tiger Knee Land Effect
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_land_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0.0, 0.0, 0, 0.0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);
        macros::EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 0.0, 0.0, 0, 0.0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);
    }
}

//Rising Tiger Knee Land Sound
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_land_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_heavy_hit_m"));
    }
}

//Rising Tiger Knee Land Expression
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_land_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

//Grounded Onslaught End ACMD
unsafe extern "C" fn ssbexo_miifighter_grounded_onslaught_end_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 80, 0, 7.5, 0.0, 8.5, 8.5, Some(0.0), Some(8.5), Some(0.0), 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 80, 0, 7.5, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(0.0), 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 80, 0, 7.5, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(0.0), 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 80, 0, 7.5, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(0.0), 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_DISABLE_OPPONENT_PASSIVE);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 80, 0, 7.5, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(0.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_DISABLE_OPPONENT_PASSIVE);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("toer"), 7.0, 80, 144, 0, 58, 7.0, 0.0, 0.0, 0.0, Some(-6.0), Some(-2.0), Some(1.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Aerial Onslaught End ACMD
unsafe extern "C" fn ssbexo_miifighter_aerial_onslaught_end_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::SET_SPEED_EX(agent, 1, 1, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_LANDING_MUL_SPEED_X);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_LANDING_MUL_SPEED_X);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 80, 0, 7.0, 0.0, 8.0, 6.3, Some(0.0), Some(8.0), Some(0.0), 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::SET_SPEED_EX(agent, 0.3, 0.5, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_LANDING_MUL_SPEED_X);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 80, 0, 7.5, 0.0, 8.0, 6.0, Some(0.0), Some(8.0), Some(0.0), 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::SET_SPEED_EX(agent, 0.3, 0.5, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_LANDING_MUL_SPEED_X);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 80, 0, 7.5, 0.0, 8.0, 9.0, Some(0.0), Some(8.0), Some(0.0), 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 0.3, y: 0.5, z: 0.0});
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 80, 0, 7.5, 0.0, 8.0, 9.0, Some(0.0), Some(8.0), Some(0.0), 0.7, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: -0.3, y: 0.7, z: 0.0});
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 80, 0, 7.5, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(0.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 0.85, y: 3.0, z: 0.0});
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 70, 135, 0, 54, 7.0, 0.0, 16.0, 9.5, Some(0.0), Some(10.0), Some(8.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 55.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_CONTROL_X);
    }
}

//Suplex Catch ACMD
unsafe extern "C" fn ssbexo_miifighter_suplex_catch_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        HitModule::set_whole(agent.module_accessor, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 0, 10, 0, 100, 1.0, 1.0, *ATTACK_LR_CHECK_POS, 0.0, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 50, 100, 0, 60, 6.0, 0.0, 10.0, 4.0, Some(0.0), Some(4.0), Some(4.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 50, 100, 0, 60, 6.0, -10.0, 7.0, 4.0, Some(10.0), Some(7.0), Some(4.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 50, 100, 0, 60, 6.0, 0.0, 10.0, 4.0, Some(0.0), Some(4.0), Some(4.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        sv_animcmd::SET_AIR(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("head"), 3.0, 361, 70, 0, 65, 6.7, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Suplex Fall ACMD
unsafe extern "C" fn ssbexo_miifighter_suplex_fall_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        HitModule::set_whole(agent.module_accessor, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 0, 10, 0, 100, 1.0, 1.0, *ATTACK_LR_CHECK_POS, 0.0, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("top"), 4.5, 361, 100, 0, 45, 6.7, 0.0, 4.5, 0.0, Some(0.0), Some(8.0), Some(0.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK_IGNORE_THROW(agent, 1, 0, Hash40::new("top"), 4.5, 361, 10, 0, 10, 6.2, 0.0, 5.0, 0.0, Some(0.0), Some(8.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
}

//Suplex Throw ACMD
unsafe extern "C" fn ssbexo_miifighter_suplex_throw_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        HitModule::set_whole(agent.module_accessor, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 30, 65, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 65, 30, 0, 60, 4.0, 0.0, 4.0, 4.2, Some(0.0), Some(4.0), Some(-3.2), 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(agent, -6, 7);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.4);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        JostleModule::set_status(agent.module_accessor, false);
        HitModule::set_whole(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::REVERSE_LR(agent);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 41.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

//Armor Crushing Thunder Kick Start ACMD
unsafe extern "C" fn ssbexo_miifighter_armor_crushing_thunder_kick_start_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_ARMOR_CRUSHING_THUNDER_KICK_ACTIVE_ARMOR);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_ARMOR_CRUSHING_THUNDER_KICK_DASH_CANCEL);
    }
}

//Armor Crushing Thunder Kick Start Effect
unsafe extern "C" fn ssbexo_miifighter_armor_crushing_thunder_kick_start_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("miifighter_smash_s_end"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 0.0);
    }
}

//Armor Crushing Thunder Kick Start Sound
unsafe extern "C" fn ssbexo_miifighter_armor_crushing_thunder_kick_start_sound(_agent: &mut L2CAgentBase) {}

//Grounded Armor Crushing Thunder Kick Start Expression
unsafe extern "C" fn ssbexo_miifighter_grounded_armor_crushing_thunder_kick_start_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Aerial Armor Crushing Thunder Kick Start Expression
unsafe extern "C" fn ssbexo_miifighter_aerial_armor_crushing_thunder_kick_start_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Armor Crushing Thunder Kick Charge ACMD
unsafe extern "C" fn ssbexo_miifighter_armor_crushing_thunder_kick_charge_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_ARMOR_CRUSHING_THUNDER_KICK_DASH_CANCEL);
    }
}

//Armor Crushing Thunder Kick Charge Effect
unsafe extern "C" fn ssbexo_miifighter_armor_crushing_thunder_kick_charge_effect(agent: &mut L2CAgentBase) {
    for _ in 0..10 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("miifighter_sidekick_hold"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 0.8, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
            macros::EFFECT_FOLLOW(agent, Hash40::new("miifighter_smash_s_end"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
            macros::EFFECT_FOLLOW(agent, Hash40::new("miifighter_smash_s_end"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
            macros::EFFECT_FOLLOW(agent, Hash40::new("miifighter_smash_s_end"), Hash40::new("legl"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 1, 1, 0.392, 0.392);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 1, 0.392, 0, 0.353);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_explosion_sign"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 55.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_explosion_sign"), false, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_explosion_flash"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
    }
}

//Armor Crushing Thunder Kick Charge Sound
unsafe extern "C" fn ssbexo_miifighter_armor_crushing_thunder_kick_charge_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_miifighter_special_c3_n01"));
    }
}

//Grounded Armor Crushing Thunder Kick Charge Expression
unsafe extern "C" fn ssbexo_miifighter_grounded_armor_crushing_thunder_kick_charge_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Aerial Armor Crushing Thunder Kick Charge Expression
unsafe extern "C" fn ssbexo_miifighter_aerial_armor_crushing_thunder_kick_charge_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Armor Crushing Thunder Kick Attack ACMD
unsafe extern "C" fn ssbexo_miifighter_armor_crushing_thunder_kick_attack_acmd(agent: &mut L2CAgentBase) {
    let damage = WorkModule::get_float(agent.module_accessor, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLOAT_ARMOR_CRUSHING_THUNDER_KICK_CURRENT_DAMAGE);
    let attribute = WorkModule::get_int(agent.module_accessor, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_ARMOR_CRUSHING_THUNDER_KICK_ATTRIBUTE);
    let collision_attr = match attribute {
        0 => Hash40::new("collision_attr_elec"),
        _ => Hash40::new("collision_attr_saving")
    };
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_ARMOR_CRUSHING_THUNDER_KICK_ACTIVE_ARMOR);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_ARMOR_CRUSHING_THUNDER_KICK_DASH_CANCEL);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("legl"), damage, 361, 40, 0, 40, 4.0, 6.8, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("legl"), damage, 361, 40, 0, 40, 3.0, -4.7, 0.0, 0.0, Some(2.2), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, collision_attr, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        match attribute {
            1 => {
                AttackModule::set_attack_level(agent.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_2 as u8);
                AttackModule::set_attack_level(agent.module_accessor, 1, *FIGHTER_RYU_SAVING_LV_2 as u8);
            },
            2 => {
                AttackModule::set_attack_level(agent.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
                AttackModule::set_attack_level(agent.module_accessor, 1, *FIGHTER_RYU_SAVING_LV_3 as u8);
            },
            _ => {}
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Grounded Armor Crushing Thunder Kick Attack Effect
unsafe extern "C" fn ssbexo_miifighter_grounded_armor_crushing_thunder_kick_attack_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_explosion_flash"), false, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("miifighter_smash_s_end"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 1.5, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
        macros::EFFECT_FOLLOW(agent, Hash40::new("miifighter_smash_s_end"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1.5, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
        macros::EFFECT_FOLLOW(agent, Hash40::new("miifighter_smash_s_end"), Hash40::new("legl"), 0, 0, 0, 0, 0, 0, 1.5, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_dead"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_dead"), Hash40::new("toel"), 0, 0, 0, 0, 0, 60, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_dead_dark"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 0.15, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 3.5, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Armor Crushing Thunder Kick Attack Effect
unsafe extern "C" fn ssbexo_miifighter_aerial_armor_crushing_thunder_kick_attack_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_explosion_flash"), false, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("miifighter_smash_s_end"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 1.5, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
        macros::EFFECT_FOLLOW(agent, Hash40::new("miifighter_smash_s_end"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1.5, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
        macros::EFFECT_FOLLOW(agent, Hash40::new("miifighter_smash_s_end"), Hash40::new("legl"), 0, 0, 0, 0, 0, 0, 1.5, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_dead"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_dead"), Hash40::new("toel"), 0, 0, 0, 0, 0, 60, 0.25, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_dead_dark"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 0.15, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.0);
    }
}

//Armor Crushing Thunder Kick Attack Sound
unsafe extern "C" fn ssbexo_miifighter_armor_crushing_thunder_kick_attack_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_miifighter_special_c3_n02"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_miifighter_rnd_special_c3_n01"));
    }
}

//Armor Crushing Thunder Kick Attack Expression
unsafe extern "C" fn ssbexo_miifighter_armor_crushing_thunder_kick_attack_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

//Counter Throw ACMD
unsafe extern "C" fn ssbexo_miifighter_counter_throw_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_SHIELD);
        macros::SEARCH(agent, 0, 0, Hash40::new("top"), 8.0, 0.0, 7.0, 0.0, None, None, None, *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
        macros::SEARCH(agent, 1, 0, Hash40::new("top"), 8.0, 0.0, 7.0, 0.0, None, None, None, *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ITEM, *COLLISION_PART_MASK_ALL, false);
        search!(agent, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_SHIELD);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
}

//Counter Throw Throw Toss ACMD
unsafe extern "C" fn ssbexo_miifighter_counter_throw_throw_toss_acmd(agent: &mut L2CAgentBase) {
    let counter_throw_object_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_COUNTER_THROW_OBJECT_ID);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 2.0);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 1.0);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("haver"), 11.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.5, 1.5, 50, false, 1.5, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(agent.lua_state_agent, 18.0);
    if !counter_throw_object_id != *BATTLE_OBJECT_ID_INVALID {
        if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_WEAPON {
            let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
            if macros::is_excute(agent) {
                WorkModule::off_flag(agent.module_accessor, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_COUNTER_THROW_IS_LINK);
                macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
                GroundModule::set_ignore_boss(counter_throw_boma, false);
                GroundModule::set_passable_check(counter_throw_boma, true);
                GroundModule::set_collidable(counter_throw_boma, true);
                JostleModule::set_status(counter_throw_boma, true);
                shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
            }
        }
        if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_ITEM {
            let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
            if macros::is_excute(agent) {
                WorkModule::off_flag(agent.module_accessor, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_COUNTER_THROW_IS_LINK);
                macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
                GroundModule::set_ignore_boss(counter_throw_boma, false);
                GroundModule::set_passable_check(counter_throw_boma, true);
                GroundModule::set_collidable(counter_throw_boma, true);
                JostleModule::set_status(counter_throw_boma, true);
                shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
            }   
        }
    }
    else {
        if macros::is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_FLAG_COUNTER_THROW_IS_LINK);
            macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
            shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        }
    }
}

//Grounded Counter Throw Throw Toss Effect
unsafe extern "C" fn ssbexo_miifighter_grounded_counter_throw_throw_toss_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::COL_PRI(agent, 101);
        macros::FLASH(agent, 1, 1, 1, 0);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_catch"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("miifighter_counter_arc"), Hash40::new("miifighter_counter_arc"), Hash40::new("top"), -1, 8, 1, 0, 112, 90, 0.8, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 1, 1, 1, 0);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("miifighter_counter_arc"), Hash40::new("top"), 0, 11, 4, -13, -56, 46, 1.1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_reflection"), true, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_reflection"), true, true);
    }
}

//Aerial Counter Throw Throw Toss Effect
unsafe extern "C" fn ssbexo_miifighter_aerial_counter_throw_throw_toss_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::COL_PRI(agent, 101);
        macros::FLASH(agent, 1, 1, 1, 0);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_catch"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("miifighter_counter_arc"), Hash40::new("miifighter_counter_arc"), Hash40::new("top"), -1, 8, 1, 0, 112, 90, 0.8, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 1, 1, 1, 0);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("miifighter_counter_arc"), Hash40::new("top"), 0, 11, 4, -13, -56, 46, 1.1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_reflection"), true, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_reflection"), true, true);
    }
}

//Counter Throw Throw Toss Sound
unsafe extern "C" fn ssbexo_miifighter_counter_throw_throw_toss_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_cliff_catch"));
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_miifighter_rnd_special_c3_l01"));
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_miifighter_special_c3_l03"));
    }
}

//Grounded Counter Throw Throw Toss Expression
unsafe extern "C" fn ssbexo_miifighter_grounded_counter_throw_throw_toss_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_attacks"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x23c33f3bdc));
    }
}

//Aerial Counter Throw Throw Toss Expression
unsafe extern "C" fn ssbexo_miifighter_aerial_counter_throw_throw_toss_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_attacks"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x23c33f3bdc));
    }
}

pub fn install() {
    Agent::new("miifighter")
    .game_acmd("game_specialn3", ssbexo_miifighter_rising_tiger_knee_start_acmd, Priority::Low)
    .game_acmd("game_specialairn3", ssbexo_miifighter_rising_tiger_knee_start_acmd, Priority::Low)
    .effect_acmd("effect_specialn3", ssbexo_miifighter_grounded_rising_tiger_knee_start_effect, Priority::Low)
    .effect_acmd("effect_specialairn3", ssbexo_miifighter_aerial_rising_tiger_knee_start_effect, Priority::Low)
    .sound_acmd("sound_specialn3", ssbexo_miifighter_rising_tiger_knee_start_sound, Priority::Low)
    .sound_acmd("sound_specialairn3", ssbexo_miifighter_rising_tiger_knee_start_sound, Priority::Low)
    .expression_acmd("expression_specialn3", ssbexo_miifighter_grounded_rising_tiger_knee_start_expression, Priority::Low)
    .expression_acmd("expression_specialairn3", ssbexo_miifighter_aerial_rising_tiger_knee_start_expression, Priority::Low)
    .game_acmd("game_specialn3rise", ssbexo_miifighter_rising_tiger_knee_rise_acmd, Priority::Low)
    .effect_acmd("effect_specialn3rise", ssbexo_miifighter_rising_tiger_knee_rise_effect, Priority::Low)
    .sound_acmd("sound_specialn3rise", ssbexo_miifighter_rising_tiger_knee_rise_sound, Priority::Low)
    .expression_acmd("expression_specialn3rise", ssbexo_miifighter_rising_tiger_knee_rise_expression, Priority::Low)
    .game_acmd("game_specialn3dive", ssbexo_miifighter_rising_tiger_knee_dive_acmd, Priority::Low)
    .effect_acmd("effect_specialn3dive", ssbexo_miifighter_rising_tiger_knee_dive_effect, Priority::Low)
    .expression_acmd("expression_specialn3dive", ssbexo_miifighter_rising_tiger_knee_dive_expression, Priority::Low)
    .game_acmd("game_specialn3land", ssbexo_miifighter_rising_tiger_knee_land_acmd, Priority::Low)
    .effect_acmd("effect_specialn3land", ssbexo_miifighter_rising_tiger_knee_land_effect, Priority::Low)
    .sound_acmd("sound_specialn3land", ssbexo_miifighter_rising_tiger_knee_land_sound, Priority::Low)
    .expression_acmd("expression_specialn3land", ssbexo_miifighter_rising_tiger_knee_land_expression, Priority::Low)
    .game_acmd("game_specials1end", ssbexo_miifighter_grounded_onslaught_end_acmd, Priority::Low)
    .game_acmd("game_specialairs1end", ssbexo_miifighter_aerial_onslaught_end_acmd, Priority::Low)
    .game_acmd("game_specials3throw", ssbexo_miifighter_suplex_catch_acmd, Priority::Low)
    .game_acmd("game_specialairs3catch", ssbexo_miifighter_suplex_catch_acmd, Priority::Low)
    .game_acmd("game_specialairs3fall", ssbexo_miifighter_suplex_fall_acmd, Priority::Low)
    .game_acmd("game_specialairs3landing", ssbexo_miifighter_suplex_throw_acmd, Priority::Low)
    .game_acmd("game_speciallw1", ssbexo_miifighter_armor_crushing_thunder_kick_start_acmd, Priority::Low)
    .game_acmd("game_specialairlw1", ssbexo_miifighter_armor_crushing_thunder_kick_start_acmd, Priority::Low)
    .effect_acmd("effect_speciallw1", ssbexo_miifighter_armor_crushing_thunder_kick_start_effect, Priority::Low)
    .effect_acmd("effect_specialairlw1", ssbexo_miifighter_armor_crushing_thunder_kick_start_effect, Priority::Low)
    .sound_acmd("sound_speciallw1", ssbexo_miifighter_armor_crushing_thunder_kick_start_sound, Priority::Low)
    .sound_acmd("sound_specialairlw1", ssbexo_miifighter_armor_crushing_thunder_kick_start_sound, Priority::Low)
    .expression_acmd("expression_speciallw1", ssbexo_miifighter_grounded_armor_crushing_thunder_kick_start_expression, Priority::Low)
    .expression_acmd("expression_specialairlw1", ssbexo_miifighter_aerial_armor_crushing_thunder_kick_start_expression, Priority::Low)
    .game_acmd("game_speciallw1charge", ssbexo_miifighter_armor_crushing_thunder_kick_charge_acmd, Priority::Low)
    .game_acmd("game_specialairlw1charge", ssbexo_miifighter_armor_crushing_thunder_kick_charge_acmd, Priority::Low)
    .effect_acmd("effect_speciallw1charge", ssbexo_miifighter_armor_crushing_thunder_kick_charge_effect, Priority::Low)
    .effect_acmd("effect_specialairlw1charge", ssbexo_miifighter_armor_crushing_thunder_kick_charge_effect, Priority::Low)
    .sound_acmd("sound_speciallw1charge", ssbexo_miifighter_armor_crushing_thunder_kick_charge_sound, Priority::Low)
    .sound_acmd("sound_specialairlw1charge", ssbexo_miifighter_armor_crushing_thunder_kick_charge_sound, Priority::Low)
    .expression_acmd("expression_speciallw1charge", ssbexo_miifighter_grounded_armor_crushing_thunder_kick_charge_expression, Priority::Low)
    .expression_acmd("expression_specialairlw1charge", ssbexo_miifighter_aerial_armor_crushing_thunder_kick_charge_expression, Priority::Low)
    .game_acmd("game_speciallw1attack", ssbexo_miifighter_armor_crushing_thunder_kick_attack_acmd, Priority::Low)
    .game_acmd("game_specialairlw1attack", ssbexo_miifighter_armor_crushing_thunder_kick_attack_acmd, Priority::Low)
    .effect_acmd("effect_speciallw1attack", ssbexo_miifighter_grounded_armor_crushing_thunder_kick_attack_effect, Priority::Low)
    .effect_acmd("effect_specialairlw1attack", ssbexo_miifighter_aerial_armor_crushing_thunder_kick_attack_effect, Priority::Low)
    .sound_acmd("sound_speciallw1attack", ssbexo_miifighter_armor_crushing_thunder_kick_attack_sound, Priority::Low)
    .sound_acmd("sound_specialairlw1attack", ssbexo_miifighter_armor_crushing_thunder_kick_attack_sound, Priority::Low)
    .expression_acmd("expression_speciallw1attack", ssbexo_miifighter_armor_crushing_thunder_kick_attack_expression, Priority::Low)
    .expression_acmd("expression_specialairlw1attack", ssbexo_miifighter_armor_crushing_thunder_kick_attack_expression, Priority::Low)
    .game_acmd("game_speciallw3", ssbexo_miifighter_counter_throw_acmd, Priority::Low)
    .game_acmd("game_specialairlw3", ssbexo_miifighter_counter_throw_acmd, Priority::Low)
    .game_acmd("game_speciallw3throwtoss", ssbexo_miifighter_counter_throw_throw_toss_acmd, Priority::Low)
    .game_acmd("game_specialairlw3throwtoss", ssbexo_miifighter_counter_throw_throw_toss_acmd, Priority::Low)
    .effect_acmd("effect_speciallw3throwtoss", ssbexo_miifighter_grounded_counter_throw_throw_toss_effect, Priority::Low)
    .effect_acmd("effect_specialairlw3throwtoss", ssbexo_miifighter_aerial_counter_throw_throw_toss_effect, Priority::Low)
    .sound_acmd("sound_speciallw3throwtoss", ssbexo_miifighter_counter_throw_throw_toss_sound, Priority::Low)
    .sound_acmd("sound_specialairlw3throwtoss", ssbexo_miifighter_counter_throw_throw_toss_sound, Priority::Low)
    .expression_acmd("expression_speciallw3throwtoss", ssbexo_miifighter_grounded_counter_throw_throw_toss_expression, Priority::Low)
    .expression_acmd("expression_specialairlw3throwtoss", ssbexo_miifighter_aerial_counter_throw_throw_toss_expression, Priority::Low)
    .install()
    ;
}