use super::*;

//Neutral Special Start Expression
unsafe extern "C" fn ssbexo_link_neutral_special_start_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        ArticleModule::change_status_exist(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_HAVE);
    }
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_drawhold"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Arrow Fly ACMD
unsafe extern "C" fn ssbexo_link_bowarrow_fly_acmd(agent: &mut L2CAgentBase) {
    let weapon = get_weapon_common_from_accessor(&mut *(agent.module_accessor));
    let owner_boma = get_owner_boma(weapon);
    let shoot_num = WorkModule::get_int(agent.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_SHOOT_NUM);
    let arrow_type = WorkModule::get_int(agent.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
    let shot_angle = WorkModule::get_float(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLOAT_SHOT_ANGLE);
    let angle = if shot_angle < 0.0 {361 as u64} else {shot_angle as u64};
    if shoot_num > 0 {
        if arrow_type == *WN_LINK_BOWARROW_NORMAL_ARROW {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, angle, 160, 0, 10, 1.35, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            }
        }
        if arrow_type == *WN_LINK_BOWARROW_FIRE_ARROW {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, angle, 50, 0, 35, 1.35, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            }
        }
        if arrow_type == *WN_LINK_BOWARROW_SHOCK_ARROW {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, angle, 70, 0, 25, 1.35, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
            }
        }
        if arrow_type == *WN_LINK_BOWARROW_ICE_ARROW {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, angle, 110, 0, 25, 1.35, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            }
        }
    }
    else {
        if arrow_type == *WN_LINK_BOWARROW_NORMAL_ARROW {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, angle, 60, 0, 10, 1.35, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
                AttackModule::enable_safe_pos(agent.module_accessor);
            }
        }
        if arrow_type == *WN_LINK_BOWARROW_FIRE_ARROW {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, angle, 50, 0, 35, 1.35, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
                AttackModule::enable_safe_pos(agent.module_accessor);
            }
        }
        if arrow_type == *WN_LINK_BOWARROW_SHOCK_ARROW {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, angle, 70, 0, 25, 1.35, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
                AttackModule::enable_safe_pos(agent.module_accessor);
            }
        }
        if arrow_type == *WN_LINK_BOWARROW_ICE_ARROW {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, angle, 30, 0, 25, 1.35, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
                AttackModule::enable_safe_pos(agent.module_accessor);
            }
        }
        if arrow_type == *WN_LINK_BOWARROW_LIGHT_ARROW {
            if WorkModule::is_flag(owner_boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_MAX_CHARGE) {
                if is_excute(agent) {
                    ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, angle, 110, 0, 25, 1.35, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
                    AttackModule::enable_safe_pos(agent.module_accessor);
                }
            }
        }
    }
}

