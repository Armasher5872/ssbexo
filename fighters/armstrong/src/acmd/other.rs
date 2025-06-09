use super::*;

//Entry Effect
unsafe extern "C" fn ssbexo_armstrong_entry_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 39.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_machstamp"), Hash40::new("top"), 0, 0, 0.0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
    }
    frame(agent.lua_state_agent, 43.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("edge_gokumon_impact"), Hash40::new("top"), -2.3, 0, 2, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.5);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0.0, 0.0, 0, 0.0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.75);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 0.0, 0.0, 0, 0.0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.75);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_machstamp"), false, false);
    }
    frame(agent.lua_state_agent, 60.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("edge_gokumon_impact"), Hash40::new("top"), -2.3, 0, 2, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        LAST_EFFECT_SET_RATE(agent, 8.0);
    }
    frame(agent.lua_state_agent, 61.0);
    if is_excute(agent) {
        LAST_EFFECT_SET_RATE(agent, 0.3);
    }
}

//Entry Sound
unsafe extern "C" fn ssbexo_armstrong_entry_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_appear01"));
    }
    frame(agent.lua_state_agent, 41.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_step_right_m"));
        PLAY_SE(agent, Hash40::new("se_common_heavy_hit_l"));
    }
    frame(agent.lua_state_agent, 100.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_step_right_m"));
    }
    frame(agent.lua_state_agent, 110.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_step_left_m"));
    }
}

//Entry Expression
unsafe extern "C" fn ssbexo_armstrong_entry_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BACKSHIELD_INVISIBLE);
    }
    frame(agent.lua_state_agent, 41.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 1);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BACKSHIELD_INVISIBLE);
        QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, 0);
    }
}

//Taunt Effect
unsafe extern "C" fn ssbexo_armstrong_taunt_effect(_agent: &mut L2CAgentBase) {}

//Up Taunt Sound
unsafe extern "C" fn ssbexo_armstrong_up_taunt_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_appeal_h01"));
    }
}

//Side Taunt Sound
unsafe extern "C" fn ssbexo_armstrong_side_taunt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 53.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_appeal_s01"));
    }
}

//Down Taunt ACMD
unsafe extern "C" fn ssbexo_armstrong_down_taunt_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0.0);
        HitModule::set_check_catch(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 80.0);
    if is_excute(agent) {
        HitModule::set_check_catch(agent.module_accessor, true, 0);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
    }
}

//Down Taunt Sound
unsafe extern "C" fn ssbexo_armstrong_down_taunt_sound(_agent: &mut L2CAgentBase) {}

//Down Taunt Expression
unsafe extern "C" fn ssbexo_armstrong_down_taunt_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 96.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

//Ledge Attack Effect
unsafe extern "C" fn ssbexo_armstrong_ledge_attack_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 0, 0, 4, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 1, 10, 4, 0, 25, 25, 1.25, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        LAST_EFFECT_SET_RATE(agent, 2.0);
    }
}

//Face Down Getup Attack Effect
unsafe extern "C" fn ssbexo_armstrong_face_down_getup_attack_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 2.5, 5, -15, 180, 0, 1.4, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        LAST_EFFECT_SET_RATE(agent, 2.0);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -7, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 8, -18.5, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true, 1.5);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -0.5, 10, 0.8, 180, -230, 90, 1.4, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 14, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 27.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_attack_arc_d"), false, false);
    }
}

//Face Up Getup Attack Effect
unsafe extern "C" fn ssbexo_armstrong_face_up_getup_attack_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -3, 6, -1, 0, 180, 15, 1.6, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        LAST_EFFECT_SET_RATE(agent, 1.7);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_attack_arc_d"), false, false);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 5.5, 0, 0, 40, 13, 1.5, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
}

//Standing Tech Sound
unsafe extern "C" fn ssbexo_armstrong_passive_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_blowaway_s"));
        STOP_SE(agent, Hash40::new("se_common_blowaway_m"));
        STOP_SE(agent, Hash40::new("se_common_blowaway_l"));
        PLAY_SE(agent, Hash40::new("vc_ganon_cliffcatch"));
        PLAY_LANDING_SE(agent, Hash40::new("se_ganon_landing01"));
    }
}

