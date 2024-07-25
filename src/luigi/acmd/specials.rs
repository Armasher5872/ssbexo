use super::*;

//Side Special ACMD
unsafe extern "C" fn ssbexo_luigi_side_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::SEARCH(agent, 0, 0, Hash40::new("top"), 8.0, 0.0, 6.5, 2.5, Some(0.0), Some(6.5), Some(8.0), *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 8.0, 0.0, 6.5, 2.5, 0.0, 6.5, -6.0, 0.0, 0.0, 0, false, 1.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::SA_SET(agent, *SITUATION_KIND_AIR);
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_S_RAM);
        macros::CORRECT(agent, *GROUND_CORRECT_KIND_AIR);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 10.0, 361, 80, 0, 30, 4.8, 1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
        AttackModule::set_attack_keep_rumble(agent.module_accessor, 0, true);
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_GROUND_CHECK);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 10.0, 361, 80, 0, 30, 4.8, 1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::enable_transition_term(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_BRAKE);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
}

//Side Special Misfire ACMD
unsafe extern "C" fn ssbexo_luigi_side_special_misfire_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::SEARCH(agent, 0, 0, Hash40::new("top"), 8.0, 0.0, 6.5, 2.5, Some(0.0), Some(6.5), Some(8.0), *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 8.0, 0.0, 6.5, 2.5, 0.0, 6.5, -6.0, 0.0, 0.0, 0, false, 1.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::SA_SET(agent, *SITUATION_KIND_AIR);
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_S_RAM);
        macros::CORRECT(agent, *GROUND_CORRECT_KIND_AIR);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 25.0, 361, 100, 0, 20, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
        AttackModule::set_attack_keep_rumble(agent.module_accessor, 0, true);
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_GROUND_CHECK);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 25.0, 361, 100, 0, 20, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::enable_transition_term(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_BRAKE);
    }
}

//Grounded Up Special ACMD
unsafe extern "C" fn ssbexo_luigi_grounded_up_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 88, 88, 0, 50, 2.2, 1.2, 6.0, 7.0, Some(-1.2), Some(6.0), Some(7.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_PUNCH);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("head"), 1.0, 80, 1, 0, 1, 5.8, 2.0, 2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 1.0, 80, 1, 0, 1, 4.7, 0.0, 4.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Aerial Up Special ACMD
unsafe extern "C" fn ssbexo_luigi_aerial_up_special_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHECK_DEAD_AREA_FORCE);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHECK_DEAD_AREA_FORCE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 90, 80, 0, 40, 2.7, 0.0, 6.0, 5.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_PUNCH);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("head"), 1.0, 80, 1, 0, 1, 5.8, 2.0, 2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 1.0, 80, 1, 0, 1, 4.7, 0.0, 4.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Special ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_acmd(agent: &mut L2CAgentBase) {
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU) {
        if macros::is_excute(agent) {
            ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, false, -1);
    }
}

//Down Special Effect
unsafe extern "C" fn ssbexo_luigi_down_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Down Special Sound
unsafe extern "C" fn ssbexo_luigi_down_special_sound(_agent: &mut L2CAgentBase) {}

//Down Special Expression
unsafe extern "C" fn ssbexo_luigi_down_special_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Down Special Loop ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_loop_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 0.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.2, 180, 100, 30, 0, 6.0, 0.0, 6.0, 8.0, Some(0.0), Some(6.0), Some(27.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 5.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
}

//Down Special Loop Effect
unsafe extern "C" fn ssbexo_luigi_down_special_loop_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("luigi_final_vacuum"), Hash40::new("top"), 0, 6, 14, 0, 0, 0, 1, true);
    }
}

//Down Special Loop Sound
unsafe extern "C" fn ssbexo_luigi_down_special_loop_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_luigi_final02"));
    }
}

//Down Special Loop Expression
unsafe extern "C" fn ssbexo_luigi_down_special_loop_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_none"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Down Special Catch ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_catch_acmd(_agent: &mut L2CAgentBase) {}

//Down Special Catch Effect
unsafe extern "C" fn ssbexo_luigi_down_special_catch_effect(_agent: &mut L2CAgentBase) {}

//Down Special Catch Sound
unsafe extern "C" fn ssbexo_luigi_down_special_catch_sound(_agent: &mut L2CAgentBase) {}

//Down Special Catch Pull Expression
unsafe extern "C" fn ssbexo_luigi_down_special_catch_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