//Arrow Fly Effect
unsafe extern "C" fn ssbexo_link_bowarrow_fly_effect(agent: &mut L2CAgentBase) {
    let arrow_type = WorkModule::get_int(agent.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
    if arrow_type == *WN_LINK_BOWARROW_NORMAL_ARROW {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("link_arrow_trace"), Hash40::new("arrow"), 0, 0, 1, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("link_arrow"), Hash40::new("arrow"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    if arrow_type == *WN_LINK_BOWARROW_FIRE_ARROW {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("link_arrow_trace"), Hash40::new("arrow"), 0, 0, 1, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("link_arrow"), Hash40::new("arrow"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("link_shock_arrow_aura"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 0.3, true);
            LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 0.0);
            EFFECT_FOLLOW(agent, Hash40::new("link_fire_arrow_aura"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 0.6, true);
        }
    }
    if arrow_type == *WN_LINK_BOWARROW_SHOCK_ARROW {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("link_arrow_trace"), Hash40::new("arrow"), 0, 0, 1, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("link_arrow"), Hash40::new("arrow"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("link_shock_arrow_aura"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 0.3, true);
        }
    }
    if arrow_type == *WN_LINK_BOWARROW_ICE_ARROW {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("link_arrow_trace"), Hash40::new("arrow"), 0, 0, 1, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("link_arrow"), Hash40::new("arrow"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("link_shock_arrow_aura"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 0.3, true);
            LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 1.0);
            EFFECT_FOLLOW(agent, Hash40::new("link_ice_arrow_aura"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 0.3, true);
        }
    }
}

//Arrow Stick ACMD
unsafe extern "C" fn ssbexo_link_bowarrow_stick_acmd(agent: &mut L2CAgentBase) {
    let arrow_type = WorkModule::get_int(agent.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
    if arrow_type == *WN_LINK_BOWARROW_FIRE_ARROW {
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            ATTACK(agent, 0, 1, Hash40::new("top"), 3.0, 75, 50, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        }
        frame(agent.lua_state_agent, 4.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

//Boomerang Fly ACMD
unsafe extern "C" fn ssbexo_link_boomerang_fly_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 70, 45, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -4, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 65, 45, 0, 30, 3.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

//Boomerang Turn ACMD
unsafe extern "C" fn ssbexo_link_boomerang_turn_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 70, 40, 0, 50, 3.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -1.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

//Up Special ACMD
unsafe extern "C" fn ssbexo_link_special_hi_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, true, -1);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, false, ArticleOperationTarget(0));
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, Hash40::new("special_hi"), false, -1.0);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        sv_kinetic_energy!(set_limit_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 2.2);
        sv_kinetic_energy!(set_stable_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 2.2);
        sv_kinetic_energy!(set_accel, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.36);
        sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        sv_kinetic_energy!(set_brake, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.05, 0.0);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_CAN_ASCEND);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, true, ArticleOperationTarget(0));
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 60, 145, 0, 50, 6.0, 0.0, 4.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 60, 145, 0, 50, 6.0, 0.0, 4.0, 5.0, None, None, None, 1.0, 0.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        sv_kinetic_energy!(set_limit_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.3);
        sv_kinetic_energy!(set_stable_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.3);
        sv_kinetic_energy!(set_accel, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.07);
        sv_kinetic_energy!(set_brake, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 1.0);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_CAN_ASCEND);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Up Special Effect
unsafe extern "C" fn ssbexo_link_special_hi_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 1.0);
        LAST_EFFECT_SET_RATE(agent, 0.5);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 13, 0, -90, 0, 0, 0.625, true, *EF_FLIP_YZ);
        LAST_PARTICLE_SET_COLOR(agent, 0.25, 1.00, 0.5);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_vector"), Hash40::new("sys_vector"), Hash40::new("top"), 0, 25, 0, -90, 0, 0, 2.0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(agent, 0, 2, 0.5);
        LAST_EFFECT_SET_SCALE_W(agent, 1.0, 3.25, 1);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("brave_tornado1_wind"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0); 
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_vector"), false, false);
    }
}

//Up Special Sound
unsafe extern "C" fn ssbexo_link_special_hi_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_jump02"));
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_special_h01"));
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_link_ottotto"));
    }
}

//Up Special Expression
unsafe extern "C" fn ssbexo_link_special_hi_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
}

//Up Special Spin Attack ACMD
unsafe extern "C" fn ssbexo_link_special_hi_spin_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 367, 100, 107, 0, 6.0, 0.0, 10.0, 14.5, Some(0.0), Some(10.0), Some(8.0), 1.0, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 145, 100, 95, 0, 5.0, 0.0, 11.5, -12.0, Some(0.0), Some(11.0), Some(-9.0), 1.0, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 10.0, y: 20.0}, 10, false);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 107, 0, 6.0, 0.0, 6.0, 15.0, Some(0.0), Some(7.5), Some(8.0), 1.0, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 145, 100, 95, 0, 5.0, 0.0, 11.5, -12.5, Some(0.0), Some(11.0), Some(-9.0), 1.0, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 10.0, y: 20.0}, 10, false);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 83, 100, 71, 0, 6.0, 0.0, 10.0, 15.0, Some(0.0), Some(10.5), Some(8.0), 1.0, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        ATTACK(agent, 1, 0, Hash40::new("sword2"), 2.0, 150, 100, 95, 0, 5.0, 2.0, 0.0, 0.0, Some(5.5), Some(0.0), Some(0.0), 1.0, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 10.0, y: 20.0}, 10, false);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 31.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 69, 100, 62, 0, 7.0, 0.0, 14.0, 12.5, Some(0.0), Some(12.5), Some(8.0), 1.0, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 38.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("sword2"), 2.0, 160, 100, 95, 0, 5.0, 2.0, 0.0, 0.0, Some(5.5), Some(0.0), Some(0.0), 1.0, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 10.0, y: 20.0}, 10, false);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 47.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 200, 0, 30, 7.5, 0.0, 7.5, 14.0, Some(0.0), Some(9.5), Some(7.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 48.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 200, 0, 30, 7.5, 0.0, 13.0, 15.5, Some(0.0), Some(12.5), Some(7.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 49.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 200, 0, 30, 7.5, 0.0, 19.0, 12.5, Some(0.0), Some(14.0), Some(7.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        JostleModule::set_status(agent.module_accessor, true);
    }
}

//Up Special Spin Attack Effect
unsafe extern "C" fn ssbexo_link_special_hi_spin_attack_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_link_sword1"), Hash40::new("tex_link_sword2"), 4, Hash40::new("sword1"), 1.0, 0.0, 0.0, Hash40::new("sword1"), 14.6, 0.2, -0.2, true, Hash40::new("null"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.6, 0.2);
        EFFECT_FOLLOW(agent, Hash40::new("link_sword_flare"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        FLASH(agent, 1, 0.6, 0, 0.1);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("link_kaiten_air"), Hash40::new("top"), 0, 12, 0, 0, -90, 25, 1, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("link_kaiten_air"), Hash40::new("top"), 0, 10, 0, -20, -150, 30, 1, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("link_kaiten_air"), Hash40::new("top"), 0, 11, 0, 0, -120, 20, 1, true);
    }
    frame(agent.lua_state_agent, 27.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("link_kaiten_air"), Hash40::new("top"), 0, 10, 0, 25, -170, 15, 1, true);
    }
    frame(agent.lua_state_agent, 42.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(agent.lua_state_agent, 44.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_link_sword1"), Hash40::new("tex_link_sword2"), 6, Hash40::new("sword1"), 1.0, 0.0, 0.0, Hash40::new("sword1"), 14.6, 0.2, -0.2, true, Hash40::new("null"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.6, 0.2);
    }
    frame(agent.lua_state_agent, 49.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), 13, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 53.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 5, 0, 0, 0, 0);
        AFTER_IMAGE_OFF(agent, 4);
        EFFECT_OFF_KIND(agent, Hash40::new("link_kaiten"), false, false);
    }
    frame(agent.lua_state_agent, 54.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        EFFECT_OFF_KIND(agent, Hash40::new("link_sword_flare"), false, false);
    }
}