//Ceiling Tech Sound
unsafe extern "C" fn ssbexo_armstrong_passive_ceil_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_blowaway_s"));
        STOP_SE(agent, Hash40::new("se_common_blowaway_m"));
        STOP_SE(agent, Hash40::new("se_common_blowaway_l"));
        PLAY_SE(agent, Hash40::new("vc_ganon_cliffcatch"));
        PLAY_LANDING_SE(agent, Hash40::new("se_ganon_landing01"));
    }
}

//Wall Tech Sound
unsafe extern "C" fn ssbexo_armstrong_passive_wall_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_blowaway_s"));
        STOP_SE(agent, Hash40::new("se_common_blowaway_m"));
        STOP_SE(agent, Hash40::new("se_common_blowaway_l"));
        PLAY_SE(agent, Hash40::new("vc_ganon_cliffcatch"));
        PLAY_SE(agent, Hash40::new("se_ganon_landing01"));
    }
}

//Final Smash Start
unsafe extern "C" fn ssbexo_armstrong_final_smash_start_acmd(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_CHARGE) {
        if is_excute(agent) {
            CHECK_VALID_FINAL_START_CAMERA(agent, 0, 40, 15, 0, 0, 0);
            REQ_FINAL_START_CAMERA_arg3(agent, Hash40::new("d04finalstart.nuanmb"), false, false);
            FT_SET_FINAL_FEAR_FACE(agent, 60);
            FT_START_CUTIN(agent);
        }
        frame(agent.lua_state_agent, 27.0);
        if is_excute(agent) {
            CAM_ZOOM_OUT(agent);
            ATTACK(agent, 0, 0, Hash40::new("top"), 40.0, 361, 40, 0, 40, 10.0, 0.0, 9.0, 6.0, None, None, None, 3.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze_ghost"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
            AttackModule::set_paralyze_frame(agent.module_accessor, 0, 120, false);
        }
        frame(agent.lua_state_agent, 37.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    else {
        if is_excute(agent) {
            WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
            SLOW_OPPONENT(agent, 8.0, 70.0);
            SlowModule::set_whole(agent.module_accessor, 2, 20);
            CHECK_VALID_FINAL_START_CAMERA(agent, 0, 40, 15, 0, 0, 0);
            REQ_FINAL_START_CAMERA_arg3(agent, Hash40::new("d04finalstart.nuanmb"), false, false);
            FT_SET_FINAL_FEAR_FACE(agent, 60);
            FT_START_CUTIN(agent);
        }
        frame(agent.lua_state_agent, 27.0);
        if is_excute(agent) {
            CAM_ZOOM_OUT(agent);
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 0.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        frame(agent.lua_state_agent, 28.0);
        if is_excute(agent) {
            CATCH(agent, 0, Hash40::new("top"), 14.0, 0.0, 9.0, 6.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        }
        frame(agent.lua_state_agent, 37.0);
        if is_excute(agent) {
            grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        }
    }
}

//Grounded Final Smash Start Effect
unsafe extern "C" fn ssbexo_armstrong_grounded_final_smash_start_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EffectModule::req_screen(agent.module_accessor, Hash40::new("bg_ganon_final"), false, false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_final_aura"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_hold"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
    }
    frame(agent.lua_state_agent, 27.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("handr"), 0.5, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("kneel"), 1, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("head"), 1, 0, 0, 0, 0, 0, 1.2, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("handl"), 0.5, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("kneer"), 1, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 44.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Final Smash Start Effect
unsafe extern "C" fn ssbexo_armstrong_aerial_final_smash_start_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EffectModule::req_screen(agent.module_accessor, Hash40::new("bg_ganon_final"), false, false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_final_aura"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_hold"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
    }
    frame(agent.lua_state_agent, 27.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("handr"), 0.5, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("kneel"), 1, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("head"), 1, 0, 0, 0, 0, 0, 1.2, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("handl"), 0.5, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("kneer"), 1, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 30.0);
    for _ in 0..=4 {
        if is_excute(agent) {
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, 2, 0, 180, 0, 1.5, true);
            LAST_PARTICLE_SET_COLOR(agent, 0.88, 0.35, 0.13);
        }
        wait(agent.lua_state_agent, 3.0);
    }
}

//Final Smash Start Sound
unsafe extern "C" fn ssbexo_armstrong_final_smash_start_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_attack05"));
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_swing_10"));
    }
    wait(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_swing_10"));
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_attackhard_03"));
    }
}

//Grounded Final Smash Start Expression
unsafe extern "C" fn ssbexo_armstrong_grounded_final_smash_start_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        sv_animcmd::START_INFO_FLASH_EYE(agent.lua_state_agent);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0.0, 3.0, 200.0, 8.0, 1.0, -4.0, 8.0, 30.0, 20.0, 50.0);
        sv_animcmd::AREA_WIND_2ND_arg10(agent.lua_state_agent);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_rush"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 44.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

