use super::*;

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

pub fn install() {
    Agent::new("littlemac")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialairsblow", ssbexo_littlemac_jolt_haymaker_attack_acmd, Low)
    .effect_acmd("effect_specialairsblow", ssbexo_littlemac_jolt_haymaker_attack_effect, Low)
    .install()
    ;
}