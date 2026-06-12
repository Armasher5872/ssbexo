use super::*;

//Jab 1 ACMD
unsafe extern "C" fn ssbexo_simon_jab_1_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 20, 0, 20, 3.0, 0.0, 4.5, 7.5, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 20, 0, 20, 3.2, 0.0, 4.5, 9.5, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 361, 20, 0, 15, 3.4, 0.0, 4.5, 12.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 180, 20, 0, 20, 3.6, 0.0, 4.5, 14.5, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 361, 20, 0, 20, 3.6, 0.0, 4.5, 14.5, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        AttackModule::set_add_reaction_frame(boma, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 2.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 2.0, false);
        AttackModule::set_add_reaction_frame(boma, 3, 2.0, false);
        AttackModule::set_add_reaction_frame(boma, 4, 2.0, false);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_ENABLE_HOLD);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

//Whip Dangle ACMD
unsafe extern "C" fn ssbexo_simon_whip_dangle_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("handr"), 8.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, false, 1.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
}

//Rapid Jab ACMD
unsafe extern "C" fn ssbexo_simon_rapid_jab_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    loop {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 361, 10, 0, 8, 7.3, 0.0, 7.5, 10.0, Some(0.0), Some(7.5), Some(13.0), 0.5, 0.35, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
            AttackModule::set_add_reaction_frame(boma, 0, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 5);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        if is_excute(agent) {
            FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.0, 5.0);
        }
        frame(lua_state, 2.0);
        ssbexo_simon_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_simon_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_simon_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_simon_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_simon_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_simon_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_simon_rapid_jab_sub_acmd(agent);
        wait(lua_state, 2.0);
        wait_loop_clear(agent);
    }
}

//Rapid Jab Sub ACMD
unsafe extern "C" fn ssbexo_simon_rapid_jab_sub_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 361, 10, 0, 8, 7.3, 0.0, 7.5, 10.0, Some(0.0), Some(7.5), Some(13.0), 0.5, 0.35, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        AttackModule::set_add_reaction_frame(boma, 0, 3.0, false);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 5);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_simon_dash_attack_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 30, 80, 0, 60, 10.5, 0.0, 8.8, 10.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 0, false);
        ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 65, 60, 0, 60, 9.5, 0.0, 8.8, 0.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 90, 60, 0, 40, 10.5, 0.0, 8.8, -4.0, Some(0.0), Some(8.8), Some(-15.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 0, false);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//Dash Attack Effect
unsafe extern "C" fn ssbexo_simon_dash_attack_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Dash Attack Sound
unsafe extern "C" fn ssbexo_simon_dash_attack_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_simon_whip_holding"));
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_simon_jump01"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_simon_rnd_jump"));
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_simon_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_simon_smash_s02"));
    }
}

//Dash Attack Expression
unsafe extern "C" fn ssbexo_simon_dash_attack_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_jump"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

//Whip Dash Attack ACMD
unsafe extern "C" fn ssbexo_simon_whip_dash_attack_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_MOVE);
        agent.clear_lua_stack();
        let object = sv_system::battle_object(lua_state) as *mut BattleObject;
        if !object.is_null() {
            WeaponSpecializer_SimonWhip::reset_node_fix_flag_list(object as *mut smash::app::Weapon);
            WeaponSpecializer_SimonWhip::set_node_fix_flag_list(object as *mut smash::app::Weapon, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, -1, -1, -1, -1, -1, -1, -1, -1);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_MOVE);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_COLLIDE);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
        agent.clear_lua_stack();
        let object = sv_system::battle_object(lua_state) as *mut BattleObject;
        if !object.is_null() {
            WeaponSpecializer_SimonWhip::reset_node_fix_flag_list(object as *mut smash::app::Weapon);
        }
    }
}

//Whip Dash Attack Effect
unsafe extern "C" fn ssbexo_simon_whip_dash_attack_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 11.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        let object = sv_system::battle_object(lua_state) as *mut BattleObject;
        if !object.is_null() {
            WeaponSpecializer_SimonWhip::set_chain_2_visibility(object as *mut smash::app::Weapon, true);
        }
        EFFECT_FOLLOW(agent, Hash40::new("simon_whip_light"), Hash40::new("hookshot6"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.3);
        EFFECT_FOLLOW(agent, Hash40::new("simon_whip_flash_top"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        let object = sv_system::battle_object(lua_state) as *mut BattleObject;
        if !object.is_null() {
            WeaponSpecializer_SimonWhip::set_chain_2_visibility(object as *mut smash::app::Weapon, true);
        }
        EFFECT_FOLLOW(agent, Hash40::new("simon_whip_light"), Hash40::new("hookshot6"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 1.3);
        EFFECT_FOLLOW(agent, Hash40::new("simon_whip_flash_top"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        let object = sv_system::battle_object(lua_state) as *mut BattleObject;
        if !object.is_null() {
            WeaponSpecializer_SimonWhip::set_chain_2_visibility(object as *mut smash::app::Weapon, false);
        }
    }
}

pub fn install() {
    Agent::new("simon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_attack11", ssbexo_simon_jab_1_acmd, Low)
    .acmd("game_attackhold", ssbexo_simon_whip_dangle_acmd, Low)
    .acmd("game_attack100", ssbexo_simon_rapid_jab_acmd, Low)
    .acmd("game_attackdash", ssbexo_simon_dash_attack_acmd, Low)
    .acmd("effect_attackdash", ssbexo_simon_dash_attack_effect, Low)
    .acmd("sound_attackdash", ssbexo_simon_dash_attack_sound, Low)
    .acmd("expression_attackdash", ssbexo_simon_dash_attack_expression, Low)
    .install()
    ;
    Agent::new("simon_whip")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_attackdash", ssbexo_simon_whip_dash_attack_acmd, Low)
    .acmd("effect_attackdash", ssbexo_simon_whip_dash_attack_effect, Low)
    .install()
    ;
}