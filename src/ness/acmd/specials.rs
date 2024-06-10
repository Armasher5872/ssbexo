use super::*;

//PSI Offense Up ACMD
unsafe extern "C" fn ssbexo_ness_shield_special_acmd(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE);
    }
    frame(agent.lua_state_agent, 107.0);
    if OFFENSE_UP_ACTIVE[entry_id] == true {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 0.5);
            macros::WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        }
    }
    else {
        MotionModule::set_rate(agent.module_accessor, 1.0);
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    frame(agent.lua_state_agent, 110.0);
    if OFFENSE_UP_ACTIVE[entry_id] == true {
        if macros::is_excute(agent) {
            macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
            macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
            macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
            macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
            macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
            macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        }
    }
    else {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(agent.lua_state_agent, 118.0);
    if OFFENSE_UP_ACTIVE[entry_id] == true {
        if macros::is_excute(agent) {
            HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            MotionModule::set_rate(agent.module_accessor, 1.0);
            macros::ATTACK(agent, 0, 0, Hash40::new("waist"), 17.0, 95, 77, 0, 48, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("waist"), 12.0, 85, 77, 0, 48, 11.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);  
        }
    }
    else {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 123.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 160.0);
    if macros::is_excute(agent) {
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
    }
}

//PSI Offense Up Effect
unsafe extern "C" fn ssbexo_ness_shield_special_effect(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let vectora = Vector3f {x: 0.0, y: 5.0, z: 0.0};
    let vectorb = Vector3f {x: 0.0, y: 0.0, z: 0.0};
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 16.0);
    for _ in 0..13 {
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 9, 0, 9, 0, 0, 0, false);
            macros::FLASH(agent, 0.7, 1, 0.2, 0.4);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.8, 1, 1, 0.6);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 4.0);
    }
    if OFFENSE_UP_ACTIVE[entry_id] == true {
        frame(agent.lua_state_agent, 107.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("ness_pkfl_hold"), Hash40::new("waist"), 0, 0, 0.0, 0.0, -90, 0, 0.75, 0, 0, 0, 0, 0, 360, true);
            macros::EFFECT(agent, Hash40::new("ness_psi_atk"), Hash40::new("waist"), 0, 0, 0.0, 0.0, -90, 0, 0.75, 0, 0, 0, 0, 0, 360, true);
            macros::FLASH(agent, 1, 0, 0, 0.4);
        }
        frame(agent.lua_state_agent, 110.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("ness_pkfl_hold"), Hash40::new("waist"), 0, 0, 0.0, 0.0, -90, 0, 1.5, 0, 0, 0, 0, 0, 360, true);
            macros::EFFECT(agent, Hash40::new("ness_psi_atk"), Hash40::new("waist"), 0, 0, 0.0, 0.0, -90, 0, 1.5, 0, 0, 0, 0, 0, 360, true);
            macros::FLASH(agent, 2, 0, 0, 0.8);
        }
        frame(agent.lua_state_agent, 118.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("ness_pkfl_bomb_max"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(agent, Hash40::new("ness_psi_atk"), Hash40::new("waist"), 0, 0, 0.0, 0.0, -90, 0, 2.0, 0, 0, 0, 0, 0, 360, true);
            macros::COL_NORMAL(agent);
            smash::app::FighterUtil::flash_eye_info(agent.module_accessor);
		    EffectModule::req_follow(agent.module_accessor, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("head"), &vectora, &vectorb, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
        }
    }
    else {
        frame(agent.lua_state_agent, 108.0);
        for _ in 0..3 {
            if macros::is_excute(agent) {
                macros::FLASH(agent, 0.7, 1, 0.2, 0.4);
            }
            wait(agent.lua_state_agent, 1.0);
            if macros::is_excute(agent) {
                macros::FLASH(agent, 2, 1, 1, 0.6);
            }
            wait(agent.lua_state_agent, 2.0);
            if macros::is_excute(agent) {
                macros::COL_NORMAL(agent);
            }
            wait(agent.lua_state_agent, 1.0);
        }
    }
    frame(agent.lua_state_agent, 120.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("bust"), 1.5, -2, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
}

//PSI Offense Up Sound
unsafe extern "C" fn ssbexo_ness_shield_special_sound(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ness_special_n05"));
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_ness_special_n02"));
    }
    frame(agent.lua_state_agent, 107.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_ness_special_n02"));
    }
    frame(agent.lua_state_agent, 118.0);
    if OFFENSE_UP_ACTIVE[entry_id] == true
    && macros::is_excute(agent) {
        macros::PLAY_SE_REMAIN(agent, Hash40::new("se_ness_special_n04_ll"));
    }
    else {
        macros::PLAY_SE_REMAIN(agent, Hash40::new("se_ness_special_n04_l"));
    }
    frame(agent.lua_state_agent, 140.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ness_landing02"));
    }
}

