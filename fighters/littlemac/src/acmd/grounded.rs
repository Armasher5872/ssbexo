use super::*;

//Jab 1 ACMD
unsafe extern "C" fn ssbexo_littlemac_jab_1_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SUCCESSFUL_DREAMLAND_EXPRESS_INPUTS);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 361, 25, 0, 25, 1.8, 0.0, 10.0, 5.0, Some(0.0), Some(5.0), Some(0.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 361, 25, 0, 25, 1.8, 0.0, 10.0, 8.0, Some(0.0), Some(5.0), Some(8.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 1.5, 180, 20, 0, 25, 2.3, 0.0, 10.0, 11.0, Some(0.0), Some(5.0), Some(11.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("top"), 1.5, 361, 20, 0, 25, 2.3, 0.0, 10.0, 11.0, Some(0.0), Some(5.0), Some(11.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_MIDDLE), false);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_CAN_INPUT_DREAMLAND_EXPRESS);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_CAN_INPUT_DREAMLAND_EXPRESS);
        WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SUCCESSFUL_DREAMLAND_EXPRESS_INPUTS);
    }
}

//Jab 2 ACMD
unsafe extern "C" fn ssbexo_littlemac_jab_2_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 361, 25, 0, 25, 2.5, 0.0, 9.0, 5.0, Some(0.0), Some(6.0), Some(5.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 361, 25, 0, 25, 2.5, 0.0, 9.0, 8.0, Some(0.0), Some(6.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 1.5, 361, 20, 0, 20, 3.3, 0.0, 9.0, 12.0, Some(0.0), Some(6.0), Some(12.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 2.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 2.0, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_CAN_INPUT_DREAMLAND_EXPRESS);
    }
    FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_CAN_INPUT_DREAMLAND_EXPRESS);
        WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SUCCESSFUL_DREAMLAND_EXPRESS_INPUTS);
    }
    frame(agent.lua_state_agent, 19.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 21.0);
    FT_MOTION_RATE(agent, 1.0);
}

//Jab 3 ACMD
unsafe extern "C" fn ssbexo_littlemac_jab_3_acmd(agent: &mut L2CAgentBase) {
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        let successful_dreamland_express_inputs = WorkModule::get_int(agent.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SUCCESSFUL_DREAMLAND_EXPRESS_INPUTS);
        if successful_dreamland_express_inputs == 2 {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DREAMLAND_EXPRESS);
            WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SUCCESSFUL_DREAMLAND_EXPRESS_INPUTS);
        }
    }
    frame(agent.lua_state_agent, 4.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DREAMLAND_EXPRESS) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 75, 10, 0, 60, 3.5, 0.0, 8.0, 6.0, None, None, None, 2.0, 0.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("arml"), 5.0, 75, 10, 0, 60, 3.2, 2.0, 0.0, 0.0, None, None, None, 2.0, 0.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 30.0, false);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 30.0, false);
        }
    }
    else {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 60, 100, 0, 50, 3.5, 0.0, 8.0, 6.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("arml"), 5.0, 60, 100, 0, 50, 3.2, 2.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DREAMLAND_EXPRESS);
    }
}

//Jab 3 Effect
unsafe extern "C" fn ssbexo_littlemac_jab_3_effect(agent: &mut L2CAgentBase) {
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    frame(agent.lua_state_agent, 1.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DREAMLAND_EXPRESS) {
        if is_excute(agent) {
            FLASH(agent, 1, 1, 1, 0.8);
        }
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DREAMLAND_EXPRESS) {
            EFFECT_FOLLOW(agent, Hash40::new("littlemac_straight_aura"), Hash40::new("handl"), 0, 0, 0, 0, 90, 0, 1, true);
        }
        EFFECT(agent, Hash40::new("littlemac_attack_arc_glove_b"), Hash40::new("top"), -1, 11, 1, -10, -45, 120, 0.8, 0, 0, 0, 0, 0, 0, true);
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
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 10, 15, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, false, 0.7);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0.5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("littlemac_straight_aura"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("littlemac_attack_arc_glove_b"), true, true);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_littlemac_dash_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 10.0, 70, 55, 0, 60, 4.5, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 10.0, 70, 55, 0, 60, 3.5, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("shoulderr"), 10.0, 70, 55, 0, 60, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 6.0, 70, 55, 0, 60, 4.5, 3.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 6.0, 70, 55, 0, 60, 3.5, -1.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("shoulderr"), 6.0, 70, 55, 0, 60, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Dash Attack Effect
unsafe extern "C" fn ssbexo_littlemac_dash_attack_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("littlemac_straight_line"), Hash40::new("top"), 0, 6, 2, 0, 0, 0, 0.5, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("littlemac_straight_max"), Hash40::new("top"), -1.3, 8, 7, 0, 0, 0, 0.5, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.6, 0, 1, 0, 0, 0, 0, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.4, 0, 1, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("littlemac_straight_line"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("littlemac_straight_max"), false, false);
        EFFECT_FOLLOW(agent, Hash40::new("littlemac_straight_aura"), Hash40::new("handr"), 0, 0, 0, 0, 90, 0, 1, true);
        LANDING_EFFECT(agent, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 7, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 36.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("littlemac_straight_aura"), false, false);
    }
}

//Dash Attack Sound
unsafe extern "C" fn ssbexo_littlemac_dash_attack_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_littlemac_special_n03"));
        PLAY_SE(agent, Hash40::new("se_littlemac_special_n02_l"));
    }
}

//Dash Attack Expression
unsafe extern "C" fn ssbexo_littlemac_dash_attack_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_49_dashbrow"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 36.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("littlemac")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attack11", ssbexo_littlemac_jab_1_acmd, Low)
    .game_acmd("game_attack12", ssbexo_littlemac_jab_2_acmd, Low)
    .game_acmd("game_attack13", ssbexo_littlemac_jab_3_acmd, Low)
    .effect_acmd("effect_attack13", ssbexo_littlemac_jab_3_effect, Low)
    .game_acmd("game_attackdash", ssbexo_littlemac_dash_attack_acmd, Low)
    .effect_acmd("effect_attackdash", ssbexo_littlemac_dash_attack_effect, Low)
    .sound_acmd("sound_attackdash", ssbexo_littlemac_dash_attack_sound, Low)
    .expression_acmd("expression_attackdash", ssbexo_littlemac_dash_attack_expression, Low)
    .install()
    ;
}