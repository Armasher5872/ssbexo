use super::*;

//Win 2 Effect
unsafe extern "C" fn ssbexo_sonic_win_2_effect(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_SONIC {
        if is_excute(agent) {
            EFFECT_FOLLOW_arg11(agent, Hash40::new("sonic_runtrace"), Hash40::new("throw"), 0, 0, 0, 180, 180, 0, -1, true, WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR));
            LAST_EFFECT_SET_RATE(agent, 1.09);
        }
        else {
            if is_excute(agent) {
                EFFECT_FOLLOW_arg11(agent, Hash40::new("sonic_runtrace"), Hash40::new("throw"), 0, 0, 0, 0, 180, 0, 1, true, WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR));
                LAST_EFFECT_SET_RATE(agent, 1.09);
            }
        }
    }
    for _ in 0..11 {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("throw"), 5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_RATE(agent, 1.5);
        }
        wait(agent.lua_state_agent, 2.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("throw"), 5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_RATE(agent, 1.5);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    frame(agent.lua_state_agent, 43.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.675, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.78);
        EFFECT(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 12.5, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), -9.869, 19.497, -1.426, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 9.665, 7.789, -1.426, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(agent.lua_state_agent, 46.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.675, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.78);
        EFFECT(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 12.5, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.675, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.78);
        EFFECT(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 12.5, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
    }
    frame(agent.lua_state_agent, 52.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), -5.595, 16.493, -0.235, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 2.638, 11.604, -0.071, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(agent.lua_state_agent, 55.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.675, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.78);
        EFFECT(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 12.5, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
    }
    frame(agent.lua_state_agent, 61.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.675, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.78);
        EFFECT(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 12.5, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), -5.424, 11.947, -0.091, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 7.699, 17.179, -0.058, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(agent.lua_state_agent, 70.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.675, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.78);
        EFFECT(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 12.5, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.55);
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), -9.841, 13.105, -0.058, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 9.715, 11.046, -0.782, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(agent.lua_state_agent, 79.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), -9.869, 7.789, -1.426, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 9.665, 19.497, -1.426, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(agent.lua_state_agent, 88.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.675, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.78);
    }
}

//Win 2 Wait Effect
unsafe extern "C" fn ssbexo_sonic_win_2_wait_effect(_agent: &mut L2CAgentBase) {}

//Final Smash Start ACMD
unsafe extern "C" fn ssbexo_sonic_final_start_acmd(agent: &mut L2CAgentBase) {
    let scale = PostureModule::scale(agent.module_accessor);
    let lr = PostureModule::lr(agent.module_accessor);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_CHARGE) {
        frame(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            CHECK_VALID_FINAL_START_CAMERA(agent, 0, 7, 20, 0, 0, 0);
            FT_SET_FINAL_FEAR_FACE(agent, 60);
            camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, -2.0*lr, 8);
            FT_START_CUTIN(agent);
        }
        frame(agent.lua_state_agent, 16.0);
        if is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, *EFFECT_SUB_ATTRIBUTE_EMIT, false);
            sv_animcmd::EFFECT_OFF(agent.lua_state_agent);
            ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_SONIC_GENERATE_ARTICLE_SUPERSONIC, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::change_status(agent.module_accessor, *FIGHTER_SONIC_GENERATE_ARTICLE_SUPERSONIC, *WEAPON_SONIC_SUPERSONIC_STATUS_KIND_START, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            VisibilityModule::set_whole(agent.module_accessor, false);
            SA_SET(agent, *SITUATION_KIND_AIR);
            CORRECT(agent, *GROUND_CORRECT_KIND_NONE);
        }
        frame(agent.lua_state_agent, 17.0);
        if is_excute(agent) {
            CAM_ZOOM_OUT(agent);
            CAM_ZOOM_IN_arg5(agent, 0.3, 2.0, 40.0*scale, 0.0, 0.0);
        }
        frame(agent.lua_state_agent, 20.0);
        if is_excute(agent) {
            SlowModule::clear_whole(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 33.0);
        if is_excute(agent) {
            CAM_ZOOM_OUT(agent);
            CAM_ZOOM_IN_arg5(agent, 2.0, 0.0, 1.0*scale, 0.0, 0.0);
        }
        frame(agent.lua_state_agent, 35.0);
        if is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_SONIC_STATUS_FINAL_FLAG_FINAL_CAMERA);
        }
    }
    else {
        frame(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            CHECK_VALID_FINAL_START_CAMERA(agent, 0, 7, 20, 0, 0, 0);
            SLOW_OPPONENT(agent, 4.0, 60.0);
            FT_SET_FINAL_FEAR_FACE(agent, 60);
            camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, -2.0*lr, 8);
            FT_START_CUTIN(agent);
        }
        frame(agent.lua_state_agent, 15.0);
        if is_excute(agent) {
            SlowModule::set_whole(agent.module_accessor, 4, 0);
        }
        frame(agent.lua_state_agent, 16.0);
        if is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, *EFFECT_SUB_ATTRIBUTE_EMIT, false);
            sv_animcmd::EFFECT_OFF(agent.lua_state_agent);
            ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_SONIC_GENERATE_ARTICLE_SUPERSONIC, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::change_status(agent.module_accessor, *FIGHTER_SONIC_GENERATE_ARTICLE_SUPERSONIC, *WEAPON_SONIC_SUPERSONIC_STATUS_KIND_START, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            VisibilityModule::set_whole(agent.module_accessor, false);
            SA_SET(agent, *SITUATION_KIND_AIR);
            CORRECT(agent, *GROUND_CORRECT_KIND_NONE);
        }
        frame(agent.lua_state_agent, 20.0);
        if is_excute(agent) {
            SlowModule::clear_whole(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 33.0);
        if is_excute(agent) {
            CAM_ZOOM_OUT(agent);
            CAM_ZOOM_IN_arg5(agent, 2.0, 0.0, 1.0*scale, 0.0, 0.0);
        }
        frame(agent.lua_state_agent, 35.0);
        if is_excute(agent) {
            CAM_ZOOM_OUT(agent);
        }
    }
}