//PSI Offense Up Expression
unsafe extern "C" fn ssbexo_ness_shield_special_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 107.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 115.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(agent.lua_state_agent, 121.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackl"), 0, false, 0);
    }
    frame(agent.lua_state_agent, 126.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
    }
}

//PK Flash ACMD
unsafe extern "C" fn ssbexo_ness_pk_flash_acmd(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let rand_num = sv_math::rand(hash40("agent"), 100);
    let offense_hash_check = if (60..100).contains(&rand_num) {Hash40::new("collision_attr_death")} else if (25..59).contains(&rand_num) {Hash40::new_raw(0x1C655B0AE7)} else if (14..24).contains(&rand_num) {Hash40::new("collision_attr_paralyze")} else {Hash40::new("collision_attr_curse_poison")};
    let normal_hash_check = if (53..100).contains(&rand_num) {Hash40::new("collision_attr_paralyze")} else if (18..52).contains(&rand_num) {Hash40::new_raw(0x1C655B0AE7)} else if (3..17).contains(&rand_num) {Hash40::new("collision_attr_curse_poison")} else {Hash40::new("collision_attr_death")};
    if PK_FLASH_TIMER[entry_id] >= 120 {
        if OFFENSE_UP_ACTIVE[entry_id] == true {
            if (0..13).contains(&rand_num) {
                if macros::is_excute(agent) {
                    macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 70, 65, 0, 50, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, offense_hash_check, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
                    AttackModule::set_poison_param(agent.module_accessor, 0, 600, 20, 0.2, false);
                }
            }
            else {
                if macros::is_excute(agent) {
                    macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 70, 65, 0, 50, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, offense_hash_check, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);   
                }
            }
        }
        else {
            if (3..17).contains(&rand_num) {
                if macros::is_excute(agent) {
                    macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 70, 65, 0, 50, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, normal_hash_check, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
                    AttackModule::set_poison_param(agent.module_accessor, 0, 600, 20, 0.2, false);
                }
            }
            else {
                if macros::is_excute(agent) {
                    macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 70, 65, 0, 50, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, normal_hash_check, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);   
                }
            }
        }
    }
    else {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 70, 65, 0, 50, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x27936db96d));
    }
}

//PK Fire Shoot ACMD
unsafe extern "C" fn ssbexo_ness_pk_fire_shoot_acmd(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(agent) {
        if OFFENSE_UP_ACTIVE[entry_id] == true {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 150, 80, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            AttackModule::enable_safe_pos(agent.module_accessor);
            AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
        }
        else {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 80, 80, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            AttackModule::enable_safe_pos(agent.module_accessor);
            AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
        }
    }
}

//Grounded PK Fire Pillar ACMD
unsafe extern "C" fn ssbexo_ness_grounded_pk_fire_pillar_acmd(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(agent) {
        if OFFENSE_UP_ACTIVE[entry_id] == true {
            AttackModule::clear_all(agent.module_accessor);
        }
        else {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 68, 30, 0, 10, 6.5, 0.0, 3.1, 2.0, None, None, None, 0.5, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 68, 30, 0, 10, 4.5, 0.0, 9.6, 2.0, None, None, None, 0.5, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
            agent.clear_lua_stack();
            lua_args!(agent, 0, 1, 0.05, 200, 0.6, 0, 10, 20, 60);
            smash::app::sv_animcmd::AREA_WIND_2ND_RAD_arg9(agent.lua_state_agent);
            agent.pop_lua_stack(1);
        }
    }
}

