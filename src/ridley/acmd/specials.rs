use super::*;

//Up Special Wall Bounce Forward ACMD
unsafe extern "C" fn ssbexo_ridley_up_special_wall_bounce_forward_acmd(agent : &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 70, 0, 80, 6.5, 0.0, -5.0, 4.0, Some(0.0), Some(18.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

//Up Special Wall Bounce Back ACMD
unsafe extern "C" fn ssbexo_ridley_up_special_wall_bounce_back_acmd(agent : &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 70, 0, 80, 6.5, 0.0, 0.0, -4.0, Some(0.0), Some(23.0), Some(-4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

//Up Special Wall Bounce Up ACMD
unsafe extern "C" fn ssbexo_ridley_up_special_wall_bounce_up_acmd(agent : &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 355, 70, 0, 80, 6.5, 0.0, 19.0, -10.0, Some(0.0), Some(19.0), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Grounded Down Special Stab
unsafe extern "C" fn ssbexo_ridley_grounded_down_special_stab_acmd(agent : &mut L2CAgentBase) {
    let special_zoom_gfx = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_RIDLEY_INSTANCE_WORK_ID_INT_DISABLE_SPECIAL_LW_FINISH_COUNT) == 0 {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 45.0, 25, 10, 0, 70, 2.5, 0.0, 8.0, 28.5, Some(0.0), Some(7.0), Some(35.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_BODY_HEAD, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
            AttackModule::set_no_dead_all(agent.module_accessor, true, false);
            if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
                WorkModule::inc_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
                if special_zoom_gfx < 2 {
                    SlowModule::set_whole(agent.module_accessor, 8, 80);
                    macros::CAM_ZOOM_IN_arg5(agent, /*frames*/ 2.0,/*no*/ 0.0,/*zoom*/ 1.8,/*yrot*/ 0.0,/*xrot*/ 0.0);
                    EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_bg_criticalhit"), Hash40::new("top"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                    macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 0.0);
                    macros::PLAY_SE(agent, Hash40::new("se_common_criticalhit"));
                    macros::QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
                }
                if special_zoom_gfx >= 4 {
                    SlowModule::clear_whole(agent.module_accessor);
                    CameraModule::reset_all(agent.module_accessor);
                    EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
                    macros::CAM_ZOOM_OUT(agent);
                }
            }
        }
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_RIDLEY_INSTANCE_WORK_ID_FLAG_DISABLE_CRITICAL_SPECIAL_LW) == false {
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 45.0, 25, 10, 0, 70, 2.5, 0.0, 8.0, 28.5, Some(0.0), Some(7.0), Some(35.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
            if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
                WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
                if special_zoom_gfx < 2 {
                    SlowModule::set_whole(agent.module_accessor, 8, 80);
                    macros::CAM_ZOOM_IN_arg5(agent, /*frames*/ 2.0,/*no*/ 0.0,/*zoom*/ 1.8,/*yrot*/ 0.0,/*xrot*/ 0.0);
                    EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_bg_criticalhit"), Hash40::new("top"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                    macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 0.0);
                    macros::PLAY_SE(agent, Hash40::new("se_common_criticalhit"));
                    macros::QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
                }
                if special_zoom_gfx >= 4 {
                    SlowModule::clear_whole(agent.module_accessor);
                    CameraModule::reset_all(agent.module_accessor);
                    EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
                    macros::CAM_ZOOM_OUT(agent);
                }
            }
        }
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 361, 50, 0, 30, 2.5, 0.0, 8.0, 8.0, Some(0.0), Some(7.0), Some(29.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 0.2);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Grounded Down Special Release
unsafe extern "C" fn ssbexo_ridley_grounded_down_special_release_acmd(agent : &mut L2CAgentBase) {
    let capture_id = LinkModule::get_node_object_id(agent.module_accessor, *LINK_NO_CAPTURE) as u32;
    let capture_boma = sv_battle_object::module_accessor(capture_id);
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        macros::WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        if DamageModule::damage(capture_boma, 0) > 150.0 {
            macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_DEDEDE_FINAL, 0, 10.0, 57, 100, 150, 0, 2.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_TAIL);
        }
        else {
            macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_DEDEDE_FINAL, 0, 10.0, 57, 100, 150, 0, 2.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_TAIL);
        }
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_WORK_ID_FLAG_FINAL_ABS_SET);
        AttackModule::set_no_finish_camera(agent.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, true, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_THROW);
        JostleModule::set_status(agent.module_accessor, true);
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
}

//Aerial Down Special ACMD
unsafe extern "C" fn ssbexo_ridley_aerial_down_special_acmd(agent : &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("tail8"), 16.0, 300, 55, 0, 40, 5.0, 0.0, 0.0, 0.0, Some(12.0), Some(0.0), Some(0.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 2, 0, Hash40::new("tail8"), 16.0, 300, 55, 0, 40, 5.0, 0.0, 0.0, 0.0, Some(12.0), Some(0.0), Some(0.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 3, 0, Hash40::new("tail7"), 5.0, 361, 55, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 4, 0, Hash40::new("tail6"), 5.0, 361, 55, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 5, 0, Hash40::new("tail5"), 5.0, 361, 55, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 6, 0, Hash40::new("tail4"), 5.0, 361, 55, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 7, 0, Hash40::new("tail3"), 5.0, 361, 55, 0, 40, 3.0, 0.0, 0.0, 0.0, Some(-12.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("tail8"), 5.0, 361, 55, 0, 40, 5.0, 0.0, 0.0, 0.0, Some(12.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 2, 0, Hash40::new("tail7"), 5.0, 361, 55, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 3, 0, Hash40::new("tail6"), 5.0, 361, 55, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 4, 0, Hash40::new("tail5"), 5.0, 361, 55, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 5, 0, Hash40::new("tail4"), 5.0, 361, 55, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 6, 0, Hash40::new("tail3"), 5.0, 361, 55, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 7, 0, Hash40::new("tail2"), 5.0, 361, 55, 0, 40, 3.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Aerial Down Special Effect
unsafe extern "C" fn ssbexo_ridley_aerial_down_special_effect(agent : &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("tail8"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ridley_death_stab_flare"), Hash40::new("tail8"), 0, 0, 0, 0, 180, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("ridley_death_stab_line"), Hash40::new("top"), 0, -7, -8, 60, 0, 0, 0.9, true);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("tail8"), 10, -0.0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ridley_death_stab_flare"), false, true);
    }
}

//Aerial Down Special Sound
unsafe extern "C" fn ssbexo_ridley_aerial_down_special_sound(agent : &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ridley_special_l01"));
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ridley_special_l02"));
    }
}

//Aerial Down Special Expression
unsafe extern "C" fn ssbexo_ridley_aerial_down_special_expression(agent : &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, 0);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_piercel"), 0);
    }
}

//Aerial Down Special Landing ACMD
unsafe extern "C" fn ssbexo_ridley_aerial_down_special_landing_acmd(agent : &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 10.0);
    MotionModule::set_rate(agent.module_accessor, 0.5);
    frame(agent.lua_state_agent, 30.0);
    MotionModule::set_rate(agent.module_accessor, 1.0);
}

