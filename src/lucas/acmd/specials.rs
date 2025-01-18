use super::*;

//Shield Special Effect
unsafe extern "C" fn ssbexo_lucas_shield_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.3, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.3, true);
    }
    frame(agent.lua_state_agent, 16.0);
    for _ in 0..13 {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_bullet"), true, true);
            macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.6, true);
            macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.6, true);
            macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_hold"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 1, true);
            macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 9, 0, 9, 0, 0, 0, false);
            macros::FLASH(agent, 0.01, 0.5, 1, 0.4);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 1, 1, 1, 0.6);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 5.0);
    }
    frame(agent.lua_state_agent, 108.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_bullet"), true, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_hold"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.9, true);
        macros::EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 11, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    for _ in 0..5 {
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.01, 0.5, 1, 0.4);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 1, 1, 1, 0.6);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 3.0);
    }
    frame(agent.lua_state_agent, 135.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_hold"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_bullet"), false, false);
    }
}

//Shield Special Sound
unsafe extern "C" fn ssbexo_lucas_shield_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucas_special_n05"));
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_lucas_pk_charge"));
    }
    frame(agent.lua_state_agent, 70.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_lucas_pk_charge"));
        macros::PLAY_SE(agent, Hash40::new("se_lucas_special_n02"));
    }
    frame(agent.lua_state_agent, 115.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_lucas_attack05"));
        macros::PLAY_SE(agent, Hash40::new("se_lucas_special_n04_ll"));
    }
}

//Shield Special Expression
unsafe extern "C" fn ssbexo_lucas_shield_special_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(agent.lua_state_agent, 16.0);
    for _ in 0..13 {
        if macros::is_excute(agent) {
            macros::AREA_WIND_2ND_arg10(agent, 0, 1, 90, 3, 0.2, 0, 5, 40, 10, 80);
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(agent.lua_state_agent, 6.0);
    }
    frame(agent.lua_state_agent, 120.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_beamm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 140.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

//Shield Special Burst ACMD
unsafe extern "C" fn ssbexo_lucas_shield_special_burst_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 80, 70, 0, 50, 12.0, 0.0, 6.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Shield Special Burst Effect
unsafe extern "C" fn ssbexo_lucas_shield_special_burst_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.6, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bullet"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.6, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_hold"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.9, true);
        macros::EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 11, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        macros::FLASH(agent, 0.01, 0.5, 1, 0.4);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 1, 1, 1, 0.6);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_bullet"), false, false);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("lucas_pkfr_bomb_max"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.5, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_pkfr_bullet_ed"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.5, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_hold"), false, false);
    }
}

//Shield Special Burst Sound
unsafe extern "C" fn ssbexo_lucas_shield_special_burst_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_lucas_attack05"));
        macros::PLAY_SE(agent, Hash40::new("se_lucas_special_n04_ll"));
    }
}

//Shield Special Burst Expression
unsafe extern "C" fn ssbexo_lucas_shield_special_burst_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_beamm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_L);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

//Down Special ACMD
unsafe extern "C" fn ssbexo_lucas_down_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        ShieldModule::set_status(agent.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_LUCAS_SHIELD_GROUP_KIND_SPECIAL_LW_GUARD);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        ShieldModule::set_status(agent.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_LUCAS_SHIELD_GROUP_KIND_SPECIAL_LW_GUARD);
    }
}

//Grounded Down Special Effect
unsafe extern "C" fn ssbexo_lucas_grounded_down_special_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -1.5, 0, -1, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_hold"), Hash40::new("havel"), -0.3, 0, 0.1, 0, 0, 0, 0.8, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0, 0.0, 12.0, 0, 0, 0, 1.1, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.753, 0.796);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0, 12.0, 12.0, 0, 0, 0, 1.1, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.753, 0.796);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0, 6.0, 6.0, 0, 0, 0, 1.1, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.753, 0.796);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0, 6.0, 18.0, 0, 0, 0, 1.1, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.753, 0.796);
    }
    frame(agent.lua_state_agent, 7.0);
    for _ in 0..6 {
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0, 1, 1, 0.2);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 0.5, 1, 1, 0.4);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FLASH_FRM(agent, 10, 0, 1, 1, 0.1);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
}

