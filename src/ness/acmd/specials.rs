use super::*;

//Shield Special Effect
unsafe extern "C" fn ssbexo_ness_shield_special_effect(agent: &mut L2CAgentBase) {
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
        macros::EFFECT_FLW_POS(agent, Hash40::new("ness_pkfl_bullet"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.3, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("ness_pkfl_bullet"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.3, true);
    }
    frame(agent.lua_state_agent, 16.0);
    for _ in 0..13 {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("ness_pkfl_bullet"), true, true);
            macros::EFFECT_OFF_KIND(agent, Hash40::new("ness_pkfl_hold"), true, true);
            macros::EFFECT_FLW_POS(agent, Hash40::new("ness_pkfl_bullet"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.6, true);
            macros::EFFECT_FLW_POS(agent, Hash40::new("ness_pkfl_bullet"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.6, true);
            macros::EFFECT_FLW_POS(agent, Hash40::new("ness_pkfl_hold"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 1, true);
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
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ness_pkfl_bullet"), true, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ness_pkfl_hold"), true, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("ness_pkfl_bullet"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("ness_pkfl_bullet"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("ness_pkfl_hold"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.9, true);
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
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ness_pkfl_hold"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ness_pkfl_bullet"), false, false);
    }
}

//Shield Special Sound
unsafe extern "C" fn ssbexo_ness_shield_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ness_special_n05"));
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_ness_special_n02"));
    }
    frame(agent.lua_state_agent, 109.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_ness_special_n02"));
    }
    frame(agent.lua_state_agent, 115.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ness_attack05"));
        macros::PLAY_SE(agent, Hash40::new("se_ness_special_n04_ll"));
    }
}

//Shield Special Expression
unsafe extern "C" fn ssbexo_ness_shield_special_expression(agent: &mut L2CAgentBase) {
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
unsafe extern "C" fn ssbexo_ness_shield_special_burst_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 361, 80, 0, 40, 12.0, 0.0, 6.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Shield Special Burst Effect
unsafe extern "C" fn ssbexo_ness_shield_special_burst_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("ness_pkfl_bullet"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.6, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("ness_pkfl_bullet"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.6, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("ness_pkfl_hold"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.9, true);
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
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ness_pkfl_bullet"), false, false);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("ness_pkfl_bomb_max"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.5, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("ness_pkfl_bullet_ed"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.5, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ness_pkfl_hold"), false, false);
    }
}

//Shield Special Burst Sound
unsafe extern "C" fn ssbexo_ness_shield_special_burst_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ness_attack05"));
        macros::PLAY_SE(agent, Hash40::new("se_ness_special_n04_ll"));
    }
}

//Shield Special Burst Expression
unsafe extern "C" fn ssbexo_ness_shield_special_burst_expression(agent: &mut L2CAgentBase) {
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

//PK Flash Bang ACMD
unsafe extern "C" fn ssbexo_ness_pkflash_bang_acmd(agent: &mut L2CAgentBase) {
    let float_count = WorkModule::get_float(agent.module_accessor, *WEAPON_NESS_PK_FLASH_INSTANCE_WORK_ID_FLOAT_COUNT);
    let rng = sv_math::rand(hash40("agent"), 100);
    let offense_up_hashes = if (60..100).contains(&rng) {Hash40::new("collision_attr_death")} else if (25..59).contains(&rng) {Hash40::new("collision_attr_head_mushroom")} else if (14..24).contains(&rng) {Hash40::new("collision_attr_paralyze")} else {Hash40::new("collision_attr_normal_poison")};
    let default_hashes = if (53..100).contains(&rng) {Hash40::new("collision_attr_paralyze")} else if (18..52).contains(&rng) {Hash40::new("collision_attr_head_mushroom")} else if (3..17).contains(&rng) {Hash40::new("collision_attr_normal_poison")} else {Hash40::new("collision_attr_death")};
    if float_count > 90.0 {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
            if offense_up_hashes == Hash40::new("collision_attr_normal_poison") {
                if macros::is_excute(agent) {
                    macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 70, 65, 0, 50, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, offense_up_hashes, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
                    AttackModule::set_poison_param(agent.module_accessor, 0, 600, 20, 0.2, false);
                }
            }
            else {
                if macros::is_excute(agent) {
                    macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 70, 65, 0, 50, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, offense_up_hashes, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
                }
            }
        }
        else {
            if default_hashes == Hash40::new("collision_attr_normal_poison") {
                if macros::is_excute(agent) {
                    macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 70, 65, 0, 50, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, default_hashes, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
                    AttackModule::set_poison_param(agent.module_accessor, 0, 600, 20, 0.2, false);
                }
            }
            else {
                if macros::is_excute(agent) {
                    macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 70, 65, 0, 50, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, default_hashes, *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
                }
            }
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 70, 65, 0, 50, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        }
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x27936db96d));
    }
}

//PK Fire Grounded Pillar ACMD
unsafe extern "C" fn ssbexo_ness_pkfire_grounded_pillar_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 68, 30, 0, 10, 6.5, 0.0, 3.1, 2.0, None, None, None, 1.0, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 68, 30, 0, 10, 4.5, 0.0, 9.6, 2.0, None, None, None, 1.0, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        macros::AREA_WIND_2ND_RAD_arg9(agent, 0, 1, 0.05, 200, 0.6, 0, 10, 20, 60);
    }
}

//PK Fire Aerial Pillar ACMD
unsafe extern "C" fn ssbexo_ness_pkfire_aerial_pillar_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 58, 45, 0, 16, 6.5, 0.0, 3.1, 2.0, None, None, None, 1.0, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 58, 45, 0, 16, 4.5, 0.0, 9.6, 2.0, None, None, None, 1.0, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        macros::AREA_WIND_2ND_RAD_arg9(agent, 0, 1, 0.05, 200, 0.6, 0, 10, 20, 60);
    }
}

pub fn install() {
    Agent::new("ness")
    .effect_acmd("effect_specialguard", ssbexo_ness_shield_special_effect, Priority::Low)
    .sound_acmd("sound_specialguard", ssbexo_ness_shield_special_sound, Priority::Low)
    .expression_acmd("expression_specialguard", ssbexo_ness_shield_special_expression, Priority::Low)
    .game_acmd("game_specialguardburst", ssbexo_ness_shield_special_burst_acmd, Priority::Low)
    .effect_acmd("effect_specialguardburst", ssbexo_ness_shield_special_burst_effect, Priority::Low)
    .sound_acmd("sound_specialguardburst", ssbexo_ness_shield_special_burst_sound, Priority::Low)
    .expression_acmd("expression_specialguardburst", ssbexo_ness_shield_special_burst_expression, Priority::Low)
    .install()
    ;
    Agent::new("ness_pkflash")
    .game_acmd("game_bang", ssbexo_ness_pkflash_bang_acmd, Priority::Low)
    .install()
    ;
    Agent::new("ness_pkfire")
    .game_acmd("game_pillar", ssbexo_ness_pkfire_grounded_pillar_acmd, Priority::Low)
    .game_acmd("game_pillarair", ssbexo_ness_pkfire_aerial_pillar_acmd, Priority::Low)
    .install()
    ;
}