//Aerial Final Smash Start Expression
unsafe extern "C" fn ssbexo_armstrong_aerial_final_smash_start_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        sv_animcmd::START_INFO_FLASH_EYE(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 0.0, 3.0, 200.0, 8.0, 1.0, -4.0, 8.0, 30.0, 20.0, 50.0);
        sv_animcmd::AREA_WIND_2ND_arg10(agent.lua_state_agent);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_rush"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Final Smash Throw ACMD
unsafe extern "C" fn ssbexo_armstrong_final_smash_throw_acmd(agent: &mut L2CAgentBase) {
    let grabbed_boma = get_grabbed_opponent_boma(agent.module_accessor);
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 20.0, 0, 50, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        REQ_FINAL_START_CAMERA_arg3(agent, Hash40::new("d04finalthrow.nuanmb"), false, false);
        FT_SET_FINAL_FEAR_FACE(agent, 200);
        FT_START_CUTIN(agent);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("handr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 38.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("handr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 44.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 65.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("handr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 68.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 83.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("handr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 86.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 96.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("handr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 99.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 113.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("handr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 116.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 126.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("handr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 129.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 143.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("handr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 146.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 156.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("handr"), 6.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 159.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 195.0);
    if is_excute(agent) {
        if DamageModule::damage(grabbed_boma, 0) > 100.0 {
            VisibilityModule::set_whole(grabbed_boma, true);
            ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 10.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("armr"), 10.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 2, 0, Hash40::new("handr"), 10.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 10.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("armr"), 10.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 2, 0, Hash40::new("handr"), 10.0, 0, 70, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        }
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 200.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 201.0);
    if is_excute(agent) {
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        ATTACK(agent, 0, 0, Hash40::new("top"), 30.0, 361, 40, 0, 70, 25.0, 0.0, 13.0, 0.0, None, None, None, 3.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        VisibilityModule::set_whole(grabbed_boma, true);
        CAM_ZOOM_OUT(agent);
    }
    frame(agent.lua_state_agent, 205.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Final Smash Throw Effect
unsafe extern "C" fn ssbexo_armstrong_final_smash_throw_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EffectModule::req_screen(agent.module_accessor, Hash40::new("bg_ganon_final"), false, false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_final_aura"), false, true);
    }
    frame(agent.lua_state_agent, 201.0);
    if is_excute(agent) {
        EffectModule::remove_screen(agent.module_accessor, Hash40::new("bg_ganon_final"), -1);
        EFFECT(agent, Hash40::new("edge_gokumon_impact"), Hash40::new("top"), -2.3, 0, 2, 0, 0, 0, 5.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.5);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0.0, 0.0, 0, 0.0, 0, 0, 2.5, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.75);
        LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 0.0, 0.0, 0, 0.0, 0, 0, 2.5, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.75);
    }
}

//Final Smash Throw Sound
unsafe extern "C" fn ssbexo_armstrong_final_smash_throw_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 36.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));
    }
    frame(agent.lua_state_agent, 80.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));
    }
    frame(agent.lua_state_agent, 110.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));
    }
    frame(agent.lua_state_agent, 140.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));
    }
    frame(agent.lua_state_agent, 192.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));
    }
}