//Down Special End ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_end_acmd(agent: &mut L2CAgentBase) {
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER) {
        if macros::is_excute(agent) {
            ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    if macros::is_excute(agent) {
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Special End Effect
unsafe extern "C" fn ssbexo_luigi_down_special_end_effect(_agent: &mut L2CAgentBase) {}

//Down Special End Sound
unsafe extern "C" fn ssbexo_luigi_down_special_end_sound(_agent: &mut L2CAgentBase) {}

//Down Special End Expression
unsafe extern "C" fn ssbexo_luigi_down_special_end_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Down Special Forward Throw ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_throw_f_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 45, 65, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::CHECK_FINISH_CAMERA(agent, 18, 4);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.0);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 12.0, y: -1.0, z: 0.0});
        macros::FT_CATCH_STOP(agent, 5, 1);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

//Down Special Forward Throw Effect
unsafe extern "C" fn ssbexo_luigi_down_special_throw_f_effect(agent: &mut L2CAgentBase) {
    let discharge_chance = WorkModule::get_int(agent.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_S_DISCHARGE_CHANCE);
    let lr = PostureModule::lr(agent.module_accessor);
    let offset = WorkModule::get_param_float(agent.module_accessor, hash40("height"), 0);
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, -3, 3, 0, 20, -90, 1.7, true, *EF_FLIP_YZ, 0.4);
        EffectModule::set_disable_render_offset_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 17, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 17, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_spin_wind"), false, false);
    }
    if discharge_chance > 1 {
        if macros::is_excute(agent) {
            if WorkModule::get_param_int(agent.module_accessor, hash40("param_motion"), hash40("flip")) != 0 {
                macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_flash"), Hash40::new("sys_flash"), Hash40::new("top"), -5, offset, 2, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
            }
            else {
                macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), -5.0 * lr, offset, 2, 0, 0, 0, 1.0, true);
            }
            if discharge_chance == 9 {
                macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 1.0); //White
            }
            else if discharge_chance == 7 {
                macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 0.0); //Red
            }
            else if discharge_chance == 5 {
                macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.647, 0.0); //Orange
            }
            else {
                macros::LAST_EFFECT_SET_COLOR(agent, 0.831, 0.686, 0.216); //Yellow
            }
        }
    }
    else {
        if macros::is_excute(agent) {
            smash::app::FighterUtil::flash_eye_info(agent.module_accessor);
            if WorkModule::get_param_int(agent.module_accessor, hash40("param_motion"), hash40("flip")) != 0 {
                macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_flash"), Hash40::new("sys_flash"), Hash40::new("top"), -5, offset, 2, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
            }
            else {
                macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), -5.0 * lr, offset, 2, 0, 0, 0, 1.0, true);
            }
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0); //Green
        }
    }
}

//Down Special Forward Throw Sound
unsafe extern "C" fn ssbexo_luigi_down_special_throw_f_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    wait(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_02"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_luigi_rnd_attack"));
    }
}

//Down Special Forward Throw Expression
unsafe extern "C" fn ssbexo_luigi_down_special_throw_f_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Down Special Back Throw ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_throw_b_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 45, 65, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        macros::CHECK_FINISH_CAMERA(agent, 21, 3);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.0);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 10.0, y: -1.0, z: 0.0});
        macros::FT_CATCH_STOP(agent, 5, 1);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::REVERSE_LR(agent);
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

//Down Special Back Throw Effect
unsafe extern "C" fn ssbexo_luigi_down_special_throw_b_effect(agent: &mut L2CAgentBase) {
    let discharge_chance = WorkModule::get_int(agent.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_S_DISCHARGE_CHANCE);
    let lr = PostureModule::lr(agent.module_accessor);
    let offset = WorkModule::get_param_float(agent.module_accessor, hash40("height"), 0);
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, -3, 0, 0, 230, 90, 1.7, true, *EF_FLIP_YZ, 0.4);
        EffectModule::set_disable_render_offset_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), -16, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -16, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_spin_wind"), false, false);
    }
    if discharge_chance > 1 {
        if macros::is_excute(agent) {
            if WorkModule::get_param_int(agent.module_accessor, hash40("param_motion"), hash40("flip")) != 0 {
                macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_flash"), Hash40::new("sys_flash"), Hash40::new("top"), -5, offset, 2, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
            }
            else {
                macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), -5.0 * lr, offset, 2, 0, 0, 0, 1.0, true);
            }
            if discharge_chance == 9 {
                macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 1.0); //White
            }
            else if discharge_chance == 7 {
                macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 0.0); //Red
            }
            else if discharge_chance == 5 {
                macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.647, 0.0); //Orange
            }
            else {
                macros::LAST_EFFECT_SET_COLOR(agent, 0.831, 0.686, 0.216); //Yellow
            }
        }
    }
    else {
        if macros::is_excute(agent) {
            smash::app::FighterUtil::flash_eye_info(agent.module_accessor);
            if WorkModule::get_param_int(agent.module_accessor, hash40("param_motion"), hash40("flip")) != 0 {
                macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_flash"), Hash40::new("sys_flash"), Hash40::new("top"), -5, offset, 2, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
            }
            else {
                macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), -5.0 * lr, offset, 2, 0, 0, 0, 1.0, true);
            }
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0); //Green
        }
    }
}