//Up Special Spin Attack Sound
unsafe extern "C" fn ssbexo_link_special_hi_spin_attack_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_link_special_h02"));
        PLAY_SE(agent, Hash40::new("se_link_special_h07"));
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_special_h08"));
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_special_h09"));
    }
    frame(agent.lua_state_agent, 47.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_special_h11"));
        PLAY_SE(agent, Hash40::new("se_link_special_h10"));
    }
}

//Up Special Spin Attack Expression
unsafe extern "C" fn ssbexo_link_special_hi_spin_attack_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 9, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashs"), 0);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 11, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 11, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 36.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 11, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 47.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
    frame(agent.lua_state_agent, 51.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Glider Up Special Effect
unsafe extern "C" fn ssbexo_link_glider_special_hi_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("link_entry"), Hash40::new("trans"), 9.5, 3.6, 1.3, 0, 0, 0, 1, false);
        EFFECT_FOLLOW(agent, Hash40::new("link_entry"), Hash40::new("trans"), -9.5, 3.6, 1.3, 0, 0, 0, 1, false);
    }
}

//Up Special Ascend Start ACMD
unsafe extern "C" fn ssbexo_link_special_hi_ascend_start_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE(agent, 0.5);
    if is_excute(agent) {       
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
}

//Up Special Ascend Start Effect
unsafe extern "C" fn ssbexo_link_special_hi_ascend_start_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {        
        let target_y = WorkModule::get_float(agent.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_TARGET_Y);
        let pos_y = PostureModule::pos_y(agent.module_accessor);
        EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, target_y-pos_y+5.0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.25, 1.0, 0.5);
        EFFECT(agent, Hash40::new("sys_assist_out"), Hash40::new("top"), 0, -2.0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.25, 1, 0.5);
        LAST_EFFECT_SET_RATE(agent, 1.25);
        EFFECT_FOLLOW(agent, Hash40::new("sys_aura_light"), Hash40::new("hip"), 0, 0.0, 0, 0, 0, 0, 3.0, false);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {       
        EFFECT(agent, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.25, 1.0, 0.5);
        LAST_EFFECT_SET_RATE(agent, 1.1);
    }
}

//Up Special Ascend Start Sound
unsafe extern "C" fn ssbexo_link_special_hi_ascend_start_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {        
        STOP_SE(agent, Hash40::new("se_link_special_h01"));
        PLAY_SE(agent, Hash40::new("se_link_special_h02"));
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_link_jump01"));
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_escapeair"));
    }
}

//Up Special Ascend Start Expression
unsafe extern "C" fn ssbexo_link_special_hi_ascend_start_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {  
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32); 
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);  
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
}

