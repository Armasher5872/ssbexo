use super::*;

//Up Special Wall Bounce Forward ACMD
#[acmd_script( agent = "ridley", script = "game_specialairhiwallf", category = ACMD_GAME )]
unsafe fn ssbuexo_ridley_up_special_wall_bounce_forward_acmd(fighter : &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 70, 0, 80, 6.5, 0.0, -5.0, 4.0, Some(0.0), Some(18.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
    }
    frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

//Up Special Wall Bounce Back ACMD
#[acmd_script( agent = "ridley", script = "game_specialairhiwallb", category = ACMD_GAME )]
unsafe fn ssbuexo_ridley_up_special_wall_bounce_back_acmd(fighter : &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 70, 0, 80, 6.5, 0.0, 0.0, -4.0, Some(0.0), Some(23.0), Some(-4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

//Up Special Wall Bounce Up ACMD
#[acmd_script( agent = "ridley", script = "game_specialairhiceil", category = ACMD_GAME )]
unsafe fn ssbuexo_ridley_up_special_wall_bounce_up_acmd(fighter : &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 355, 70, 0, 80, 6.5, 0.0, 19.0, -10.0, Some(0.0), Some(19.0), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Grounded Down Special Stab
#[acmd_script( agent = "ridley", script = "game_speciallwstab", category = ACMD_GAME )]
unsafe fn ssbuexo_ridley_grounded_down_special_stab_acmd(fighter : &mut L2CAgentBase) {
    let special_zoom_gfx = WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RIDLEY_INSTANCE_WORK_ID_INT_DISABLE_SPECIAL_LW_FINISH_COUNT) == 0 {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 45.0, 25, 10, 0, 70, 2.5, 0.0, 8.0, 28.5, Some(0.0), Some(7.0), Some(35.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_BODY_HEAD, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
            AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                WorkModule::inc_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
                if special_zoom_gfx < 2 {
                    SlowModule::set_whole(fighter.module_accessor, 8, 80);
                    macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 2.0,/*no*/ 0.0,/*zoom*/ 1.8,/*yrot*/ 0.0,/*xrot*/ 0.0);
                    EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), Hash40::new("top"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
                    macros::PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
                    macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_XL);
                }
                if special_zoom_gfx >= 4 {
                    SlowModule::clear_whole(fighter.module_accessor);
                    CameraModule::reset_all(fighter.module_accessor);
                    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
                    macros::CAM_ZOOM_OUT(fighter);
                }
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RIDLEY_INSTANCE_WORK_ID_FLAG_DISABLE_CRITICAL_SPECIAL_LW) == false {
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 45.0, 25, 10, 0, 70, 2.5, 0.0, 8.0, 28.5, Some(0.0), Some(7.0), Some(35.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
                if special_zoom_gfx < 2 {
                    SlowModule::set_whole(fighter.module_accessor, 8, 80);
                    macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 2.0,/*no*/ 0.0,/*zoom*/ 1.8,/*yrot*/ 0.0,/*xrot*/ 0.0);
                    EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), Hash40::new("top"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
                    macros::PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
                    macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_XL);
                }
                if special_zoom_gfx >= 4 {
                    SlowModule::clear_whole(fighter.module_accessor);
                    CameraModule::reset_all(fighter.module_accessor);
                    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
                    macros::CAM_ZOOM_OUT(fighter);
                }
            }
        }
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 361, 50, 0, 30, 2.5, 0.0, 8.0, 8.0, Some(0.0), Some(7.0), Some(29.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.2);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Grounded Down Special Release
#[acmd_script( agent = "ridley", script = "game_speciallwfinish", category = ACMD_GAME )]
unsafe fn ssbuexo_ridley_grounded_down_special_release_acmd(fighter : &mut L2CAgentBase) {
    let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE) as u32;
    let capture_boma = sv_battle_object::module_accessor(capture_id);
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        macros::WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
        if DamageModule::damage(capture_boma, 0) > 150.0 {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_DEDEDE_FINAL, 0, 10.0, 57, 100, 150, 0, 2.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_TAIL);
        }
        else {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_DEDEDE_FINAL, 0, 10.0, 57, 100, 150, 0, 2.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_TAIL);
        }
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_WORK_ID_FLAG_FINAL_ABS_SET);
        AttackModule::set_no_finish_camera(fighter.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, true, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_THROW);
        JostleModule::set_status(fighter.module_accessor, true);
        macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
    }
}