//Aerial Down Special Landing Effect
unsafe extern "C" fn ssbexo_ridley_aerial_down_special_landing_effect(agent : &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ridley_death_stab_flare"), false, true);
        EffectModule::set_visible_kind(agent.module_accessor, Hash40::new("ridley_death_stab_line"), false);
        EffectModule::set_visible_kind(agent.module_accessor, Hash40::new("sys_sp_flash"), false);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), -18, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Down Special Landing Sound
unsafe extern "C" fn ssbexo_ridley_aerial_down_special_landing_sound(agent : &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ridley_landing03"));
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ridley_attackair_f01"));
    }
}

//Aerial Down Special Landing Expression
unsafe extern "C" fn ssbexo_ridley_aerial_down_special_landing_expression(agent : &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_landl_hv"), 0, false, 0);
    }
}

pub fn install() {
    Agent::new("ridley")
    .game_acmd("game_specialairhiwallf", ssbexo_ridley_up_special_wall_bounce_forward_acmd, Priority::Low)
    .game_acmd("game_specialairhiwallb", ssbexo_ridley_up_special_wall_bounce_back_acmd, Priority::Low)
    .game_acmd("game_specialairhiceil", ssbexo_ridley_up_special_wall_bounce_up_acmd, Priority::Low)
    .game_acmd("game_speciallwrelease", ssbexo_ridley_grounded_down_special_stab_acmd, Priority::Low)
    .game_acmd("game_speciallwfinish", ssbexo_ridley_grounded_down_special_release_acmd, Priority::Low)
    .game_acmd("game_specialairlwstab", ssbexo_ridley_aerial_down_special_acmd, Priority::Low)
    .effect_acmd("effect_specialairlwstab", ssbexo_ridley_aerial_down_special_effect, Priority::Low)
    .sound_acmd("sound_specialairlwstab", ssbexo_ridley_aerial_down_special_sound, Priority::Low)
    .expression_acmd("expression_specialairlwstab", ssbexo_ridley_aerial_down_special_expression, Priority::Low)
    .game_acmd("game_speciallwpogolanding", ssbexo_ridley_aerial_down_special_landing_acmd, Priority::Low)
    .effect_acmd("effect_speciallwpogolanding", ssbexo_ridley_aerial_down_special_landing_effect, Priority::Low)
    .sound_acmd("sound_speciallwpogolanding", ssbexo_ridley_aerial_down_special_landing_sound, Priority::Low)
    .expression_acmd("expression_speciallwpogolanding", ssbexo_ridley_aerial_down_special_landing_expression, Priority::Low)
    .install()
    ;
}