use super::*;

//Up Special Jump ACMD
unsafe extern "C" fn ssbexo_wario_up_special_jump_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 0.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 7.0, 4.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(agent, 1, Hash40::new("top"), 4.0, 0.0, 7.0, -3.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(agent, 2, Hash40::new("top"), 4.0, 0.0, 13.0, 4.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(agent, 3, Hash40::new("top"), 4.0, 0.0, 13.0, -3.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    wait(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_HI_FLAG_DISABLE_MOTION_ANGLE);
    }
}

//Up Special Grab Effect
unsafe extern "C" fn ssbexo_wario_up_special_grab_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("wario_attack_air_n"), Hash40::new("wario_attack_air_n"), Hash40::new("rot"), 0, 4, 0, 0, -90, 0, 1, true, *EF_FLIP_YZ, 1);
        macros::LAST_EFFECT_SET_RATE(agent, 1.25);
        macros::EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("wario_attack_air_n"), Hash40::new("wario_attack_air_n"), Hash40::new("rot"), 0, 5, 0, 0, -90, 0, 1.0, true, *EF_FLIP_YZ, 0.6);
        macros::LAST_EFFECT_SET_RATE(agent, 1.25);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("rot"), 0, 20, 0, 90, 0, 0, 1.2, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.5);
    }
}

//Up Special Grab Sound
unsafe extern "C" fn ssbexo_wario_up_special_grab_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_wario_jump02"));
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_wario_006"));
    }
}

//Up Special Grab Expression
unsafe extern "C" fn ssbexo_wario_up_special_grab_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Up Special Grounded Throw ACMD
unsafe extern "C" fn ssbexo_wario_up_special_grounded_throw_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 85, 95, 0, 55, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::CHECK_FINISH_CAMERA(agent, 1, 1);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
        sv_kinetic_energy!(set_speed, agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 3.5);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

//Up Special Aerial Throw ACMD
unsafe extern "C" fn ssbexo_wario_up_special_aerial_throw_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 85, 25, 0, 55, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::CHECK_FINISH_CAMERA(agent, 1, 1);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
        sv_kinetic_energy!(set_speed, agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 3.5);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

//Up Special Grounded Throw Effect
unsafe extern "C" fn ssbexo_wario_up_special_grounded_throw_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0.0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//Up Special Aerial Throw Effect
unsafe extern "C" fn ssbexo_wario_up_special_aerial_throw_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0.0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//Up Special Grounded Throw Sound
unsafe extern "C" fn ssbexo_wario_up_special_grounded_throw_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_02"));
        macros::PLAY_LANDING_SE(agent, Hash40::new("se_wario_landing02"));
    }
}

//Up Special Aerial Throw Sound
unsafe extern "C" fn ssbexo_wario_up_special_aerial_throw_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_02"));
    }
}

//Up Special Grounded Throw Expression
unsafe extern "C" fn ssbexo_wario_up_special_grounded_throw_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_L);
    }
}

//Up Special Aerial Throw Expression
unsafe extern "C" fn ssbexo_wario_up_special_aerial_throw_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

//Half Charge Down Special ACMD
unsafe extern "C" fn ssbexo_wario_half_charge_down_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.6);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 45, 68, 0, 50, 11.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_JUMP);
    }
    frame(agent.lua_state_agent, 7.8);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 1.0);
}

//Full Charge Down Special ACMD
unsafe extern "C" fn ssbexo_wario_full_charge_down_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.5);
    frame(agent.lua_state_agent, 7.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 22.0, 35, 65, 0, 50, 11.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_JUMP);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_CRITICAL_HIT);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_CRITICAL_HIT);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 80, 75, 0, 0, 7.0, 0.0, 9.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("wario")
    .game_acmd("game_specialhijump", ssbexo_wario_up_special_jump_acmd, Priority::Low)
    .effect_acmd("effect_specialhicatch", ssbexo_wario_up_special_grab_effect, Priority::Low)
    .sound_acmd("sound_specialhicatch", ssbexo_wario_up_special_grab_sound, Priority::Low)
    .expression_acmd("expression_specialhicatch", ssbexo_wario_up_special_grab_expression, Priority::Low)
    .game_acmd("game_specialhithrow", ssbexo_wario_up_special_grounded_throw_acmd, Priority::Low)
    .effect_acmd("effect_specialhithrow", ssbexo_wario_up_special_grounded_throw_effect, Priority::Low)
    .sound_acmd("sound_specialhithrow", ssbexo_wario_up_special_grounded_throw_sound, Priority::Low)
    .expression_acmd("expression_specialhithrow", ssbexo_wario_up_special_grounded_throw_expression, Priority::Low)
    .game_acmd("game_specialhithrowair", ssbexo_wario_up_special_aerial_throw_acmd, Priority::Low)
    .effect_acmd("effect_specialhithrowair", ssbexo_wario_up_special_aerial_throw_effect, Priority::Low)
    .sound_acmd("sound_specialhithrowair", ssbexo_wario_up_special_aerial_throw_sound, Priority::Low)
    .expression_acmd("expression_specialhithrowair", ssbexo_wario_up_special_aerial_throw_expression, Priority::Low)
    .game_acmd("game_speciallwlr", ssbexo_wario_half_charge_down_special_acmd, Priority::Low)
    .game_acmd("game_specialairlwlr", ssbexo_wario_half_charge_down_special_acmd, Priority::Low)
    .game_acmd("game_speciallwflyr", ssbexo_wario_full_charge_down_special_acmd, Priority::Low)
    .game_acmd("game_specialairlwflyr", ssbexo_wario_full_charge_down_special_acmd, Priority::Low)
    .install()
    ;
}