//Aerial Down Special ACMD
#[acmd_script( agent = "ridley", script = "game_specialairlwstab", category = ACMD_GAME )]
unsafe fn ssbuexo_ridley_aerial_down_special_acmd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("tail8"), 16.0, 300, 55, 0, 40, 5.0, 0.0, 0.0, 0.0, Some(12.0), Some(0.0), Some(0.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 2, 0, Hash40::new("tail8"), 16.0, 300, 55, 0, 40, 5.0, 0.0, 0.0, 0.0, Some(12.0), Some(0.0), Some(0.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 3, 0, Hash40::new("tail7"), 5.0, 361, 55, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 4, 0, Hash40::new("tail6"), 5.0, 361, 55, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 5, 0, Hash40::new("tail5"), 5.0, 361, 55, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 6, 0, Hash40::new("tail4"), 5.0, 361, 55, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 7, 0, Hash40::new("tail3"), 5.0, 361, 55, 0, 40, 3.0, 0.0, 0.0, 0.0, Some(-12.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("tail8"), 5.0, 361, 55, 0, 40, 5.0, 0.0, 0.0, 0.0, Some(12.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 2, 0, Hash40::new("tail7"), 5.0, 361, 55, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 3, 0, Hash40::new("tail6"), 5.0, 361, 55, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 4, 0, Hash40::new("tail5"), 5.0, 361, 55, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 5, 0, Hash40::new("tail4"), 5.0, 361, 55, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 6, 0, Hash40::new("tail3"), 5.0, 361, 55, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 7, 0, Hash40::new("tail2"), 5.0, 361, 55, 0, 40, 3.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Aerial Down Special Effect
#[acmd_script( agent = "ridley", script = "effect_specialairlwstab", category = ACMD_EFFECT )]
unsafe fn ssbuexo_ridley_aerial_down_special_effect(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("tail8"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("ridley_death_stab_flare"), Hash40::new("tail8"), 0, 0, 0, 0, 180, 0, 1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("ridley_death_stab_line"), Hash40::new("top"), 0, -7, -8, 60, 0, 0, 0.9, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("tail8"), 10, -0.0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("ridley_death_stab_flare"), false, true);
    }
}

//Aerial Down Special Sound
#[acmd_script( agent = "ridley", script = "sound_specialairlwstab", category = ACMD_SOUND )]
unsafe fn ssbuexo_ridley_aerial_down_special_sound(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ridley_special_l01"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ridley_special_l02"));
    }
}

//Aerial Down Special Expression
#[acmd_script( agent = "ridley", script = "expression_specialairlwstab", category = ACMD_EXPRESSION )]
unsafe fn ssbuexo_ridley_aerial_down_special_expression(fighter : &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_piercel"), 0);
    }
}

//Aerial Down Special Landing ACMD
#[acmd_script( agent = "ridley", script = "game_speciallwpogolanding", category = ACMD_GAME )]
unsafe fn ssbuexo_ridley_aerial_down_special_landing_acmd(fighter : &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 10.0);
    MotionModule::set_rate(fighter.module_accessor, 0.5);
    frame(fighter.lua_state_agent, 30.0);
    MotionModule::set_rate(fighter.module_accessor, 1.0);
}

//Aerial Down Special Landing Effect
#[acmd_script( agent = "ridley", script = "effect_speciallwpogolanding", category = ACMD_EFFECT )]
unsafe fn ssbuexo_ridley_aerial_down_special_landing_effect(fighter : &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("ridley_death_stab_flare"), false, true);
        EffectModule::set_visible_kind(fighter.module_accessor, Hash40::new("ridley_death_stab_line"), false);
        EffectModule::set_visible_kind(fighter.module_accessor, Hash40::new("sys_sp_flash"), false);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), -18, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Down Special Landing Sound
#[acmd_script( agent = "ridley", script = "sound_speciallwpogolanding", category = ACMD_SOUND )]
unsafe fn ssbuexo_ridley_aerial_down_special_landing_sound(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ridley_landing03"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ridley_attackair_f01"));
    }
}

//Aerial Down Special Landing Expression
#[acmd_script( agent = "ridley", script = "expression_speciallwpogolanding", category = ACMD_EXPRESSION )]
unsafe fn ssbuexo_ridley_aerial_down_special_landing_expression(fighter : &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_landl_hv"), 0, false, 0);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_ridley_up_special_wall_bounce_forward_acmd,
        ssbuexo_ridley_up_special_wall_bounce_back_acmd,
        ssbuexo_ridley_up_special_wall_bounce_up_acmd,
        ssbuexo_ridley_grounded_down_special_stab_acmd,
        ssbuexo_ridley_grounded_down_special_release_acmd,
        ssbuexo_ridley_aerial_down_special_acmd,
        ssbuexo_ridley_aerial_down_special_effect,
        ssbuexo_ridley_aerial_down_special_sound,
        ssbuexo_ridley_aerial_down_special_expression,
        ssbuexo_ridley_aerial_down_special_landing_acmd,
        ssbuexo_ridley_aerial_down_special_landing_effect,
        ssbuexo_ridley_aerial_down_special_landing_sound,
        ssbuexo_ridley_aerial_down_special_landing_expression
    );
}