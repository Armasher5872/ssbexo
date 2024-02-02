use super::*;

//Shield Special ACMD
unsafe extern "C" fn ssbexo_pichu_shield_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 56.0);
    if macros::is_excute(agent) {
        macros::FT_ADD_DAMAGE(agent, 2.0);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 65, 0, 90, 11.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 59.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Shield Special Effect
unsafe extern "C" fn ssbexo_pichu_shield_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("pichu_elec2"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 0.9, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("pichu_elec2"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.1, true);
    }
    frame(agent.lua_state_agent, 13.0);
    for _ in 0..3 {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 1, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 13.0);
    }
    frame(agent.lua_state_agent, 56.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_elec2"), false, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_kaminari_hit2"), Hash40::new("top"), 0, 0, 0, 0, 90, 0, 0.8, true);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_kaminari_hit"), Hash40::new("top"), 0, 0, 0, 0, 90, 0, 0.9, true);
    }
    for _ in 0..3 {
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0, 0, 0, 0);
            macros::BURN_COLOR(agent, 2, 2, 0.5, 0.9);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH_FRM(agent, 2, 0, 0, 0, 0);
            macros::BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0.7);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH_FRM(agent, 2, 0, 0, 0, 0);
            macros::BURN_COLOR_FRAME(agent, 2, 2, 2, 0.5, 0);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_NORMAL(agent);
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    frame(agent.lua_state_agent, 80.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_kaminari_hit2"), false, true);
        macros::FLASH(agent, 0, 0, 0, 0);
        macros::BURN_COLOR(agent, 2, 2, 0.5, 0.9);
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_cheek"), false, true);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

//Shield Special Sound
unsafe extern "C" fn ssbexo_pichu_shield_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_pichu_appeal_h01"));
        macros::PLAY_SE(agent, Hash40::new("vc_pichu_appeal01"));
    }
    frame(agent.lua_state_agent, 55.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent,Hash40::new("vc_pichu_appeal01"));
        macros::STOP_SE(agent,Hash40::new("se_pichu_appeal_h01"));
        agent.clear_lua_stack();
        lua_args!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
        smash::app::sv_module_access::sound(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
    frame(agent.lua_state_agent, 56.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_special_l03"));
    }
}

//Neutral Special Shoot ACMD
unsafe extern "C" fn ssbexo_pichu_neutral_special_shoot_acmd(agent: &mut L2CAgentBase) {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(agent.lua_state_agent);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if DISCHARGE_ACTIVE[entry_id] == false {
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 100, 35, 0, 80, 5.3, 0.0, 4.7, 11.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 100, 35, 0, 80, 4.3, 0.0, 4.7, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 24.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    if DISCHARGE_ACTIVE[entry_id] == true {
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PICHU_GENERATE_ARTICLE_DENGEKIDAMA, false, 0);
            DamageModule::add_damage(agent.module_accessor, -1.4, 0);
        }
    }
}

//Neutral Special Shoot Effect
unsafe extern "C" fn ssbexo_pichu_neutral_special_shoot_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("pichu_cheek2"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("pichu_cheek_elec"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("pichu_elec_shock"), Hash40::new("top"), 0, 5.5, 13, 0, 0, 0, 0.85, true);
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_cheek2"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_cheek_elec"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_elec_shock"), false, true);
    }
}

//Electroweb ACMD
unsafe extern "C" fn ssbexo_pichu_electroweb_acmd(agent: &mut L2CAgentBase) { 
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 361, 30, 0, 50, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
}

//Grounded Tackle ACMD
unsafe extern "C" fn ssbexo_pichu_grounded_tackle_acmd(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 8.0, 40, 55, 0, 65, 4.0, 5.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_BRAKE_TRIGGER);
    }
    frame(agent.lua_state_agent, 69.0);
    if macros::is_excute(agent) {
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
    }
}

//Aerial Tackle ACMD
unsafe extern "C" fn ssbexo_pichu_aerial_tackle_acmd(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 8.0, 40, 55, 0, 65, 4.0, 5.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_BRAKE_TRIGGER);
    }
    frame(agent.lua_state_agent, 69.0);
    if macros::is_excute(agent) {
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
    }
}

