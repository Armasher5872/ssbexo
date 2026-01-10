use super::*;

//Nair ACMD
unsafe extern "C" fn ssbexo_wario_nair_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hip"), 6.5, 51, 100, 0, 30, 8.2, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hip"), 3.8, 71, 135, 0, 17, 7.0, 4.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 45.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Nair Effect
unsafe extern "C" fn ssbexo_wario_nair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 360, true, 1);
    }
}

//Nair Sound
unsafe extern "C" fn ssbexo_wario_nair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_swing_s"));
        let rand = sv_math::randf(hash40("fighter"), 100.0);
        if rand > 80.0 {
            PLAY_SE(agent, Hash40::new("vc_wario_attack01"));
        }
        else if rand <= 80.0 && rand > 60.0 {
            PLAY_SE(agent, Hash40::new("vc_wario_attack02"));
        }
        else if rand <= 60.0 && rand > 40.0 {
            PLAY_SE(agent, Hash40::new("vc_wario_attack03"));
        }
        else if rand <= 40.0 && rand > 20.0 {
            PLAY_SE(agent, Hash40::new("vc_wario_attack04"));
        }
    }
}

//Nair Expression
unsafe extern "C" fn ssbexo_wario_nair_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Fair ACMD
unsafe extern "C" fn ssbexo_wario_fair_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_XLU);
        ATTACK(agent, 0, 0, Hash40::new("handr"), 5.7, 55, 75, 0, 28, 5.7, 1.5, 0.0, -3.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 5.7, 55, 75, 0, 28, 4.3, 0.0, 0.0, -3.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("shoulderr"), 5.7, 55, 75, 0, 28, 3.0, 0.0, 0.0, -3.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("handr"), 5.7, 367, 75, 0, 28, 5.7, 1.5, 0.0, 0.6, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 4, 0, Hash40::new("armr"), 5.7, 367, 75, 0, 28, 4.3, 0.0, 0.0, -3.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 5, 0, Hash40::new("shoulderr"), 5.7, 367, 75, 0, 28, 3.0, 0.0, 0.0, -3.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("handl"), 6.2, 71, 68, 0, 58, 7.0, 0.0, 0.0, 0.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 6.2, 71, 68, 0, 58, 5.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("shoulderl"), 6.2, 71, 68, 0, 58, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 43.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
//Fair Effect
unsafe extern "C" fn ssbexo_wario_fair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), -3, 6, -11, 0, 10, 0, 1.5, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 10.0);
	if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.0, true, 0.9);
        LAST_EFFECT_SET_RATE(agent, 2.0);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 6, 5, 180, 190, -90, 0.6, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 24.0);
	if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true, 0.9);
        LAST_EFFECT_SET_RATE(agent, 2.0);
    }
}

//Fair Sound
unsafe extern "C" fn ssbexo_wario_fair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_l"));
        let rand = sv_math::randf(hash40("fighter"), 100.0);
        if rand > 75.0 {
            PLAY_SE(agent, Hash40::new("vc_wario_attack01"));
        }
        else if rand <= 75.0 && rand > 50.0 {
            PLAY_SE(agent, Hash40::new("vc_wario_attack02"));
        }
        else if rand <= 50.0 && rand > 25.0 {
            PLAY_SE(agent, Hash40::new("vc_wario_attack03"));
        }
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_l"));
        let rand = sv_math::randf(hash40("fighter"), 100.0);
        if rand > 66.0 {
            PLAY_SE(agent, Hash40::new("vc_wario_attack03"));
        }
        else if rand <= 66.0 && rand > 33.0 {
            PLAY_SE(agent, Hash40::new("vc_wario_007"));
        }
        else {
            let final_sfx = SoundModule::play_se(agent.module_accessor, Hash40::new("vc_wario_final05"), true, false, false, false, smash::app::enSEType(0));
            SoundModule::set_se_vol(agent.module_accessor, final_sfx as i32, 0.75, 0);
        }
    }
}

//Fair Expression
unsafe extern "C" fn ssbexo_wario_fair_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

//Bair ACMD
unsafe extern "C" fn ssbexo_wario_bair_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(agent, 0, 0, Hash40::new("hip"), 12.7, 43, 87, 0, 46, 7.6, 0.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hip"), 8.0, 65, 55, 0, 40, 7.6, 0.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    frame(agent.lua_state_agent, 34.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 45.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Bair Effect
unsafe extern "C" fn ssbexo_wario_bair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 5, -7, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 360, true, 1.5);
    }
}