//Aerial Down Special Effect
unsafe extern "C" fn ssbexo_lucas_aerial_down_special_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_hold"), Hash40::new("havel"), -0.3, 0, 0.1, 0, 0, 0, 0.8, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0, 2.0, 8.0, 0, 0, 0, 1.1, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.753, 0.796);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0, 12.0, 8.0, 0, 0, 0, 1.1, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.753, 0.796);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0, 6.0, 4.0, 0, 0, 0, 1.1, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.753, 0.796);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0, 6.0, 12.0, 0, 0, 0, 1.1, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.753, 0.796);
    }
    frame(agent.lua_state_agent, 7.0);
    for _ in 0..6 {
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0, 1, 1, 0.2);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 0.5, 1, 1, 0.4);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FLASH_FRM(agent, 10, 0, 1, 1, 0.1);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
}

//Down Special Sound
unsafe extern "C" fn ssbexo_lucas_down_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_lucas_special_l01"));
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_lucas_special_l01"));
    }
}

//Grounded Down Special Expression
unsafe extern "C" fn ssbexo_lucas_grounded_down_special_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_shield_off"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

//Aerial Down Special Expression
unsafe extern "C" fn ssbexo_lucas_aerial_down_special_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_shield_off"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

//Down Special Catch ACMD
unsafe extern "C" fn ssbexo_lucas_down_special_catch_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 13.0, 0.0, 6.5, 0.0, Some(0.0), Some(6.5), Some(11.5), *FIGHTER_STATUS_KIND_THROWN, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
}

//Down Special Catch Effect
unsafe extern "C" fn ssbexo_lucas_down_special_catch_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psimagnet_hit"), Hash40::new("trans"), 0, 6.5, 12, 0, 0, 0, 1, false);
        macros::FLASH(agent, 0.5, 1, 1, 0.4);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH_FRM(agent, 4, 0, 1, 1, 0.1);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::FLASH_FRM(agent, 6, 0, 0, 1, 0);
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
}

//Down Special Catch Sound
unsafe extern "C" fn ssbexo_lucas_down_special_catch_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucas_special_l02"));
    }
}

//Down Special Catch Expression
unsafe extern "C" fn ssbexo_lucas_down_special_catch_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Down Special Throw ACMD
unsafe extern "C" fn ssbexo_lucas_down_special_throw_acmd(agent: &mut L2CAgentBase) {
    let angle = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ATTACK_ANGLE);
    let speed_x = (angle as f32).to_radians().sin()*3.0*PostureModule::lr(agent.module_accessor);
    let speed_y = (angle as f32).to_radians().cos()*3.0;
    let inverse_angle;
    if angle < 361 {
        if angle < 180 {
            inverse_angle = angle+180;
        }
        else {
            inverse_angle = angle-180;
        }
    }
    else {
        inverse_angle = 361;
    }
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 15.0, inverse_angle as u64, 60, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        macros::CHECK_FINISH_CAMERA(agent, 7, 16);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 8.0, z: 0.0});
        StatusModule::set_situation_kind(agent.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_PSI_COUNTER_THROWN);
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 0.0, y: speed_y, z: speed_x});
    }
}

//Down Special Throw Effect
unsafe extern "C" fn ssbexo_lucas_down_special_throw_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_atk"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.9);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_atk"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.9);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_atk"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.9);
    }
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucas_psi_atk_lw"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.2, true);
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}

//Down Special Throw Sound
unsafe extern "C" fn ssbexo_lucas_down_special_throw_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucas_throw_h01"));
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    wait(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucas_throw_h02"));
        macros::PLAY_SE(agent, Hash40::new("vc_lucas_attack05"));
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_02"));
    }
}

