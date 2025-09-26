use super::*;

//Grounded Star Punch ACMD
unsafe extern "C" fn ssbexo_littlemac_grounded_star_punch_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        let star_punch_strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
        let damage = if star_punch_strength == 3 {35.0} else if star_punch_strength == 2 {29.0} else if star_punch_strength == 1 {22.0} else {15.0};
        let hitstop = if star_punch_strength == 3 {1.7} else if star_punch_strength == 2 {1.5} else if star_punch_strength == 1 {1.2} else {1.0};
        let priority = if star_punch_strength == 3 {*ATTACK_SETOFF_KIND_THRU} else {*ATTACK_SETOFF_KIND_ON}; 
        let hit_sound = if star_punch_strength == 3 {*COLLISION_SOUND_ATTR_HEAVY} else if star_punch_strength == 2 {*COLLISION_SOUND_ATTR_KICK} else {*COLLISION_SOUND_ATTR_PUNCH};
        ATTACK(agent, 0, 0, Hash40::new("armr"), damage, 80, 100, 0, 25, 4.5, 3.0, 0.0, 0.0, None, None, None, hitstop, 1.0, priority, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, hit_sound, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), damage, 80, 100, 0, 25, 4.5, 0.0, 9.0, 5.0, None, None, None, hitstop, 1.0, priority, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, hit_sound, *ATTACK_REGION_PUNCH);
        if star_punch_strength == 3 {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
            AttackModule::set_damage_shake_scale(agent.module_accessor, 0.67);
        }
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 1, false);
        SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        let star_punch_strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
        if star_punch_strength == 3 {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_RESET_KO_GAUGE);
        }
    }
    frame(agent.lua_state_agent, 39.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_KO_GRAVITY);
    }
}

//Grounded Star Punch Effect
unsafe extern "C" fn ssbexo_littlemac_grounded_star_punch_effect(agent: &mut L2CAgentBase) {
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("littlemac_ko_uppercut_start"), Hash40::new("handr"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("littlemac_ko_uppercut"), Hash40::new("handr"), 0.5, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 8.0);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            let star_punch_strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
            if star_punch_strength == 3 {
                EFFECT_FOLLOW(agent, Hash40::new("littlemac_ko_uppercut_arc"), Hash40::new("rot"), -4, 1, -3, -15, -60, 90, 1, true);
                EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("littlemac_ko_uppercut_arc_splash"), Hash40::new("rot"), -4, 1, -3, -15, -60, 90, 1, false);
            }
            else if star_punch_strength == 2 {
                EFFECT_FOLLOW(agent, Hash40::new("littlemac_attack_ground"), Hash40::new("top"), -4, 10, 0, 0, 0, 0, 1.8, false);
                LAST_PARTICLE_SET_COLOR(agent, 1, 1, 0);
                EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("littlemac_ko_uppercut_arc_splash"), Hash40::new("rot"), -4, 1, -3, -15, -60, 90, 1, false);
            }
            else if star_punch_strength == 1 {
                EFFECT_FOLLOW(agent, Hash40::new("littlemac_attack_ground"), Hash40::new("top"), -4, 10, 0, 0, 0, 0, 2.0, false);
            }
            else {
                EFFECT(agent, Hash40::new("littlemac_attack_arc_glove_b"), Hash40::new("top"), -1, 11, -6, -10, -45, 120, 2.0, 0, 0, 0, 0, 0, 0, true);
                match color {
                    _ if [0, 4, 5, 6].contains(&color) => {
                        LAST_PARTICLE_SET_COLOR(agent, 0.43, 1, 0.3);
                    }
                    1 => {
                        LAST_PARTICLE_SET_COLOR(agent, 1, 0.6, 0.3);
                    }
                    2 => {
                        LAST_PARTICLE_SET_COLOR(agent, 0.4, 0.4, 0.4);
                    }
                    3 => {
                        LAST_PARTICLE_SET_COLOR(agent, 1, 0.3, 0.3);
                    }
                    7 => {
                        LAST_PARTICLE_SET_COLOR(agent, 1, 0.4, 0.5);
                    }
                    _ => {}
                }
            }
        }
    }
    else {
        if is_excute(agent) {
            let star_punch_strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
            if star_punch_strength == 3 {
                EFFECT_FOLLOW(agent, Hash40::new("littlemac_ko_uppercut_arc"), Hash40::new("rot"), 0.5, 1, -3, 0, -60, 70, 1, true);
                EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("littlemac_ko_uppercut_arc_splash"), Hash40::new("rot"), 0.5, 1, -3, 0, -60, 70, 1, false);
            }
            else if star_punch_strength == 2 {
                EFFECT_FOLLOW(agent, Hash40::new("littlemac_attack_ground"), Hash40::new("top"), -4, 10, 0, 0, 0, 0, 1.8, false);
                LAST_PARTICLE_SET_COLOR(agent, 1, 1, 0);
                EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("littlemac_ko_uppercut_arc_splash"), Hash40::new("rot"), -4, 1, -3, -15, -60, 90, 1, false);
            }
            else if star_punch_strength == 1 {
                EFFECT_FOLLOW(agent, Hash40::new("littlemac_attack_ground"), Hash40::new("top"), 0.5, 10, 0, 0, 0, 0, 2.0, false);
            }
            else {
                EFFECT(agent, Hash40::new("littlemac_attack_arc_glove_b"), Hash40::new("top"), 3, 11, -6, -10, -45, 120, 2.0, 0, 0, 0, 0, 0, 0, true);
                match color {
                    _ if [0, 4, 5, 6].contains(&color) => {
                        LAST_PARTICLE_SET_COLOR(agent, 0.43, 1, 0.3);
                    }
                    1 => {
                        LAST_PARTICLE_SET_COLOR(agent, 1, 0.6, 0.3);
                    }
                    2 => {
                        LAST_PARTICLE_SET_COLOR(agent, 0.4, 0.4, 0.4);
                    }
                    3 => {
                        LAST_PARTICLE_SET_COLOR(agent, 1, 0.3, 0.3);
                    }
                    7 => {
                        LAST_PARTICLE_SET_COLOR(agent, 1, 0.4, 0.5);
                    }
                    _ => {}
                }
            }
        }
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 6, 0, 0, 0, 180, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("littlemac_ko_uppercut"), false, false);
    }
}

