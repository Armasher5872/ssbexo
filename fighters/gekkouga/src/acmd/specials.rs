use super::*;

//Side Special ACMD
unsafe extern "C" fn ssbexo_gekkouga_side_special_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        SEARCH(agent, 0, 0, Hash40::new("bust"), 75.0, 0.0, 0.0, 0.0, None, None, None, *COLLISION_KIND_MASK_ALL, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_FLAG_RESET_GRAVITY);
    }
}

//Side Special Attack Hi ACMD
unsafe extern "C" fn ssbexo_gekkouga_side_special_attack_hi_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legl"), 13.0, 90, 75, 0, 50, 8.5, 10.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("legl"), 13.0, 90, 75, 0, 50, 7.0, 2.5, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Side Special Attack Hi Effect
unsafe extern "C" fn ssbexo_gekkouga_side_special_attack_hi_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        FLASH(agent, 1, 1, 1, 0);
        EFFECT(agent, Hash40::new("gekkouga_kageuchi_warp_end"), Hash40::new("top"), 0, 7, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("gekkouga_migawari_appearance"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("gekkouga_pump_line"), Hash40::new("top"), 0, 10, 17, 0, 180, 0, 1, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

//Side Special Attack Hi Sound
unsafe extern "C" fn ssbexo_gekkouga_side_special_attack_hi_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_gekkouga_004"));
        PLAY_SE(agent, Hash40::new("se_gekkouga_special_l03"));
    }
}

//Side Special Attack Hi Expression
unsafe extern "C" fn ssbexo_gekkouga_side_special_attack_hi_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, true);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

//Side Special Attack Lw ACMD
unsafe extern "C" fn ssbexo_gekkouga_side_special_attack_lw_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legl"), 13.0, 270, 65, 0, 25, 8.5, 10.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("legl"), 13.0, 45, 95, 0, 50, 8.5, 10.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Side Special Attack Lw Effect
unsafe extern "C" fn ssbexo_gekkouga_side_special_attack_lw_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        FLASH(agent, 1, 1, 1, 0);
        EFFECT(agent, Hash40::new("gekkouga_kageuchi_warp_end"), Hash40::new("top"), 0, 7, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("gekkouga_migawari_appearance"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("gekkouga_pump_line"), Hash40::new("top"), 0, 10, 17, 0, 180, 0, 1, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

//Side Special Attack Lw Sound
unsafe extern "C" fn ssbexo_gekkouga_side_special_attack_lw_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_gekkouga_004"));
        PLAY_SE(agent, Hash40::new("se_gekkouga_special_l03"));
    }
}

//Side Special Attack Lw Expression
unsafe extern "C" fn ssbexo_gekkouga_side_special_attack_lw_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, true);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

//Down Special ACMD
unsafe extern "C" fn ssbexo_gekkouga_down_special_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x26b93cec86));
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_DOLL_LINK);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_DOLL_LAUNCH);
    }
}

//Down Special Effect
unsafe extern "C" fn ssbexo_gekkouga_down_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        EFFECT_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -4, 4, -3, -12, 17, 0, 1.1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LAST_PARTICLE_SET_COLOR(agent, 0.5, 1.4, 1.6);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 10, 15, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}

//Down Special Sound
unsafe extern "C" fn ssbexo_gekkouga_down_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_gekkouga_special_l01"));
    }
}

//Down Special Expression
unsafe extern "C" fn ssbexo_gekkouga_down_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

pub fn install() {
    Agent::new("gekkouga")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specials", ssbexo_gekkouga_side_special_acmd, Low)
    .game_acmd("game_specialsattackhi", ssbexo_gekkouga_side_special_attack_hi_acmd, Low)
    .game_acmd("game_specialairsattackhi", ssbexo_gekkouga_side_special_attack_hi_acmd, Low)
    .effect_acmd("effect_specialsattackhi", ssbexo_gekkouga_side_special_attack_hi_effect, Low)
    .effect_acmd("effect_specialairsattackhi", ssbexo_gekkouga_side_special_attack_hi_effect, Low)
    .sound_acmd("sound_specialsattackhi", ssbexo_gekkouga_side_special_attack_hi_sound, Low)
    .sound_acmd("sound_specialairsattackhi", ssbexo_gekkouga_side_special_attack_hi_sound, Low)
    .expression_acmd("expression_specialsattackhi", ssbexo_gekkouga_side_special_attack_hi_expression, Low)
    .expression_acmd("expression_specialairsattackhi", ssbexo_gekkouga_side_special_attack_hi_expression, Low)
    .game_acmd("game_specialsattacklw", ssbexo_gekkouga_side_special_attack_lw_acmd, Low)
    .game_acmd("game_specialairsattacklw", ssbexo_gekkouga_side_special_attack_lw_acmd, Low)
    .effect_acmd("effect_specialsattacklw", ssbexo_gekkouga_side_special_attack_lw_effect, Low)
    .effect_acmd("effect_specialairsattacklw", ssbexo_gekkouga_side_special_attack_lw_effect, Low)
    .sound_acmd("sound_specialsattacklw", ssbexo_gekkouga_side_special_attack_lw_sound, Low)
    .sound_acmd("sound_specialairsattacklw", ssbexo_gekkouga_side_special_attack_lw_sound, Low)
    .expression_acmd("expression_specialsattacklw", ssbexo_gekkouga_side_special_attack_lw_expression, Low)
    .expression_acmd("expression_specialairsattacklw", ssbexo_gekkouga_side_special_attack_lw_expression, Low)
    .game_acmd("game_speciallw", ssbexo_gekkouga_down_special_acmd, Low)
    .game_acmd("game_specialairlw", ssbexo_gekkouga_down_special_acmd, Low)
    .effect_acmd("effect_speciallw", ssbexo_gekkouga_down_special_effect, Low)
    .effect_acmd("effect_specialairlw", ssbexo_gekkouga_down_special_effect, Low)
    .sound_acmd("sound_speciallw", ssbexo_gekkouga_down_special_sound, Low)
    .sound_acmd("sound_specialairlw", ssbexo_gekkouga_down_special_sound, Low)
    .expression_acmd("expression_speciallw", ssbexo_gekkouga_down_special_expression, Low)
    .expression_acmd("expression_specialairlw", ssbexo_gekkouga_down_special_expression, Low)
    .install()
    ;
}