//Final Smash Throw Expression
unsafe extern "C" fn ssbexo_armstrong_final_smash_throw_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("rbkind_spinattack"), 0);
        sv_animcmd::RUMBLE_HIT_STATUS(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 200.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attack_critical"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .effect_acmd("effect_entryr", ssbexo_armstrong_entry_effect, Low)
    .effect_acmd("effect_entryl", ssbexo_armstrong_entry_effect, Low)
    .sound_acmd("sound_entryr", ssbexo_armstrong_entry_sound, Low)
    .sound_acmd("sound_entryl", ssbexo_armstrong_entry_sound, Low)
    .expression_acmd("expression_entryr", ssbexo_armstrong_entry_expression, Low)
    .expression_acmd("expression_entryl", ssbexo_armstrong_entry_expression, Low)
    .effect_acmd("effect_appealhir", ssbexo_armstrong_taunt_effect, Low)
    .effect_acmd("effect_appealhil", ssbexo_armstrong_taunt_effect, Low)
    .effect_acmd("effect_appealsr", ssbexo_armstrong_taunt_effect, Low)
    .effect_acmd("effect_appealsl", ssbexo_armstrong_taunt_effect, Low)
    .effect_acmd("effect_appeallwr", ssbexo_armstrong_taunt_effect, Low)
    .effect_acmd("effect_appeallwl", ssbexo_armstrong_taunt_effect, Low)
    .expression_acmd("expression_appeallwr", ssbexo_armstrong_down_taunt_expression, Low)
    .expression_acmd("expression_appeallwl", ssbexo_armstrong_down_taunt_expression, Low)
    .sound_acmd("sound_appealhir", ssbexo_armstrong_up_taunt_sound, Low)
    .sound_acmd("sound_appealhil", ssbexo_armstrong_up_taunt_sound, Low)
    .sound_acmd("sound_appealsr", ssbexo_armstrong_side_taunt_sound, Low)
    .sound_acmd("sound_appealsl", ssbexo_armstrong_side_taunt_sound, Low)
    .game_acmd("game_appeallwr", ssbexo_armstrong_down_taunt_acmd, Low)
    .game_acmd("game_appeallwl", ssbexo_armstrong_down_taunt_acmd, Low)
    .sound_acmd("sound_appeallwr", ssbexo_armstrong_down_taunt_sound, Low)
    .sound_acmd("sound_appeallwl", ssbexo_armstrong_down_taunt_sound, Low)
    .effect_acmd("effect_cliffattack", ssbexo_armstrong_ledge_attack_effect, Low)
    .effect_acmd("effect_attackdownd", ssbexo_armstrong_face_down_getup_attack_effect, Low)
    .effect_acmd("effect_attackdownu", ssbexo_armstrong_face_up_getup_attack_effect, Low)
    .sound_acmd("sound_passive", ssbexo_armstrong_passive_sound, Low)
    .sound_acmd("sound_passiveceil", ssbexo_armstrong_passive_ceil_sound, Low)
    .sound_acmd("sound_passivewall", ssbexo_armstrong_passive_wall_sound, Low)
    .game_acmd("game_finalstart", ssbexo_armstrong_final_smash_start_acmd, Low)
    .game_acmd("game_finalairstart", ssbexo_armstrong_final_smash_start_acmd, Low)
    .effect_acmd("effect_finalstart", ssbexo_armstrong_grounded_final_smash_start_effect, Low)
    .effect_acmd("effect_finalairstart", ssbexo_armstrong_aerial_final_smash_start_effect, Low)
    .sound_acmd("sound_finalstart", ssbexo_armstrong_final_smash_start_sound, Low)
    .sound_acmd("sound_finalairstart", ssbexo_armstrong_final_smash_start_sound, Low)
    .expression_acmd("expression_finalstart", ssbexo_armstrong_grounded_final_smash_start_expression, Low)
    .expression_acmd("expression_finalairstart", ssbexo_armstrong_aerial_final_smash_start_expression, Low)
    .game_acmd("game_finalthrow", ssbexo_armstrong_final_smash_throw_acmd, Low)
    .game_acmd("game_finalairthrow", ssbexo_armstrong_final_smash_throw_acmd, Low)
    .effect_acmd("effect_finalthrow", ssbexo_armstrong_final_smash_throw_effect, Low)
    .effect_acmd("effect_finalairthrow", ssbexo_armstrong_final_smash_throw_effect, Low)
    .sound_acmd("sound_finalthrow", ssbexo_armstrong_final_smash_throw_sound, Low)
    .sound_acmd("sound_finalairthrow", ssbexo_armstrong_final_smash_throw_sound, Low)
    .expression_acmd("expression_finalthrow", ssbexo_armstrong_final_smash_throw_expression, Low)
    .expression_acmd("expression_finalairthrow", ssbexo_armstrong_final_smash_throw_expression, Low)
    .install()
    ;
}