//Aerial Star Punch ACMD
unsafe extern "C" fn ssbexo_littlemac_aerial_star_punch_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        let star_punch_strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
        let damage = if star_punch_strength == 3 {25.0} else if star_punch_strength == 2 {21.0} else if star_punch_strength == 1 {18.0} else {15.0};
        let hitstop = if star_punch_strength == 3 {1.7} else if star_punch_strength == 2 {1.5} else if star_punch_strength == 1 {1.2} else {1.0};
        let priority = if star_punch_strength == 3 {*ATTACK_SETOFF_KIND_THRU} else {*ATTACK_SETOFF_KIND_ON}; 
        let hit_sound = if star_punch_strength == 3 {*COLLISION_SOUND_ATTR_HEAVY} else if star_punch_strength == 2 {*COLLISION_SOUND_ATTR_KICK} else {*COLLISION_SOUND_ATTR_PUNCH};
        ATTACK(agent, 0, 0, Hash40::new("armr"), damage, 80, 95, 0, 50, 5.0, 3.0, 0.0, 0.0, None, None, None, hitstop, 1.0, priority, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, hit_sound, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), damage, 80, 95, 0, 50, 3.0, -1.0, 0.0, 0.0, None, None, None, hitstop, 1.0, priority, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, hit_sound, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("shoulderr"), damage, 80, 95, 0, 50, 3.0, 0.0, 0.0, 0.0, None, None, None, hitstop, 1.0, priority, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, hit_sound, *ATTACK_REGION_PUNCH);
        if star_punch_strength == 3  {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
            AttackModule::set_damage_shake_scale(agent.module_accessor, 0.67);
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        let star_punch_strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
        if star_punch_strength == 3 {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_RESET_KO_GAUGE);
        }
    }
    frame(agent.lua_state_agent, 39.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_KO_GRAVITY);
    }
}