//Down Special Throw Expression
unsafe extern "C" fn ssbexo_lucas_down_special_throw_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 32, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
}

pub fn install() {
    Agent::new("lucas")
    .effect_acmd("effect_specialguard", ssbexo_lucas_shield_special_effect, Priority::Low)
    .sound_acmd("sound_specialguard", ssbexo_lucas_shield_special_sound, Priority::Low)
    .expression_acmd("expression_specialguard", ssbexo_lucas_shield_special_expression, Priority::Low)
    .game_acmd("game_specialguardburst", ssbexo_lucas_shield_special_burst_acmd, Priority::Low)
    .effect_acmd("effect_specialguardburst", ssbexo_lucas_shield_special_burst_effect, Priority::Low)
    .sound_acmd("sound_specialguardburst", ssbexo_lucas_shield_special_burst_sound, Priority::Low)
    .expression_acmd("expression_specialguardburst", ssbexo_lucas_shield_special_burst_expression, Priority::Low)
    .game_acmd("game_speciallw", ssbexo_lucas_down_special_acmd, Priority::Low)
    .game_acmd("game_specialairlw", ssbexo_lucas_down_special_acmd, Priority::Low)
    .effect_acmd("effect_speciallw", ssbexo_lucas_grounded_down_special_effect, Priority::Low)
    .effect_acmd("effect_specialairlw", ssbexo_lucas_aerial_down_special_effect, Priority::Low)
    .sound_acmd("sound_speciallw", ssbexo_lucas_down_special_sound, Priority::Low)
    .sound_acmd("sound_specialairlw", ssbexo_lucas_down_special_sound, Priority::Low)
    .expression_acmd("expression_speciallw", ssbexo_lucas_grounded_down_special_expression, Priority::Low)
    .expression_acmd("expression_specialairlw", ssbexo_lucas_aerial_down_special_expression, Priority::Low)
    .game_acmd("game_speciallwcatch", ssbexo_lucas_down_special_catch_acmd, Priority::Low)
    .game_acmd("game_specialairlwcatch", ssbexo_lucas_down_special_catch_acmd, Priority::Low)
    .effect_acmd("effect_speciallwcatch", ssbexo_lucas_down_special_catch_effect, Priority::Low)
    .effect_acmd("effect_specialairlwcatch", ssbexo_lucas_down_special_catch_effect, Priority::Low)
    .sound_acmd("sound_speciallwcatch", ssbexo_lucas_down_special_catch_sound, Priority::Low)
    .sound_acmd("sound_specialairlwcatch", ssbexo_lucas_down_special_catch_sound, Priority::Low)
    .expression_acmd("expression_speciallwcatch", ssbexo_lucas_down_special_catch_expression, Priority::Low)
    .expression_acmd("expression_specialairlwcatch", ssbexo_lucas_down_special_catch_expression, Priority::Low)
    .game_acmd("game_speciallwthrow", ssbexo_lucas_down_special_throw_acmd, Priority::Low)
    .game_acmd("game_specialairlwthrow", ssbexo_lucas_down_special_throw_acmd, Priority::Low)
    .effect_acmd("effect_speciallwthrow", ssbexo_lucas_down_special_throw_effect, Priority::Low)
    .effect_acmd("effect_specialairlwthrow", ssbexo_lucas_down_special_throw_effect, Priority::Low)
    .sound_acmd("sound_speciallwthrow", ssbexo_lucas_down_special_throw_sound, Priority::Low)
    .sound_acmd("sound_specialairlwthrow", ssbexo_lucas_down_special_throw_sound, Priority::Low)
    .expression_acmd("expression_speciallwthrow", ssbexo_lucas_down_special_throw_expression, Priority::Low)
    .expression_acmd("expression_specialairlwthrow", ssbexo_lucas_down_special_throw_expression, Priority::Low)
    .install()
    ;
}