use super::*;

//Nair ACMD
unsafe extern "C" fn ssbexo_donkey_nair_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.433);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(agent, 0, 0, Hash40::new("armr"), 12.0, 361, 100, 0, 20, 5.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 12.0, 361, 100, 0, 20, 5.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("hip"), 12.0, 361, 100, 0, 20, 5.7, 1.6, 3.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 3, 0, Hash40::new("top"), 12.0, 361, 100, 0, 20, 5.0, 0.0, 8.0, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 3, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("armr"), 9.0, 361, 100, 0, 10, 5.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 9.0, 361, 100, 0, 10, 5.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("hip"), 9.0, 361, 100, 0, 10, 5.7, 1.6, 3.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 27.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Uair ACMD
unsafe extern "C" fn ssbexo_donkey_uair_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(agent, 0, 0, Hash40::new("head"), 13.0, 90, 90, 0, 32, 7.5, 2.5, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        ATK_SET_SHIELD_SETOFF_MUL2(agent, 0, 0.1);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Dair ACMD
unsafe extern "C" fn ssbexo_donkey_dair_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 270, 10, 0, 90, 7.0, 0.0, -5.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 361, 90, 0, 20, 5.8, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Zair ACMD
unsafe extern "C" fn ssbexo_donkey_zair_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        SEARCH(agent, 0, 0, Hash40::new("top"), 7.0, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(16.0), *COLLISION_KIND_MASK_ALL, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 0.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        CATCH(agent, 0, Hash40::new("top"), 7.0, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(16.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        grab!(agent, MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
}

//Zair Effect
unsafe extern "C" fn ssbexo_donkey_zair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("donkey_smash_slap"), Hash40::new("donkey_smash_slap"), Hash40::new("top"), 3, 15, 2.5, -53, -69, -99, 1.9, true, *EF_FLIP_YZ, 1);
        LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 22, 8, 0, 0, 0, 0, 2.3, 0, 0, 0, 0, 0, 360, false);
    }
}

//Zair Sound
unsafe extern "C" fn ssbexo_donkey_zair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_swing_10"));
    }
    wait(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_swing_10"));
    }
}

//Zair Expression
unsafe extern "C" fn ssbexo_donkey_zair_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_nohits"), 0);
    }
}

pub fn install() {
    Agent::new("donkey")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attackairn", ssbexo_donkey_nair_acmd, Low)
    .game_acmd("game_attackairhi", ssbexo_donkey_uair_acmd, Low)
    .game_acmd("game_attackairlw", ssbexo_donkey_dair_acmd, Low)
    .game_acmd("game_aircatch", ssbexo_donkey_zair_acmd, Low)
    .effect_acmd("effect_aircatch", ssbexo_donkey_zair_effect, Low)
    .sound_acmd("sound_aircatch", ssbexo_donkey_zair_sound, Low)
    .expression_acmd("expression_aircatch", ssbexo_donkey_zair_expression, Low)
    .install()
    ;
}