//Up Special Ascend Effect
unsafe extern "C" fn ssbexo_link_special_hi_ascend_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {        
        let target_y = WorkModule::get_float(agent.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_TARGET_Y);
        let pos_y = PostureModule::pos_y(agent.module_accessor);
        EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, target_y-pos_y+5.0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.25, 1.0, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_aura_light"), Hash40::new("hip"), 0, 0.0, 0, 0, 0, 0, 3.0, false);
    }
    for _ in 0..i32::MAX {
        wait(agent.lua_state_agent, 2.0);
        if is_excute(agent) {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, -4, 0, -90, 0, 0, 1.25, true, *EF_FLIP_YZ);
            LAST_PARTICLE_SET_COLOR(agent, 0.25, 1.0, 0.5);
            FLASH(agent, 0.25, 1.0, 0.5, 0.0);
        }
        wait(agent.lua_state_agent, 3.0);
    }
}

//Up Special Ascend Sound
unsafe extern "C" fn ssbexo_link_special_hi_ascend_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {        
        PLAY_STATUS(agent,Hash40::new("se_link_special_h03"));
    }
}

//Up Special Ascend Expression
unsafe extern "C" fn ssbexo_link_special_hi_ascend_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {  
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32); 
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);  
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
}

//Up Special Ascend End ACMD
unsafe extern "C" fn ssbexo_link_special_hi_ascend_end_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 6.0);
    FT_MOTION_RATE(agent, 1.0);
}

//Up Special Ascend End Effect
unsafe extern "C" fn ssbexo_link_special_hi_ascend_end_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.25, 1.0, 0.5);
        LAST_EFFECT_SET_RATE(agent, 1.1);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0.0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
}

//Up Special Ascend End Sound
unsafe extern "C" fn ssbexo_link_special_hi_ascend_end_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent,Hash40::new("se_link_special_h04"));
    }
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent,Hash40::new("vc_link_cliffcatch"));
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_jump02"));
    }
    wait(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
    wait(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
}

//Up Special Ascend End Expression
unsafe extern "C" fn ssbexo_link_special_hi_ascend_end_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_jump"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Up Special Glide Sound
unsafe extern "C" fn ssbexo_link_special_hi_glide_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        let sound = SoundModule::play_status_se(agent.module_accessor, Hash40::new("se_link_appear01"), true, false, false);
        SoundModule::set_se_vol(agent.module_accessor, sound as i32, 1.2, 0);
    }
}

//Up Special Glide Expression
unsafe extern "C" fn ssbexo_link_special_hi_glide_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            ItemModule::set_attach_item_visibility(agent.module_accessor, false, 0);
        }
        wait(agent.lua_state_agent, 25.0);
        if is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

//Glider Glide Effect
unsafe extern "C" fn ssbexo_link_glider_glide_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("link_entry"), Hash40::new("trans"), 9.5, 3.6, 1.3, 0, 0, 0, 1, false);
        EFFECT_FOLLOW(agent, Hash40::new("link_entry"), Hash40::new("trans"), -9.5, 3.6, 1.3, 0, 0, 0, 1, false);
    }
}

//Up Special Glide Drop ACMD
unsafe extern "C" fn ssbexo_link_special_hi_glide_drop_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Up Special Glide Drop Expression
unsafe extern "C" fn ssbexo_link_special_hi_glide_drop_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_attach_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
}

//Glider Glide Drop Effect
unsafe extern "C" fn ssbexo_link_glider_glide_drop_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("link_entry"), Hash40::new("trans"), 9.5, 3.6, 1.3, 0, 0, 0, 1, false);
        EFFECT_FOLLOW(agent, Hash40::new("link_entry"), Hash40::new("trans"), -9.5, 3.6, 1.3, 0, 0, 0, 1, false);
    }
}

//Up Special Glide Land ACMD
unsafe extern "C" fn ssbexo_link_special_hi_glide_land_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Up Special Glide Land Expression
unsafe extern "C" fn ssbexo_link_special_hi_glide_land_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
    }
}

//Glider Glide Land Effect
unsafe extern "C" fn ssbexo_link_glider_glide_land_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("link_entry"), Hash40::new("trans"), 9.5, 3.6, 1.3, 0, 0, 0, 1, false);
        EFFECT_FOLLOW(agent, Hash40::new("link_entry"), Hash40::new("trans"), -9.5, 3.6, 1.3, 0, 0, 0, 1, false);
    }
}