//Super Sonic Final Start ACMD
unsafe extern "C" fn ssbexo_supersonic_final_start_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 80, 100, 0, 90, 22.0, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
        AttackModule::set_final_finish_cut_in(agent.module_accessor, 0, true, true, -1.0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Super Sonic Final Start Ball ACMD
unsafe extern "C" fn ssbexo_supersonic_final_start_ball_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 361, 65, 0, 90, 22.0, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
        AttackModule::set_final_finish_cut_in(agent.module_accessor, 0, true, true, -1.0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Super Sonic Final Start Ball Effect
unsafe extern "C" fn ssbexo_supersonic_final_start_ball_effect(agent: &mut L2CAgentBase) {
    if PostureModule::lr(agent.module_accessor) == 1.0 {
        EFFECT_FOLLOW(agent, Hash40::new("sonic_final_chaos_start"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    else {
        EFFECT_FOLLOW(agent, Hash40::new("sonic_final_chaos_start_l"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1, true);
    }
}

//Super Sonic Final Idle ACMD
unsafe extern "C" fn ssbexo_supersonic_final_idle_acmd(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
            ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 361, 55, 0, 90, 15.0, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
            AttackModule::set_final_finish_cut_in(agent.module_accessor, 0, true, true, -1.0, false);
        }
        wait(agent.lua_state_agent, 6.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 44.0);
    }
}

//Super Sonic Final Idle Effect
unsafe extern "C" fn ssbexo_supersonic_final_idle_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sonic_final_flash"), Hash40::new("hip"), 0, -1.5, 0, 0, 0, 0, 1, false);
        EffectModule::req_screen(agent.module_accessor, Hash40::new("bg_sonic_final"), false, false, false);
    }
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sonic_final_flash"), Hash40::new("hip"), 0, -1.5, 0, 0, 0, 0, 1, false);
        }
        wait(agent.lua_state_agent, 50.0);
    }
}

//Super Sonic Final Turn ACMD
unsafe extern "C" fn ssbexo_supersonic_final_turn_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        REVERSE_LR(agent);
        WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Super Sonic Final Move Start ACMD
unsafe extern "C" fn ssbexo_supersonic_final_move_start_acmd(agent: &mut L2CAgentBase) {
    let scale = PostureModule::scale(agent.module_accessor);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_CHARGE) {
        if is_excute(agent) {
            WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_38_rush_sp"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        if scale <= 1.4 {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 70, 93, 0, 80, 9.0, 0.0, 11.0, 0.0, Some(0.0), Some(11.0), Some(-60.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 50, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
                AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
                AttackModule::set_final_finish_cut_in(agent.module_accessor, 0, true, true, -1.0, false);
            }
        }
        else if scale <= 0.5 {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 70, 93, 0, 80, 12.0, 0.0, 11.0, 0.0, Some(0.0), Some(11.0), Some(-170.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 50, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
                AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
                AttackModule::set_final_finish_cut_in(agent.module_accessor, 0, true, true, -1.0, false);
            }
        }
        else {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 70, 93, 0, 80, 9.0, 0.0, 11.0, 0.0, Some(0.0), Some(11.0), Some(-80.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 50, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
                AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
                AttackModule::set_final_finish_cut_in(agent.module_accessor, 0, true, true, -1.0, false);
            }
        }
    }
    else {
        if is_excute(agent) {
            WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        }
    }
}

