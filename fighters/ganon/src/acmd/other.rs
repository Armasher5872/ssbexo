use super::*;

//Dash Sound
unsafe extern "C" fn ssbexo_ganon_dash_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_dash_start"));
        SET_PLAY_INHIVIT(agent, Hash40::new("se_ganon_dash_start"), 20);
    }
}

//Turn Dash Sound
unsafe extern "C" fn ssbexo_ganon_turn_dash_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_dash_start"));
        SET_PLAY_INHIVIT(agent, Hash40::new("se_ganon_dash_start"), 20);
    }
}

//Run Sound
unsafe extern "C" fn ssbexo_ganon_run_sound(_agent: &mut L2CAgentBase) {}

//Guard On ACMD
unsafe extern "C" fn ssbexo_ganon_guard_on_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

//Guard Off ACMD
unsafe extern "C" fn ssbexo_ganon_guard_off_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

//Guard Off Sound
unsafe extern "C" fn ssbexo_ganon_guard_off_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_guardoff"));
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        let swipe = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ganon_special_n07"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, swipe as i32, 2.0, 0);
    }
}

//Back Roll Effect
unsafe extern "C" fn ssbexo_ganon_back_roll_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
} 

//Back Roll Sound
unsafe extern "C" fn ssbexo_ganon_back_roll_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_escape"));
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_landing01"));
    }
}

//Air Dodge Effect
unsafe extern "C" fn ssbexo_ganon_air_dodge_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
}

//Up Taunt Effect
unsafe extern "C" fn ssbexo_ganon_up_taunt_effect(_agent: &mut L2CAgentBase) {}

//Up Taunt Sound
unsafe extern "C" fn ssbexo_ganon_up_taunt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_appeal_h01"));
    }
}

//Side Taunt Effect
unsafe extern "C" fn ssbexo_ganon_side_taunt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_final_hand_triforce"), Hash40::new("haver"), -1.1, -0.3, -0.2, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 0.3);
    }
    frame(agent.lua_state_agent, 100.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Side Taunt Sound
unsafe extern "C" fn ssbexo_ganon_side_taunt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        let glow = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ganon_appeal_s03"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, glow as i32, 12.0, 0);
    }
    frame(agent.lua_state_agent, 95.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_appeal_s01"));
    }
}

//Side Taunt Expression
unsafe extern "C" fn ssbexo_ganon_side_taunt_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 100.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 18, true, *BATTLE_OBJECT_ID_INVALID as u32);
        AREA_WIND_2ND_arg10(agent, 0, 3, 110, 2, 0.6, 0, 15, 30, 30, 40);
    }
    frame(agent.lua_state_agent, 140.0);
    if is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
}

//Side Taunt Attack ACMD
unsafe extern "C" fn ssbexo_ganon_side_taunt_attack_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(agent.lua_state_agent, 76.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(agent.lua_state_agent, 77.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 40.0, 30, 100, 0, 40, 6.0, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("bust"), 40.0, 30, 100, 0, 40, 6.0, 0.0, 1.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 40.0, 30, 100, 0, 40, 6.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 80.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Side Taunt Attack Effect
unsafe extern "C" fn ssbexo_ganon_side_taunt_attack_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_start"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        let effect = EffectModule::req_follow(agent.module_accessor, Hash40::new("ganon_final_hand_triforce"), Hash40::new("haver"), &Vector3f{x: -1.1, y: -0.3, z: -0.2}, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::set_rate(agent.module_accessor, effect as u32, 0.3); 
        WorkModule::set_int(agent.module_accessor, effect as i32, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_EFFECT_HANDLE);
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 66.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_hold"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 77.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken"), Hash40::new("top"), 0, 12.5, 22.5, 0, -10, 0, 1, true);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(agent.lua_state_agent, 100.0);
    if is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_EFFECT_HANDLE);
    }
}

//Side Taunt Attack Sound
unsafe extern "C" fn ssbexo_ganon_side_taunt_attack_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_ganon_appeal_s03"));
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_special_n01"));
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_n01"));
    }
    wait(agent.lua_state_agent, 58.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_n02"));
    }
}

