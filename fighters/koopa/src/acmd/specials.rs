use super::*;

//Neutral Special ACMD
unsafe extern "C" fn ssbexo_koopa_neutral_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KOOPA_GENERATE_ARTICLE_BREATH, false, -1);
    }
}

//Grounded Neutral Special Effect
unsafe extern "C" fn ssbexo_koopa_grounded_neutral_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        FLASH(agent, 0.961, 0.569, 0.569, 0.392);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0.537, 0.537, 0.588);
        FLASH_FRM(agent, 20, 0, 0, 0, 0);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Neutral Special Effect
unsafe extern "C" fn ssbexo_koopa_aerial_neutral_special_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        FLASH(agent, 0.961, 0.569, 0.569, 0.392);
    }
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0.537, 0.537, 0.588);
        FLASH_FRM(agent, 20, 0, 0, 0, 0);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

//Grounded Neutral Special Sound
unsafe extern "C" fn ssbexo_koopa_grounded_neutral_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_koopa_special_n01"));
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        PLAY_SE_REMAIN(agent, Hash40::new("se_koopa_step_left_m"));
    }
}

//Aerial Neutral Special Sound
unsafe extern "C" fn ssbexo_koopa_aerial_neutral_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_koopa_special_n01"));
    }
}

//Grounded Neutral Special Expression
unsafe extern "C" fn ssbexo_koopa_grounded_neutral_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Aerial Neutral Special Expression
unsafe extern "C" fn ssbexo_koopa_aerial_neutral_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Fire Breath Move ACMD
unsafe extern "C" fn ssbexo_koopa_firebreath_move_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 361, 40, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}

//Fire Breath Move Effect
unsafe extern "C" fn ssbexo_koopa_firebreath_move_effect(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, true);
            EFFECT_FOLLOW(agent, Hash40::new("koopa_breath_m_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
        }
        wait(agent.lua_state_agent, 15.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, true);
        }
        wait(agent.lua_state_agent, 15.0);
    }
}

//Fire Breath Move Sound
unsafe extern "C" fn ssbexo_koopa_firebreath_move_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_koopa_special_n02"));
    }
}

//Flying Slam Landing ACMD
unsafe extern "C" fn ssbexo_koopa_flying_slam_landing_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 18.0, 65, 60, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 3, 9);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.3);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: -9.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_HIT);
    }
}

//Grounded Bowser Bomb ACMD
unsafe extern "C" fn ssbexo_koopa_grounded_bowser_bomb_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 80, 100, 60, 0, 4.0, 0.0, 1.0, 17.0, Some(0.0), Some(9.0), Some(17.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 100, 100, 60, 0, 2.0, 0.0, 0.5, 22.5, Some(0.0), Some(9.5), Some(22.5), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 90, 100, 60, 0, 4.0, 0.0, 2.5, 13.0, Some(0.0), Some(7.5), Some(13.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 8.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 8.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 8.0, false);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_LW_FLAG1);
    }
    frame(agent.lua_state_agent, 37.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 290, 65, 0, 13, 8.3, 0.0, 5.2, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
}

//Aerial Bowser Bomb ACMD
unsafe extern "C" fn ssbexo_koopa_aerial_bowser_bomb_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.29);
    }
    frame(agent.lua_state_agent, 31.0);
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_LW_FLAG1);
        ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 290, 65, 0, 13, 8.6, 0.0, 5.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 33.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 76, 75, 0, 45, 8.3, 0.0, 5.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
}

//Bowser Bomb Landing ACMD
unsafe extern "C" fn ssbexo_koopa_bowser_bomb_landing_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 76, 72, 0, 60, 8.0, 0.0, 1.5, -10.0, Some(0.0), Some(1.5), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    FT_MOTION_RATE(agent, 0.9);
}

pub fn install() {
    Agent::new("koopa")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialn", ssbexo_koopa_neutral_special_acmd, Low)
    .game_acmd("game_specialairn", ssbexo_koopa_neutral_special_acmd, Low)
    .effect_acmd("effect_specialn", ssbexo_koopa_grounded_neutral_special_effect, Low)
    .effect_acmd("effect_specialairn", ssbexo_koopa_aerial_neutral_special_effect, Low)
    .sound_acmd("sound_specialn", ssbexo_koopa_grounded_neutral_special_sound, Low)
    .sound_acmd("sound_specialairn", ssbexo_koopa_aerial_neutral_special_sound, Low)
    .expression_acmd("expression_specialn", ssbexo_koopa_grounded_neutral_special_expression, Low)
    .expression_acmd("expression_specialairn", ssbexo_koopa_aerial_neutral_special_expression, Low)
    .game_acmd("game_specialslanding", ssbexo_koopa_flying_slam_landing_acmd, Low)
    .game_acmd("game_speciallw", ssbexo_koopa_grounded_bowser_bomb_acmd, Low)
    .game_acmd("game_specialairlw", ssbexo_koopa_aerial_bowser_bomb_acmd, Low)
    .game_acmd("game_speciallwlanding", ssbexo_koopa_bowser_bomb_landing_acmd, Low)
    .install()
    ;
    Agent::new("koopa_breath")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_move", ssbexo_koopa_firebreath_move_acmd, Low)
    .effect_acmd("effect_move", ssbexo_koopa_firebreath_move_effect, Low)
    .sound_acmd("sound_move", ssbexo_koopa_firebreath_move_sound, Low)
    .install()
    ;
}