//Aerial Star Punch Effect
unsafe extern "C" fn ssbexo_littlemac_aerial_star_punch_effect(agent: &mut L2CAgentBase) {
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("littlemac_ko_uppercut_start"), Hash40::new("handr"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("littlemac_ko_uppercut"), Hash40::new("handr"), 0.5, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("littlemac_ko_uppercut_start"), -1);
    }
    frame(agent.lua_state_agent, 8.0);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            let star_punch_strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
            if star_punch_strength == 3 {
                EFFECT_FOLLOW(agent, Hash40::new("littlemac_ko_uppercut_arc"), Hash40::new("rot"), -4, 1, -3, -15, -60, 90, 1, true);
                EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("littlemac_ko_uppercut_arc_splash"), Hash40::new("rot"), -4, 1, -3, -15, -60, 90, 1, false);
            }
            else if star_punch_strength == 2 {
                EFFECT_FOLLOW(agent, Hash40::new("littlemac_attack_ground"), Hash40::new("top"), -4, 10, 0, 0, 0, 0, 1.8, false);
                LAST_PARTICLE_SET_COLOR(agent, 1, 1, 0);
                EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("littlemac_ko_uppercut_arc_splash"), Hash40::new("rot"), -4, 1, -3, -15, -60, 90, 1, false);
            }
            else if star_punch_strength == 1 {
                EFFECT_FOLLOW(agent, Hash40::new("littlemac_attack_ground"), Hash40::new("top"), -4, 10, 0, 0, 0, 0, 2.0, false);
            }
            else {
                EFFECT(agent, Hash40::new("littlemac_attack_arc_glove_b"), Hash40::new("top"), -1, 11, -6, -10, -45, 120, 2.0, 0, 0, 0, 0, 0, 0, true);
                match color {
                    _ if [0, 4, 5, 6].contains(&color) => {
                        LAST_PARTICLE_SET_COLOR(agent, 0.43, 1, 0.3);
                    }
                    1 => {
                        LAST_PARTICLE_SET_COLOR(agent, 1, 0.6, 0.3);
                    }
                    2 => {
                        LAST_PARTICLE_SET_COLOR(agent, 0.4, 0.4, 0.4);
                    }
                    3 => {
                        LAST_PARTICLE_SET_COLOR(agent, 1, 0.3, 0.3);
                    }
                    7 => {
                        LAST_PARTICLE_SET_COLOR(agent, 1, 0.4, 0.5);
                    }
                    _ => {}
                }
            }
        }
    }
    else {
        if is_excute(agent) {
            let star_punch_strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
            if star_punch_strength == 3 {
                EFFECT_FOLLOW(agent, Hash40::new("littlemac_ko_uppercut_arc"), Hash40::new("rot"), 0.5, 1, -3, 0, -60, 70, 1, true);
                EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("littlemac_ko_uppercut_arc_splash"), Hash40::new("rot"), 0.5, 1, -3, 0, -60, 70, 1, false);
            }
            else if star_punch_strength == 2 {
                EFFECT_FOLLOW(agent, Hash40::new("littlemac_attack_ground"), Hash40::new("top"), -4, 10, 0, 0, 0, 0, 1.8, false);
                LAST_PARTICLE_SET_COLOR(agent, 1, 1, 0);
            }
            else if star_punch_strength == 1 {
                EFFECT_FOLLOW(agent, Hash40::new("littlemac_attack_ground"), Hash40::new("top"), 0.5, 10, 0, 0, 0, 0, 2.0, false);
            }
            else {
                EFFECT(agent, Hash40::new("littlemac_attack_arc_glove_b"), Hash40::new("top"), 3, 11, -6, -10, -45, 120, 2.0, 0, 0, 0, 0, 0, 0, true);
                match color {
                    _ if [0, 4, 5, 6].contains(&color) => {
                        LAST_PARTICLE_SET_COLOR(agent, 0.43, 1, 0.3);
                    }
                    1 => {
                        LAST_PARTICLE_SET_COLOR(agent, 1, 0.6, 0.3);
                    }
                    2 => {
                        LAST_PARTICLE_SET_COLOR(agent, 0.4, 0.4, 0.4);
                    }
                    3 => {
                        LAST_PARTICLE_SET_COLOR(agent, 1, 0.3, 0.3);
                    }
                    7 => {
                        LAST_PARTICLE_SET_COLOR(agent, 1, 0.4, 0.5);
                    }
                    _ => {}
                }
            }
        }
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("littlemac_ko_uppercut"), false, false);
    }
}