//Tackle Effect
unsafe extern "C" fn ssbexo_pichu_tackle_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("pichu_rocket_bomb"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3, 0.5, 0, 0, -90, 0.6, true, 0.7);
        macros::LAST_EFFECT_SET_RATE(agent, 1.4);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3, 0.5, 0, 90, -90, 0.8, false, 0.5);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3, 0.5, 0, 0, -90, 0.6, true, 0.7);
        smash::app::sv_animcmd::EFFECT_FOLLOW_arg11(agent.lua_state_agent);
        agent.pop_lua_stack(1);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3, 0.5, 0, 45, -90, 0.8, false, 0.5);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
}

//Tackle Sound
unsafe extern "C" fn ssbexo_pichu_tackle_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_pichu_rnd_attack"));
        macros::PLAY_STATUS(agent, Hash40::new("se_pichu_attackair_n01"));
    }
}

//Wild Charge ACMD
unsafe extern "C" fn ssbexo_pichu_wild_charge_acmd(agent: &mut L2CAgentBase) { 
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 15, 90, 0, 90, 3.2, 0.0, 3.3, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
        AttackModule::set_attack_keep_rumble(agent.module_accessor, 0, true);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_BRAKE_TRIGGER);
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 120, 0, 90, 3.2, 0.0, 3.3, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 88.0);
    if macros::is_excute(agent) {
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
    }
}

//Up Special 1 ACMD
unsafe extern "C" fn ssbexo_pichu_up_special_1_acmd(agent: &mut L2CAgentBase) {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(agent.lua_state_agent);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if DISCHARGE_ACTIVE[entry_id] == true {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 2.0, 70, 50, 0, 20, 1.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            JostleModule::set_status(agent.module_accessor, false);
        }
    }
    if DISCHARGE_ACTIVE[entry_id] == false {
        if macros::is_excute(agent) {
            JostleModule::set_status(agent.module_accessor, false);
        }
    }
}

//Up Special 2 ACMD
unsafe extern "C" fn ssbexo_pichu_up_special_2_acmd(agent: &mut L2CAgentBase) {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(agent.lua_state_agent);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if DISCHARGE_ACTIVE[entry_id] == true {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 3.0, 70, 150, 0, 20, 1.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            JostleModule::set_status(agent.module_accessor, false);
        }
    }
    if DISCHARGE_ACTIVE[entry_id] == false {
        if macros::is_excute(agent) {
            JostleModule::set_status(agent.module_accessor, false);
        }
    }
}

//Up Special Start Effect
unsafe extern "C" fn ssbexo_pichu_up_special_start_effect(agent: &mut L2CAgentBase) {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(agent.lua_state_agent);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if DISCHARGE_ACTIVE[entry_id] == true {
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_cheek_sphistart"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        }
    }
    if DISCHARGE_ACTIVE[entry_id] == false {
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

//Aerial Up Special Effect
unsafe extern "C" fn ssbexo_pichu_aerial_up_special_effect(agent: &mut L2CAgentBase) {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(agent.lua_state_agent);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if DISCHARGE_ACTIVE[entry_id] == true {
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("pichu_denko_elec"), Hash40::new("rot"), 5, 0, 0, 0, 0, 0, 1, true);
            smash::app::sv_animcmd::EFFECT_FLW_POS_UNSYNC_VIS(agent.lua_state_agent);
            agent.pop_lua_stack(1);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
            macros::EFFECT_FLW_UNSYNC_VIS(agent, Hash40::new("pichu_denko_line"), Hash40::new("rot"), 0, 0, 10, 90, 0, 0, 1.5, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
            macros::EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.6, 10, 10, 10, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.6, 10, 10, 10, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.6, 10, 10, 10, 0, 0, 0, false);
        }
    }
    if DISCHARGE_ACTIVE[entry_id] == false {
        if macros::is_excute(agent) {
            macros::EFFECT_FLW_UNSYNC_VIS(agent, Hash40::new("pichu_denko_line"), Hash40::new("rot"), 0, 0, 10, 90, 0, 0, 1.5, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
            macros::EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.6, 10, 10, 10, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.6, 10, 10, 10, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.6, 10, 10, 10, 0, 0, 0, false);
        }
    }
}