//Bair Sound
unsafe extern "C" fn ssbexo_wario_bair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_swing_m"));
        let rand = sv_math::randf(hash40("fighter"), 100.0);
        if rand > 66.0 {
            PLAY_SE(agent, Hash40::new("vc_wario_attack01"));
        }
        else if rand <= 66.0 && rand > 33.0 {
            PLAY_SE(agent, Hash40::new("vc_wario_attack02"));
        }
        else {
            PLAY_SE(agent, Hash40::new("vc_wario_attack04"));
        }
    }
}

//Bair Expression
unsafe extern "C" fn ssbexo_wario_bair_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

//Uair ACMD
unsafe extern "C" fn ssbexo_wario_uair_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("handl"), *HIT_STATUS_XLU);
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 88, 78, 0, 30, 9.0, 0.0, 16.0, -0.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("handl"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 37.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Dair ACMD
unsafe extern "C" fn ssbexo_wario_dair_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("handl"), 14.5, 270, 108, 0, 17, 5.7, 0.0, 0.0, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 14.5, 270, 108, 0, 17, 4.3, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 14.5, 59, 85, 0, 28, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 65.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Dair Effect
unsafe extern "C" fn ssbexo_wario_dair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, -8, 2, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 360, true, 1.5);
    }
}

//Dair Sound
unsafe extern "C" fn ssbexo_wario_dair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_l"));
        let rand = sv_math::randf(hash40("fighter"), 100.0);
        if rand > 90.0 {
            PLAY_SE(agent, Hash40::new("vc_wario_attack01"));
        }
        else if rand <= 90.0 && rand > 76.0 {
            PLAY_SE(agent, Hash40::new("vc_wario_attack02"));
        }
        else if rand <= 76.0 && rand > 55.0 {
            PLAY_SE(agent, Hash40::new("vc_wario_attack03"));
        }
        else if rand <= 55.0 && rand > 42.0 {
            PLAY_SE(agent, Hash40::new("vc_wario_attack04"));
        }
        else if rand <= 42.0 && rand > 30.0 {
            PLAY_SE(agent, Hash40::new("vc_wario_007"));
        }
        else if rand <= 30.0 && rand > 16.0 {
            let final_sfx = SoundModule::play_se(agent.module_accessor, Hash40::new("vc_wario_final04"), true, false, false, false, smash::app::enSEType(0));
            SoundModule::set_se_vol(agent.module_accessor, final_sfx as i32, 0.75, 0);
        }
    }
}

//Dair Expression
unsafe extern "C" fn ssbexo_wario_dair_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

//Landing Dair Effect
unsafe extern "C" fn ssbexo_wario_dair_landing_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attackairn", ssbexo_wario_nair_acmd, Low)
    .effect_acmd("effect_attackairn", ssbexo_wario_nair_effect, Low)
    .sound_acmd("sound_attackairn", ssbexo_wario_nair_sound, Low)
    .expression_acmd("expression_attackairn", ssbexo_wario_nair_expression, Low)
    .game_acmd("game_attackairf", ssbexo_wario_fair_acmd, Low)
    .effect_acmd("effect_attackairf", ssbexo_wario_fair_effect, Low)
    .sound_acmd("sound_attackairf", ssbexo_wario_fair_sound, Low)
    .expression_acmd("expression_attackairf", ssbexo_wario_fair_expression, Low)
    .game_acmd("game_attackairb", ssbexo_wario_bair_acmd, Low)
    .effect_acmd("effect_attackairb", ssbexo_wario_bair_effect, Low)
    .sound_acmd("sound_attackairb", ssbexo_wario_bair_sound, Low)
    .expression_acmd("expression_attackairb", ssbexo_wario_bair_expression, Low)
    .game_acmd("game_attackairhi", ssbexo_wario_uair_acmd, Low)
    .game_acmd("game_attackairlw", ssbexo_wario_dair_acmd, Low)
    .effect_acmd("effect_attackairlw", ssbexo_wario_dair_effect, Low)
    .sound_acmd("sound_attackairlw", ssbexo_wario_dair_sound, Low)
    .expression_acmd("expression_attackairlw", ssbexo_wario_dair_expression, Low)
    .effect_acmd("effect_landingairlw", ssbexo_wario_dair_landing_effect, Low)
    .install()
    ;
}