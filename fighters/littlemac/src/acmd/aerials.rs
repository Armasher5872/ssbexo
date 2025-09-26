use super::*;

//Nair ACMD
unsafe extern "C" fn ssbexo_littlemac_nair_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("arml"), 2.0, 300, 100, 0, 5, 4.0, 3.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 2.0, 300, 100, 0, 5, 2.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 39.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

//Fair ACMD
unsafe extern "C" fn ssbexo_littlemac_fair_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("handr"), 8.0, 25, 45, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 7.0, 25, 45, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("shoulderr"), 6.0, 25, 45, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 31.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Fair Effect
unsafe extern "C" fn ssbexo_littlemac_fair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("littlemac_straight_line"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("littlemac_straight"), Hash40::new("top"), 1.6, 7.2, 13.5, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("littlemac_straight_line"), false, false);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
}

//Fair Sound
unsafe extern "C" fn ssbexo_littlemac_fair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_littlemac_special_n02"));
        PLAY_SE(agent, Hash40::new("se_littlemac_special_n02_s"));
    }
}

//Fair Expression
unsafe extern "C" fn ssbexo_littlemac_fair_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Bair ACMD
unsafe extern "C" fn ssbexo_littlemac_bair_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 160, 80, 0, 30, 5.0, 0.0, 11.0, -2.0, Some(0.0), Some(11.0), Some(-9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Bair Effect
unsafe extern "C" fn ssbexo_littlemac_bair_effect(agent: &mut L2CAgentBase) {
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 8, -1.5, 0, 160, -15, 1.0, true);
        LAST_EFFECT_SET_RATE(agent, 1.8);
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

//Bair Sound
unsafe extern "C" fn ssbexo_littlemac_bair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_littlemac_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_littlemac_attackair_b01"));
    }
}

//Bair Expression
unsafe extern "C" fn ssbexo_littlemac_bair_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

//Uair ACMD
unsafe extern "C" fn ssbexo_littlemac_uair_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 270, 30, 0, 50, 5.0, 0.0, 18.0, 0.0, Some(0.0), Some(22.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Uair Effect
unsafe extern "C" fn ssbexo_littlemac_uair_effect(agent: &mut L2CAgentBase) {
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 8.5, 0, 270, 0, 0, 0.8, false);
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
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 18.5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, false);
    }
}

//Uair Sound
unsafe extern "C" fn ssbexo_littlemac_uair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_littlemac_attackair_h01"));
    }
}

//Uair Expression
unsafe extern "C" fn ssbexo_littlemac_uair_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

//Dair ACMD
unsafe extern "C" fn ssbexo_littlemac_dair_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 5.0, 330, 30, 30, 0, 4.5, 4.4, 0.2, 0.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 5.0, 330, 30, 30, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("shoulderr"), 5.0, 330, 30, 30, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Dair Effect
unsafe extern "C" fn ssbexo_littlemac_dair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 7.5, -3, 25, 0, 0, 0.9, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 2, 11, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_attack_line"), true, true);
    }
}

//Dair Sound
unsafe extern "C" fn ssbexo_littlemac_dair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_littlemac_attackair_l01"));
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_littlemac_rnd_attack"));
    }
}

//Dair Expression
unsafe extern "C" fn ssbexo_littlemac_dair_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

pub fn install() {
    Agent::new("littlemac")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attackairn", ssbexo_littlemac_nair_acmd, Low)
    .game_acmd("game_attackairf", ssbexo_littlemac_fair_acmd, Low)
    .effect_acmd("effect_attackairf", ssbexo_littlemac_fair_effect, Low)
    .sound_acmd("sound_attackairf", ssbexo_littlemac_fair_sound, Low)
    .expression_acmd("expression_attackairf", ssbexo_littlemac_fair_expression, Low)
    .game_acmd("game_attackairb", ssbexo_littlemac_bair_acmd, Low)
    .effect_acmd("effect_attackairb", ssbexo_littlemac_bair_effect, Low)
    .sound_acmd("sound_attackairb", ssbexo_littlemac_bair_sound, Low)
    .expression_acmd("expression_attackairb", ssbexo_littlemac_bair_expression, Low)
    .game_acmd("game_attackairhi", ssbexo_littlemac_uair_acmd, Low)
    .effect_acmd("effect_attackairhi", ssbexo_littlemac_uair_effect, Low)
    .sound_acmd("sound_attackairhi", ssbexo_littlemac_uair_sound, Low)
    .expression_acmd("expression_attackairhi", ssbexo_littlemac_uair_expression, Low)
    .game_acmd("game_attackairlw", ssbexo_littlemac_dair_acmd, Low)
    .effect_acmd("effect_attackairlw", ssbexo_littlemac_dair_effect, Low)
    .sound_acmd("sound_attackairlw", ssbexo_littlemac_dair_sound, Low)
    .expression_acmd("expression_attackairlw", ssbexo_littlemac_dair_expression, Low)
    .install()
    ;
}