//Jolt Haymaker Attack ACMD
unsafe extern "C" fn ssbexo_littlemac_jolt_haymaker_attack_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        let star_punch_strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
        let knockback_growth = if star_punch_strength == 3 {72} else if star_punch_strength == 2 {63} else {48};
        let base_knockback = if star_punch_strength == 3 {85} else if star_punch_strength == 2 {75} else {65};
        ATTACK(agent, 0, 0, Hash40::new("arml"), 14.0, 361, knockback_growth, 0, base_knockback, 5.0, 3.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 14.0, 361, knockback_growth, 0, base_knockback, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 14.0, 361, knockback_growth, 0, base_knockback, 3.5, 0.0, 4.5, 5.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("top"), 14.0, 361, knockback_growth, 0, base_knockback, 4.0, 0.0, 17.0, 10.0, Some(0.0), Some(10.0), Some(11.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 3, false);
    }
    wait(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Jolt Haymaker Attack Effect
unsafe extern "C" fn ssbexo_littlemac_jolt_haymaker_attack_effect(agent: &mut L2CAgentBase) {
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if is_excute(agent) {
        let star_punch_strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
        if star_punch_strength == 3 {
            EFFECT_FOLLOW(agent, Hash40::new("littlemac_ko_uppercut_arc"), Hash40::new("rot"), 0.5, 1, -3, 0, 0, -80, 1, true);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("littlemac_ko_uppercut_arc_splash"), Hash40::new("rot"), 0.5, 1, -3, 0, -80, 0, 1, false);
        }
        else if star_punch_strength == 2 {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("littlemac_attack_ground"), Hash40::new("littlemac_attack_ground"), Hash40::new("top"), -4, 10, 0, -90, 0, 0, 1.8, false, *EF_FLIP_ZX);
            LAST_PARTICLE_SET_COLOR(agent, 1, 1, 0);
        }
        else if star_punch_strength == 1 {
            EFFECT_FOLLOW(agent, Hash40::new("littlemac_attack_arc2_aura"), Hash40::new("top"), 1, 10, -1.5, 0, -20, -110, 1, false);
            EFFECT_FOLLOW(agent, Hash40::new("littlemac_attack_arc2"), Hash40::new("top"), 1, 10, -1.5, 0, -20, -110, 1, false);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("littlemac_attack_arc2_splash"), Hash40::new("top"), 1, 10, -1.5, 0, -20, -110, 1, false);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc"), Hash40::new("top"), -1, 9.5, -3, -180, 160, 80, 0.9, false);
            match color {
                _ if [0, 4, 5, 6].contains(&color) => {
                    LAST_PARTICLE_SET_COLOR(agent, 0.43, 1, 0.3);
                }
                1 => {
                    LAST_PARTICLE_SET_COLOR(agent, 1, 0.6, 0.3);
                }
                2 => {
                    LAST_PARTICLE_SET_COLOR(agent, 0.4, 0.4, 0.4);
                }
                3 => {
                    LAST_PARTICLE_SET_COLOR(agent, 1, 0.3, 0.3);
                }
                7 => {
                    LAST_PARTICLE_SET_COLOR(agent, 1, 0.4, 0.5);
                }
                _ => {}
            }
        }
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("littlemac_joltblow"), false, false);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
}

//Rising Uppercut Jump ACMD
unsafe extern "C" fn ssbexo_littlemac_rising_uppercut_jump_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 367, 50, 0, 40, 6.0, 0.0, 26.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        let star_punch_strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
        let knockback_growth = if star_punch_strength == 3 {210} else if star_punch_strength == 2 {197} else {160};
        let base_knockback = if star_punch_strength == 3 {50} else if star_punch_strength == 2 {45} else {40};
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 80, knockback_growth, 0, base_knockback, 8.0, 0.0, 28.0, 0.0, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Slip Counter ACMD
unsafe extern "C" fn ssbexo_littlemac_slip_counter_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_SHIELD);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_SHIELD);
    }
}