//Side Taunt Attack Expression
unsafe extern "C" fn ssbexo_ganon_side_taunt_attack_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        AREA_WIND_2ND_arg10(agent, 0, 2, 75, 2, 1, 0, 12, 50, 30, 50);
    }
    frame(agent.lua_state_agent, 35.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 36.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_23_hold"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 56.0);
    if is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
    frame(agent.lua_state_agent, 64.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 68.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 70.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attack_critical"), 0);
    }
    frame(agent.lua_state_agent, 71.0);
    if is_excute(agent) {
        AREA_WIND_2ND_arg10(agent, 0, 3, 120, 2, 1, 0, 12, 50, 30, 0);
    }
    frame(agent.lua_state_agent, 101.0);
    if is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
}

//Win 1 Effect
unsafe extern "C" fn ssbexo_ganon_win_1_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 110.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_final_hand_triforce"), Hash40::new("haver"), -1.1, -0.3, -0.2, 0, 0, 0, 1, true);
    }
}

//Win 1 Sound
unsafe extern "C" fn ssbexo_ganon_win_1_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 35.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("vc_ganon_win01"));
    }
    frame(agent.lua_state_agent, 51.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_swing_l"));
    }
    frame(agent.lua_state_agent, 110.0);
    if is_excute(agent) {
        let glow = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ganon_appeal_s03"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, glow as i32, 12.0, 0);
    }
}

//Win 1 Wait Effect
unsafe extern "C" fn ssbexo_ganon_win_1_wait_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_final_hand_triforce"), Hash40::new("haver"), -1.1, -0.3, -0.2, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 0.2);
    }
}

//Win 1 Wait Sound
unsafe extern "C" fn ssbexo_ganon_win_1_wait_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_ganon_appeal_s03"));
        let glow = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ganon_appeal_s03"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, glow as i32, 12.0, 0);
        LAST_EFFECT_SET_RATE(agent, 0.2);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .sound_acmd("sound_dash", ssbexo_ganon_dash_sound, Low)
    .sound_acmd("sound_turndash", ssbexo_ganon_turn_dash_sound, Low)
    .sound_acmd("sound_run", ssbexo_ganon_run_sound, Low)
    .game_acmd("game_guardon", ssbexo_ganon_guard_off_acmd, Low)
    .game_acmd("game_guardoff", ssbexo_ganon_guard_on_acmd, Low)
    .sound_acmd("sound_guardoff", ssbexo_ganon_guard_off_sound, Low)
    .effect_acmd("effect_escapeb", ssbexo_ganon_back_roll_effect, Low)
    .sound_acmd("sound_escapeb", ssbexo_ganon_back_roll_sound, Low)
    .effect_acmd("effect_escapeair", ssbexo_ganon_air_dodge_effect, Low)
    .effect_acmd("effect_escapeairslide", ssbexo_ganon_air_dodge_effect, Low)
    .effect_acmd("effect_appealhir", ssbexo_ganon_up_taunt_effect, Low)
    .effect_acmd("effect_appealhil", ssbexo_ganon_up_taunt_effect, Low)
    .sound_acmd("sound_appealhir", ssbexo_ganon_up_taunt_sound, Low)
    .sound_acmd("sound_appealhil", ssbexo_ganon_up_taunt_sound, Low)
    .effect_acmd("effect_appealsr", ssbexo_ganon_side_taunt_effect, Low)
    .sound_acmd("sound_appealsr", ssbexo_ganon_side_taunt_sound, Low)
    .expression_acmd("expression_appealsr", ssbexo_ganon_side_taunt_expression, Low)
    .effect_acmd("effect_appealsl", ssbexo_ganon_side_taunt_effect, Low)
    .sound_acmd("sound_appealsl", ssbexo_ganon_side_taunt_sound, Low)
    .expression_acmd("expression_appealsl", ssbexo_ganon_side_taunt_expression, Low)
    .game_acmd("game_appealsattack", ssbexo_ganon_side_taunt_attack_acmd, Low)
    .effect_acmd("effect_appealsattack", ssbexo_ganon_side_taunt_attack_effect, Low)
    .sound_acmd("sound_appealsattack", ssbexo_ganon_side_taunt_attack_sound, Low)
    .expression_acmd("expression_appealsattack", ssbexo_ganon_side_taunt_attack_expression, Low)
    .effect_acmd("effect_win1", ssbexo_ganon_win_1_effect, Low)
    .sound_acmd("sound_win1", ssbexo_ganon_win_1_sound, Low)
    .effect_acmd("effect_win1wait", ssbexo_ganon_win_1_wait_effect, Low)
    .sound_acmd("sound_win1wait", ssbexo_ganon_win_1_wait_sound, Low)
    .install()
    ;
}