//Down Special Back Throw Sound
unsafe extern "C" fn ssbexo_luigi_down_special_throw_b_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    wait(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_02"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_luigi_rnd_attack"));
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_DOWN_SE(agent, Hash40::new("se_common_down_soil_s"));
        macros::PLAY_SE(agent, Hash40::new("se_common_kick_hit_m"));
    }
}

//Down Special Back Throw Expression
unsafe extern "C" fn ssbexo_luigi_down_special_throw_b_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Down Special Up Throw ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_throw_hi_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 90, 72, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("top"), 6.0, 88, 55, 0, 95, 8.5, 0.0, 8.0, 10.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::CHECK_FINISH_CAMERA(agent, -2, 24);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 7.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

//Down Special Up Throw Effect
unsafe extern "C" fn ssbexo_luigi_down_special_throw_hi_effect(agent: &mut L2CAgentBase) {
    let discharge_chance = WorkModule::get_int(agent.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_S_DISCHARGE_CHANCE);
    let lr = PostureModule::lr(agent.module_accessor);
    let offset = WorkModule::get_param_float(agent.module_accessor, hash40("height"), 0);
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    if discharge_chance > 1 {
        if macros::is_excute(agent) {
            if WorkModule::get_param_int(agent.module_accessor, hash40("param_motion"), hash40("flip")) != 0 {
                macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_flash"), Hash40::new("sys_flash"), Hash40::new("top"), -5, offset, 2, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
            }
            else {
                macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), -5.0 * lr, offset, 2, 0, 0, 0, 1.0, true);
            }
            if discharge_chance == 9 {
                macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 1.0); //White
            }
            else if discharge_chance == 7 {
                macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 0.0); //Red
            }
            else if discharge_chance == 5 {
                macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.647, 0.0); //Orange
            }
            else {
                macros::LAST_EFFECT_SET_COLOR(agent, 0.831, 0.686, 0.216); //Yellow
            }
        }
    }
    else {
        if macros::is_excute(agent) {
            smash::app::FighterUtil::flash_eye_info(agent.module_accessor);
            if WorkModule::get_param_int(agent.module_accessor, hash40("param_motion"), hash40("flip")) != 0 {
                macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_flash"), Hash40::new("sys_flash"), Hash40::new("top"), -5, offset, 2, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
            }
            else {
                macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), -5.0 * lr, offset, 2, 0, 0, 0, 1.0, true);
            }
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0); //Green
        }
    }
}

//Down Special Up Throw Sound
unsafe extern "C" fn ssbexo_luigi_down_special_throw_hi_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    wait(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_02"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_luigi_rnd_attack"));
    }
}

//Down Special Up Throw Expression
unsafe extern "C" fn ssbexo_luigi_down_special_throw_hi_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
}