//Grounded Slip Counter Hit ACMD
unsafe extern "C" fn ssbexo_littlemac_grounded_slip_counter_hit_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_COUNTER_ATTACK_START);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

//Aerial Slip Counter Hit ACMD
unsafe extern "C" fn ssbexo_littlemac_aerial_slip_counter_hit_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_COUNTER_ATTACK_START);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_COUNTER_ENABLE_GRAVITY);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

//Slip Counter Hit Effect
unsafe extern "C" fn ssbexo_littlemac_slip_counter_hit_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 14.8, -1, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("littlemac_counter_success"), Hash40::new("top"), -1, 11, -5.5, 0, 90, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
}

//Slip Counter Hit Sound
unsafe extern "C" fn ssbexo_littlemac_slip_counter_hit_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_littlemac_escape"));
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_littlemac_special_l02"));
        PLAY_SE(agent, Hash40::new("vc_littlemac_special_l02"));
    }
    wait(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_littlemac_special_l03"));
    }
}

//Slip Counter Hit Expression
unsafe extern "C" fn ssbexo_littlemac_slip_counter_hit_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_counter"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("littlemac")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialn2", ssbexo_littlemac_grounded_star_punch_acmd, Low)
    .effect_acmd("effect_specialn2", ssbexo_littlemac_grounded_star_punch_effect, Low)
    .game_acmd("game_specialairn2", ssbexo_littlemac_aerial_star_punch_acmd, Low)
    .effect_acmd("effect_specialairn2", ssbexo_littlemac_aerial_star_punch_effect, Low)
    .game_acmd("game_specialairsblow", ssbexo_littlemac_jolt_haymaker_attack_acmd, Low)
    .effect_acmd("effect_specialairsblow", ssbexo_littlemac_jolt_haymaker_attack_effect, Low)
    .game_acmd("game_specialhijump", ssbexo_littlemac_rising_uppercut_jump_acmd, Low)
    .game_acmd("game_speciallw", ssbexo_littlemac_slip_counter_acmd, Low)
    .game_acmd("game_specialairlw", ssbexo_littlemac_slip_counter_acmd, Low)
    .game_acmd("game_speciallwhit", ssbexo_littlemac_grounded_slip_counter_hit_acmd, Low)
    .game_acmd("game_specialairlwhit", ssbexo_littlemac_aerial_slip_counter_hit_acmd, Low)
    .game_acmd("game_speciallwhitf", ssbexo_littlemac_grounded_slip_counter_hit_acmd, Low)
    .game_acmd("game_specialairlwhitf", ssbexo_littlemac_aerial_slip_counter_hit_acmd, Low)
    .effect_acmd("effect_speciallwhit", ssbexo_littlemac_slip_counter_hit_effect, Low)
    .effect_acmd("effect_specialairlwhit", ssbexo_littlemac_slip_counter_hit_effect, Low)
    .effect_acmd("effect_speciallwhitf", ssbexo_littlemac_slip_counter_hit_effect, Low)
    .effect_acmd("effect_specialairlwhitf", ssbexo_littlemac_slip_counter_hit_effect, Low)
    .sound_acmd("sound_speciallwhit", ssbexo_littlemac_slip_counter_hit_sound, Low)
    .sound_acmd("sound_specialairlwhit", ssbexo_littlemac_slip_counter_hit_sound, Low)
    .sound_acmd("sound_speciallwhitf", ssbexo_littlemac_slip_counter_hit_sound, Low)
    .sound_acmd("sound_specialairlwhitf", ssbexo_littlemac_slip_counter_hit_sound, Low)
    .expression_acmd("expression_speciallwhit", ssbexo_littlemac_slip_counter_hit_expression, Low)
    .expression_acmd("expression_specialairlwhit", ssbexo_littlemac_slip_counter_hit_expression, Low)
    .expression_acmd("expression_speciallwhitf", ssbexo_littlemac_slip_counter_hit_expression, Low)
    .expression_acmd("expression_specialairlwhitf", ssbexo_littlemac_slip_counter_hit_expression, Low)
    .install()
    ;
}