//Aerial PK Fire Pillar ACMD
unsafe extern "C" fn ssbexo_ness_aerial_pk_fire_pillar_acmd(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(agent) {
        if OFFENSE_UP_ACTIVE[entry_id] == true {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.9, 150, 80, 0, 30, 6.5, 0.0, 3.1, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.9, 150, 80, 0, 30, 4.5, 0.0, 9.6, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
            agent.clear_lua_stack();
            lua_args!(agent, 0, 1, 0.05, 200, 0.6, 0, 10, 20, 60);
            smash::app::sv_animcmd::AREA_WIND_2ND_RAD_arg9(agent.lua_state_agent);
            agent.pop_lua_stack(1);
            ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_PK_FIRE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        else {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 58, 45, 0, 16, 6.5, 0.0, 3.1, 2.0, None, None, None, 0.5, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 58, 45, 0, 16, 4.5, 0.0, 9.6, 2.0, None, None, None, 0.5, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);  
            agent.clear_lua_stack();
            lua_args!(agent, 0, 1, 0.05, 200, 0.6, 0, 10, 20, 60);
            smash::app::sv_animcmd::AREA_WIND_2ND_RAD_arg9(agent.lua_state_agent);
            agent.pop_lua_stack(1);
        }
    }
}

//PK Thunderball ACMD
unsafe extern "C" fn ssbexo_ness_pk_thunder_ball_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 362, 50, 0, 60, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 48, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
    }
}

//PK Thunderball Effect
unsafe extern "C" fn ssbexo_ness_pk_thunder_ball_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ness_pkt_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, true);
    }
}

//PK Thunder Tackle ACMD
unsafe extern "C" fn ssbexo_ness_pk_thunder_tackle_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        DamageModule::add_damage(agent.module_accessor, 3.0, 0);
        GroundModule::select_cliff_hangdata(agent.module_accessor, *FIGHTER_NESS_CLIFF_HANG_DATA_SPECIAL_HI as u32);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("rot"), 25.0, 361, 80, 0, 83, 7.0, 0.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PSI);
        JostleModule::set_status(agent.module_accessor, false);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_HI_FLAG_CRITICAL);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("rot"), 17.0, 361, 70, 0, 45, 4.8, 0.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PSI);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_HI_FLAG_ATTACK_FALL_START);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_HI_FLAG_CRITICAL);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_HI_FLAG_LANDING_ENABLE);
        JostleModule::set_status(agent.module_accessor, true);
        GroundModule::select_cliff_hangdata(agent.module_accessor, *FIGHTER_NESS_CLIFF_HANG_DATA_DEFAULT as u32);
    }
}

pub fn install() {
    Agent::new("ness")
    .game_acmd("game_specialshield", ssbexo_ness_shield_special_acmd)
    .effect_acmd("effect_specialshield", ssbexo_ness_shield_special_effect)
    .sound_acmd("sound_specialshield", ssbexo_ness_shield_special_sound)
    .expression_acmd("expression_specialshield", ssbexo_ness_shield_special_expression)
    .game_acmd("game_specialairhi", ssbexo_ness_pk_thunder_tackle_acmd)
    .install()
    ;
    Agent::new("ness_pkflash")
    .game_acmd("game_bang", ssbexo_ness_pk_flash_acmd)
    .install()
    ;
    Agent::new("ness_pkfire")
    .game_acmd("game_shoot", ssbexo_ness_pk_fire_shoot_acmd)
    .game_acmd("game_shootair", ssbexo_ness_pk_fire_shoot_acmd)
    .game_acmd("game_pillar", ssbexo_ness_grounded_pk_fire_pillar_acmd)
    .game_acmd("game_pillarair", ssbexo_ness_aerial_pk_fire_pillar_acmd)
    .install()
    ;
    Agent::new("ness_pkthunder")
    .game_acmd("game_move", ssbexo_ness_pk_thunder_ball_acmd)
    .effect_acmd("effect_move", ssbexo_ness_pk_thunder_ball_effect)
    .install()
    ;
}