pub fn install() {
    Agent::new("luigi")
    .game_acmd("game_specials", ssbexo_luigi_side_special_acmd, Priority::Low)
    .game_acmd("game_specialsdischarge", ssbexo_luigi_side_special_misfire_acmd, Priority::Low)
    .game_acmd("game_specialhi", ssbexo_luigi_grounded_up_special_acmd, Priority::Low)
    .game_acmd("game_specialairhi", ssbexo_luigi_aerial_up_special_acmd, Priority::Low)
    .game_acmd("game_speciallw", ssbexo_luigi_down_special_acmd, Priority::Low)
    .game_acmd("game_specialairlw", ssbexo_luigi_down_special_acmd, Priority::Low)
    .effect_acmd("effect_speciallw", ssbexo_luigi_down_special_effect, Priority::Low)
    .effect_acmd("effect_specialairlw", ssbexo_luigi_down_special_effect, Priority::Low)
    .sound_acmd("sound_speciallw", ssbexo_luigi_down_special_sound, Priority::Low)
    .sound_acmd("sound_specialairlw", ssbexo_luigi_down_special_sound, Priority::Low)
    .expression_acmd("expression_speciallw", ssbexo_luigi_down_special_expression, Priority::Low)
    .expression_acmd("expression_specialairlw", ssbexo_luigi_down_special_expression, Priority::Low)
    .game_acmd("game_speciallwloop", ssbexo_luigi_down_special_loop_acmd, Priority::Low)
    .effect_acmd("effect_speciallwloop", ssbexo_luigi_down_special_loop_effect, Priority::Low)
    .sound_acmd("sound_speciallwloop", ssbexo_luigi_down_special_loop_sound, Priority::Low)
    .expression_acmd("expression_speciallwloop", ssbexo_luigi_down_special_loop_expression, Priority::Low)
    .game_acmd("game_speciallwcatch", ssbexo_luigi_down_special_catch_acmd, Priority::Low)
    .effect_acmd("effect_speciallwcatch", ssbexo_luigi_down_special_catch_effect, Priority::Low)
    .sound_acmd("sound_speciallwcatch", ssbexo_luigi_down_special_catch_sound, Priority::Low)
    .expression_acmd("expression_speciallwcatch", ssbexo_luigi_down_special_catch_expression, Priority::Low)
    .game_acmd("game_speciallwend", ssbexo_luigi_down_special_end_acmd, Priority::Low)
    .game_acmd("game_specialairlwend", ssbexo_luigi_down_special_end_acmd, Priority::Low)
    .effect_acmd("effect_speciallwend", ssbexo_luigi_down_special_end_effect, Priority::Low)
    .effect_acmd("effect_specialairlwend", ssbexo_luigi_down_special_end_effect, Priority::Low)
    .sound_acmd("sound_speciallwend", ssbexo_luigi_down_special_end_sound, Priority::Low)
    .sound_acmd("sound_specialairlwend", ssbexo_luigi_down_special_end_sound, Priority::Low)
    .expression_acmd("expression_speciallwend", ssbexo_luigi_down_special_end_expression, Priority::Low)
    .expression_acmd("expression_specialairlwend", ssbexo_luigi_down_special_end_expression, Priority::Low)
    .game_acmd("game_speciallwthrowf", ssbexo_luigi_down_special_throw_f_acmd, Priority::Low)
    .game_acmd("game_specialairlwthrowf", ssbexo_luigi_down_special_throw_f_acmd, Priority::Low)
    .effect_acmd("effect_speciallwthrowf", ssbexo_luigi_down_special_throw_f_effect, Priority::Low)
    .effect_acmd("effect_specialairlwthrowf", ssbexo_luigi_down_special_throw_f_effect, Priority::Low)
    .sound_acmd("sound_speciallwthrowf", ssbexo_luigi_down_special_throw_f_sound, Priority::Low)
    .sound_acmd("sound_specialairlwthrowf", ssbexo_luigi_down_special_throw_f_sound, Priority::Low)
    .expression_acmd("expression_speciallwthrowf", ssbexo_luigi_down_special_throw_f_expression, Priority::Low)
    .expression_acmd("expression_specialairlwthrowf", ssbexo_luigi_down_special_throw_f_expression, Priority::Low)
    .game_acmd("game_speciallwthrowb", ssbexo_luigi_down_special_throw_b_acmd, Priority::Low)
    .game_acmd("game_specialairlwthrowb", ssbexo_luigi_down_special_throw_b_acmd, Priority::Low)
    .effect_acmd("effect_speciallwthrowb", ssbexo_luigi_down_special_throw_b_effect, Priority::Low)
    .effect_acmd("effect_specialairlwthrowb", ssbexo_luigi_down_special_throw_b_effect, Priority::Low)
    .sound_acmd("sound_speciallwthrowb", ssbexo_luigi_down_special_throw_b_sound, Priority::Low)
    .sound_acmd("sound_specialairlwthrowb", ssbexo_luigi_down_special_throw_b_sound, Priority::Low)
    .expression_acmd("expression_speciallwthrowb", ssbexo_luigi_down_special_throw_b_expression, Priority::Low)
    .expression_acmd("expression_specialairlwthrowb", ssbexo_luigi_down_special_throw_b_expression, Priority::Low)
    .game_acmd("game_speciallwthrowhi", ssbexo_luigi_down_special_throw_hi_acmd, Priority::Low)
    .game_acmd("game_specialairlwthrowhi", ssbexo_luigi_down_special_throw_hi_acmd, Priority::Low)
    .effect_acmd("effect_speciallwthrowhi", ssbexo_luigi_down_special_throw_hi_effect, Priority::Low)
    .effect_acmd("effect_specialairlwthrowhi", ssbexo_luigi_down_special_throw_hi_effect, Priority::Low)
    .sound_acmd("sound_speciallwthrowhi", ssbexo_luigi_down_special_throw_hi_sound, Priority::Low)
    .sound_acmd("sound_specialairlwthrowhi", ssbexo_luigi_down_special_throw_hi_sound, Priority::Low)
    .expression_acmd("expression_speciallwthrowhi", ssbexo_luigi_down_special_throw_hi_expression, Priority::Low)
    .expression_acmd("expression_specialairlwthrowhi", ssbexo_luigi_down_special_throw_hi_expression, Priority::Low)
    .install()
    ;
}