//Super Sonic Final Move Start Effect
unsafe extern "C" fn ssbexo_supersonic_final_move_start_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sonic_final_attack"), Hash40::new("hip"), 0, -1.5, 0, 0, 0, 0, 1, true);
    }
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sonic_final_flash"), Hash40::new("hip"), 0, -1.5, 0, 0, 0, 0, 1, false);
        }
        wait(agent.lua_state_agent, 50.0);
    }
}

//Super Sonic Final Move ACMD
unsafe extern "C" fn ssbexo_supersonic_final_move_acmd(agent: &mut L2CAgentBase) {
    let scale = PostureModule::scale(agent.module_accessor);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_CHARGE) {
        if is_excute(agent) {
            WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_38_rush_sp"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        if scale <= 1.4 {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 70, 93, 0, 80, 9.0, 0.0, 11.0, 0.0, Some(0.0), Some(11.0), Some(-60.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 50, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
                AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
                AttackModule::set_final_finish_cut_in(agent.module_accessor, 0, true, true, -1.0, false);
            }
        }
        else if scale <= 0.5 {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 70, 93, 0, 80, 12.0, 0.0, 11.0, 0.0, Some(0.0), Some(11.0), Some(-170.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 50, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
                AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
                AttackModule::set_final_finish_cut_in(agent.module_accessor, 0, true, true, -1.0, false);
            }
        }
        else {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 70, 93, 0, 80, 9.0, 0.0, 11.0, 0.0, Some(0.0), Some(11.0), Some(-80.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 50, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
                AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
                AttackModule::set_final_finish_cut_in(agent.module_accessor, 0, true, true, -1.0, false);
            }
        }
    }
    else {
        if is_excute(agent) {
            WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
            ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 361, 80, 0, 90, 15.0, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
            AttackModule::set_final_finish_cut_in(agent.module_accessor, 0, true, true, -1.0, false);
        }
    }
}

//Super Sonic Final Move Effect
unsafe extern "C" fn ssbexo_supersonic_final_move_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sonic_final_attack"), Hash40::new("hip"), 0, -1.5, 0, 0, 0, 0, 1, true);
        EffectModule::req_screen(agent.module_accessor, Hash40::new("bg_sonic_final"), false, false, false);
    }
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sonic_final_flash"), Hash40::new("hip"), 0, -1.5, 0, 0, 0, 0, 1, false);
        }
        wait(agent.lua_state_agent, 50.0);
    }
}

pub fn install() {
    Agent::new("sonic")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .effect_acmd("effect_win2", ssbexo_sonic_win_2_effect, Low)
    .effect_acmd("effect_win2wait", ssbexo_sonic_win_2_wait_effect, Low)
    .game_acmd("game_finalstart", ssbexo_sonic_final_start_acmd, Low)
    .game_acmd("game_finalairstart", ssbexo_sonic_final_start_acmd, Low)
    .install()
    ;
    Agent::new("sonic_supersonic")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_finalstart", ssbexo_supersonic_final_start_acmd, Low)
    .game_acmd("game_finalairstart", ssbexo_supersonic_final_start_acmd, Low)
    .game_acmd("game_finalstartball", ssbexo_supersonic_final_start_ball_acmd, Low)
    .game_acmd("game_finalairstartball", ssbexo_supersonic_final_start_ball_acmd, Low)
    .effect_acmd("effect_finalballstart", ssbexo_supersonic_final_start_ball_effect, Low)
    .effect_acmd("effect_finalairballstart", ssbexo_supersonic_final_start_ball_effect, Low)
    .game_acmd("game_finalidle", ssbexo_supersonic_final_idle_acmd, Low)
    .effect_acmd("effect_finalidle", ssbexo_supersonic_final_idle_effect, Low)
    .game_acmd("game_finalturn", ssbexo_supersonic_final_turn_acmd, Low)
    .game_acmd("game_finalmovestart", ssbexo_supersonic_final_move_start_acmd, Low)
    .effect_acmd("effect_finalmovestart", ssbexo_supersonic_final_move_start_effect, Low)
    .game_acmd("game_finalmove", ssbexo_supersonic_final_move_acmd, Low)
    .effect_acmd("effect_finalmove", ssbexo_supersonic_final_move_effect, Low)
    .install()
    ;
}