//Down Special Expression
unsafe extern "C" fn ssbexo_link_down_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 34.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
    }
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .expression_acmd("expression_specialnstart", ssbexo_link_neutral_special_start_expression, Low)
    .expression_acmd("expression_specialairnstart", ssbexo_link_neutral_special_start_expression, Low)
    .game_acmd("game_specialhi", ssbexo_link_special_hi_acmd, Low)
    .game_acmd("game_specialairhi", ssbexo_link_special_hi_acmd, Low)
    .effect_acmd("effect_specialhi", ssbexo_link_special_hi_effect, Low)
    .effect_acmd("effect_specialairhi", ssbexo_link_special_hi_effect, Low)
    .sound_acmd("sound_specialhi", ssbexo_link_special_hi_sound, Low)
    .sound_acmd("sound_specialairhi", ssbexo_link_special_hi_sound, Low)
    .expression_acmd("expression_specialhi", ssbexo_link_special_hi_expression, Low)
    .expression_acmd("expression_specialairhi", ssbexo_link_special_hi_expression, Low)
    .game_acmd("game_specialhispinattack", ssbexo_link_special_hi_spin_attack_acmd, Low)
    .effect_acmd("effect_specialhispinattack", ssbexo_link_special_hi_spin_attack_effect, Low)
    .sound_acmd("sound_specialhispinattack", ssbexo_link_special_hi_spin_attack_sound, Low)
    .expression_acmd("expression_specialhispinattack", ssbexo_link_special_hi_spin_attack_expression, Low)
    .game_acmd("game_specialhiascendstart", ssbexo_link_special_hi_ascend_start_acmd, Low)
    .effect_acmd("effect_specialhiascendstart", ssbexo_link_special_hi_ascend_start_effect, Low)
    .sound_acmd("sound_specialhiascendstart", ssbexo_link_special_hi_ascend_start_sound, Low)
    .expression_acmd("expression_specialhiascendstart", ssbexo_link_special_hi_ascend_start_expression, Low)
    .effect_acmd("effect_specialhiascend", ssbexo_link_special_hi_ascend_effect, Low)
    .sound_acmd("sound_specialhiascend", ssbexo_link_special_hi_ascend_sound, Low)
    .expression_acmd("expression_specialhiascend", ssbexo_link_special_hi_ascend_expression, Low)
    .game_acmd("game_specialhiend", ssbexo_link_special_hi_ascend_end_acmd, Low)
    .effect_acmd("effect_specialhiend", ssbexo_link_special_hi_ascend_end_effect, Low)
    .sound_acmd("sound_specialhiend", ssbexo_link_special_hi_ascend_end_sound, Low)
    .expression_acmd("expression_specialhiend", ssbexo_link_special_hi_ascend_end_expression, Low)
    .sound_acmd("sound_specialhiglide", ssbexo_link_special_hi_glide_sound, Low)
    .expression_acmd("expression_specialhiglide", ssbexo_link_special_hi_glide_expression, Low)
    .sound_acmd("sound_specialhiglidef", ssbexo_link_special_hi_glide_sound, Low)
    .expression_acmd("expression_specialhiglidef", ssbexo_link_special_hi_glide_expression, Low)
    .sound_acmd("sound_specialhiglideb", ssbexo_link_special_hi_glide_sound, Low)
    .expression_acmd("expression_specialhiglideb", ssbexo_link_special_hi_glide_expression, Low)
    .game_acmd("game_specialhiglidedrop", ssbexo_link_special_hi_glide_drop_acmd, Low)
    .expression_acmd("expression_specialhiglidedrop", ssbexo_link_special_hi_glide_drop_expression, Low)
    .game_acmd("game_specialhiglideland", ssbexo_link_special_hi_glide_land_acmd, Low)
    .expression_acmd("expression_specialhiglideland", ssbexo_link_special_hi_glide_land_expression, Low)
    .expression_acmd("expression_speciallw", ssbexo_link_down_special_expression, Low)
    .expression_acmd("expression_specialairlw", ssbexo_link_down_special_expression, Low)
    .install()
    ;
    Agent::new("link_bowarrow")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_fly", ssbexo_link_bowarrow_fly_acmd, Low)
    .effect_acmd("effect_fly", ssbexo_link_bowarrow_fly_effect, Low)
    .game_acmd("game_stick", ssbexo_link_bowarrow_stick_acmd, Low)
    .game_acmd("game_hitstick", ssbexo_link_bowarrow_stick_acmd, Low)
    .install()
    ;
    Agent::new("link_boomerang")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_fly", ssbexo_link_boomerang_fly_acmd, Low)
    .game_acmd("game_turn", ssbexo_link_boomerang_turn_acmd, Low)
    .install()
    ;
    Agent::new("link_parasail")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .effect_acmd("effect_specialhi", ssbexo_link_glider_special_hi_effect, Low)
    .effect_acmd("effect_glide", ssbexo_link_glider_glide_effect, Low)
    .effect_acmd("effect_glidedrop", ssbexo_link_glider_glide_drop_effect, Low)
    .effect_acmd("effect_glideland", ssbexo_link_glider_glide_land_effect, Low)
    .install()
    ;
}