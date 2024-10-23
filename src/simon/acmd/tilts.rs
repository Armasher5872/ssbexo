use super::*;

//Up Tilt ACMD
unsafe extern "C" fn ssbexo_simon_up_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 3.0, 90, 57, 0, 88, 5.0, 0.0, 3.5, 3.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 3.0, 90, 57, 0, 88, 4.0, 0.0, 3.5, -4.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 6, 0, Hash40::new("top"), 3.0, 90, 57, 0, 88, 4.0, 0.0, 3.5, 10.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 4, false);
        AttackModule::clear(agent.module_accessor, 5, false);
        AttackModule::clear(agent.module_accessor, 6, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 93, 57, 0, 88, 5.0, 0.0, 25.5, 3.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 93, 57, 0, 88, 4.0, 0.0, 25.5, -4.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 93, 57, 0, 88, 4.0, 0.0, 25.5, 10.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 93, 82, 0, 88, 4.5, 0.0, 17.5, 3.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Tilt ACMD
unsafe extern "C" fn ssbexo_simon_down_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::SEARCH(agent, 0, 0, Hash40::new("top"), 2.5, 0.0, 5.0, 15.0, Some(0.0), Some(5.0), Some(34.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_IG, *COLLISION_PART_MASK_ALL, false);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        let object = sv_system::battle_object(agent.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            FighterSpecializer_Simon::set_whip_reflect_attack_off_id(object as *mut Fighter, 0, 1, -1, -1, -1, -1, -1, -1, -1, -1);
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 140, 45, 0, 65, 2.5, 0.0, 5.0, 34.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 70, 45, 0, 65, 2.5, 0.0, 5.0, 7.0, Some(0.0), Some(10.0), Some(34.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 361, 65, 0, 65, 4.5, 0.0, 5.0, 7.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Down Tilt Effect
unsafe extern "C" fn ssbexo_simon_down_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("simon_whip_straight"), Hash40::new("haver"), 0, 0, 0, 4, 30, 4, 0.98, true);
    }
}

//Down Tilt Sound
unsafe extern "C" fn ssbexo_simon_down_tilt_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_simon_whip_holding"));
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_simon_attackhard_s01"));
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_simon_rnd_attack"));
    }
}

//Down Tilt Expression
unsafe extern "C" fn ssbexo_simon_down_tilt_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

//Whip Down Tilt ACMD
unsafe extern "C" fn ssbexo_simon_whip_down_tilt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        PhysicsModule::set_2nd_status(agent.module_accessor, *PH2NDARY_CRAW_NONE);
        agent.clear_lua_stack();
        let object = sv_system::battle_object(agent.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            WeaponSpecializer_SimonWhip::reset_node_fix_flag_list(object as *mut smash::app::Weapon);
        }
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        PhysicsModule::set_2nd_status(agent.module_accessor, *PH2NDARY_CRAW_COLLIDE);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        PhysicsModule::set_2nd_status(agent.module_accessor, *PH2NDARY_CRAW_MOVE);
    }
}

//Whip Down Tilt Effect
unsafe extern "C" fn ssbexo_simon_whip_down_tilt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        let object = sv_system::battle_object(agent.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            WeaponSpecializer_SimonWhip::set_chain_2_visibility(object as *mut smash::app::Weapon, true);
        }
        macros::EFFECT_FOLLOW(agent, Hash40::new("simon_whip_light"), Hash40::new("hookshot6"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
        macros::EFFECT_FOLLOW(agent, Hash40::new("simon_whip_flash_top"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        let object = sv_system::battle_object(agent.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            WeaponSpecializer_SimonWhip::set_chain_2_visibility(object as *mut smash::app::Weapon, false);
        }
    }
}

pub fn install() {
    Agent::new("simon")
    .game_acmd("game_attackhi3", ssbexo_simon_up_tilt_acmd, Priority::Low)
    .game_acmd("game_attacklw3", ssbexo_simon_down_tilt_acmd, Priority::Low)
    .effect_acmd("effect_attacklw3", ssbexo_simon_down_tilt_effect, Priority::Low)
    .sound_acmd("sound_attacklw3", ssbexo_simon_down_tilt_sound, Priority::Low)
    .expression_acmd("expression_attacklw3", ssbexo_simon_down_tilt_expression, Priority::Low)
    .install()
    ;
    Agent::new("simon_whip")
    .game_acmd("game_attacklw3", ssbexo_simon_whip_down_tilt_acmd, Priority::Low)
    .effect_acmd("effect_attacklw3", ssbexo_simon_whip_down_tilt_effect, Priority::Low)
    .install()
    ;
}