//Down Special ACMD
unsafe extern "C" fn ssbexo_pichu_down_special_acmd(agent: &mut L2CAgentBase) {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(agent.lua_state_agent);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if DISCHARGE_ACTIVE[entry_id] == false {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_KAMINARI_GENERATE);
        }
    }
    if DISCHARGE_ACTIVE[entry_id] == true {
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

//Down Special Hit ACMD
unsafe extern "C" fn ssbexo_pichu_down_special_hit_acmd(agent: &mut L2CAgentBase) {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(agent.lua_state_agent);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if DISCHARGE_ACTIVE[entry_id] == false {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 65, 0, 90, 11.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    if DISCHARGE_ACTIVE[entry_id] == true {
        if macros::is_excute(agent) {
            DamageModule::add_damage(agent.module_accessor, -6.125, 0);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 65, 0, 90, 11.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

pub fn install() {
    Agent::new("pichu")
    .game_acmd("game_specialshield", ssbexo_pichu_shield_special_acmd)
    .effect_acmd("effect_specialshield", ssbexo_pichu_shield_special_effect)
    .sound_acmd("sound_specialshield", ssbexo_pichu_shield_special_sound)
    .game_acmd("game_specialn", ssbexo_pichu_neutral_special_shoot_acmd)
    .game_acmd("game_specialairn", ssbexo_pichu_neutral_special_shoot_acmd)
    .game_acmd("game_specialnshoot", ssbexo_pichu_neutral_special_shoot_acmd)
    .effect_acmd("effect_specialn", ssbexo_pichu_neutral_special_shoot_effect)
    .effect_acmd("effect_specialairn", ssbexo_pichu_neutral_special_shoot_effect)
    .effect_acmd("effect_specialnshoot", ssbexo_pichu_neutral_special_shoot_effect)
    .game_acmd("game_specialstackle", ssbexo_pichu_grounded_tackle_acmd)
    .game_acmd("game_specialairstackle", ssbexo_pichu_aerial_tackle_acmd)
    .effect_acmd("effect_specialstackle", ssbexo_pichu_tackle_effect)
    .effect_acmd("effect_specialairstackle", ssbexo_pichu_tackle_effect)
    .sound_acmd("sound_specialstackle", ssbexo_pichu_tackle_sound)
    .sound_acmd("sound_specialairstackle", ssbexo_pichu_tackle_sound)
    .game_acmd("game_specialairsstart", ssbexo_pichu_wild_charge_acmd)
    .game_acmd("game_specialairss", ssbexo_pichu_wild_charge_acmd)
    .game_acmd("game_specialhi1", ssbexo_pichu_up_special_1_acmd)
    .game_acmd("game_specialairhi1", ssbexo_pichu_up_special_1_acmd)
    .game_acmd("game_specialhi2", ssbexo_pichu_up_special_2_acmd)
    .game_acmd("game_specialairhi2", ssbexo_pichu_up_special_2_acmd)
    .effect_acmd("effect_specialhistart", ssbexo_pichu_up_special_start_effect)
    .effect_acmd("effect_specialairhistart", ssbexo_pichu_up_special_start_effect)
    .effect_acmd("effect_specialhi2", ssbexo_pichu_aerial_up_special_effect)
    .effect_acmd("effect_specialairhi2", ssbexo_pichu_aerial_up_special_effect)
    .game_acmd("game_speciallw", ssbexo_pichu_down_special_acmd)
    .game_acmd("game_specialairlw", ssbexo_pichu_down_special_acmd)
    .game_acmd("game_speciallwhit", ssbexo_pichu_down_special_hit_acmd)
    .game_acmd("game_specialairlwhit", ssbexo_pichu_down_special_hit_acmd)
    .install()
    ;
    Agent::new("pichu_dengekidama")
    .game_acmd("game_regular", ssbexo_pichu_electroweb_acmd)
    .install()
    ;
    Agent::new("pichu_dengeki")
    .game_acmd("game_regular", ssbexo_pichu_electroweb_acmd)
    .install()
    ;
}