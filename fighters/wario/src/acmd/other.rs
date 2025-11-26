use super::*;

//Slow Walk Effect
unsafe extern "C" fn ssbexo_wario_slow_walk_effect(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 40.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 70.0);
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

//Slow Walk Sound
unsafe extern "C" fn ssbexo_wario_slow_walk_sound(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 40.0);
        if is_excute(agent) {
            PLAY_STEP(agent, Hash40::new("se_wario_step_right_s"));
        }
        frame(agent.lua_state_agent, 70.0);
        PLAY_STEP(agent, Hash40::new("se_wario_step_left_s"));
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

//Slow Walk Expression
unsafe extern "C" fn ssbexo_wario_slow_walk_expression(agent: &mut L2CAgentBase) {
    loop {
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
        }
        frame(agent.lua_state_agent, 40.0);
        if is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 70.0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

//Fast Walk Effect
unsafe extern "C" fn ssbexo_wario_fast_walk_effect(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 6.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 24.0);
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

//Fast Walk Sound
unsafe extern "C" fn ssbexo_wario_fast_walk_sound(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 6.0);
        if is_excute(agent) {
            PLAY_STEP(agent, Hash40::new("se_wario_step_right_s"));
        }
        frame(agent.lua_state_agent, 24.0);
        PLAY_STEP(agent, Hash40::new("se_wario_step_left_s"));
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

//Fast Walk Expression
unsafe extern "C" fn ssbexo_wario_fast_walk_expression(agent: &mut L2CAgentBase) {
    loop {
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
        }
        frame(agent.lua_state_agent, 6.0);
        if is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 24.0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

//Up Taunt Effect
unsafe extern "C" fn ssbexo_wario_up_taunt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 71.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("wario_kamitsuki_end"), Hash40::new("top"), 0, 10, -2, 180, 0, 0, 0.7, false);
    }
}

//Up Taunt Sound
unsafe extern "C" fn ssbexo_wario_up_taunt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("vc_wario_appeal01"));
    }
    frame(agent.lua_state_agent, 65.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_appeal_h01"));
    }
}

//Down Taunt Effect
unsafe extern "C" fn ssbexo_wario_down_taunt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("wario_kamitsuki_end"), Hash40::new("top"), 4, 8.5, 8, -90, 0, 0, 0.7, false);
        EFFECT_FOLLOW(agent, Hash40::new("wario_kamitsuki_end"), Hash40::new("top"), 0, 6.5, 0, -90, 0, 0, 0.7, false);
    }
}

//Down Taunt Sound
unsafe extern "C" fn ssbexo_wario_down_taunt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_wario_011"));
    }
}

//Toot Taunt ACMD
unsafe extern "C" fn ssbexo_wario_toot_taunt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 1, 0, 0, 9.0, 0.0, 4.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 1.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_slip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Toot Taunt Effect
unsafe extern "C" fn ssbexo_wario_toot_taunt_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 13.0);
    if sv_animcmd::get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wario_ppe_s"), Hash40::new("top"), 0, 0, 0, 0, 100, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wario_ppe_s"), Hash40::new("top"), 0, 0, 0, 0, -100, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

//Toot Taunt Sound
unsafe extern "C" fn ssbexo_wario_toot_taunt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_special_l01"));
    }
    wait(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_wario_012"));
    }
}

//Toot Taunt Expression
unsafe extern "C" fn ssbexo_wario_toot_taunt_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attacks"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        agent.clear_lua_stack();
        lua_args!(agent, 1, 0.8, 0.02, 1000, 1, 0, 4, 14);
        sv_animcmd::AREA_WIND_2ND_RAD(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 1);
    }
    frame(agent.lua_state_agent, 43.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
    }
}

//Win 1 ACMD
unsafe extern "C" fn ssbexo_wario_win_1_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_GARLIC, true, -1);
        if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_GARLIC) {
            let garlic_boma = get_article_boma(agent.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_GARLIC);
            LinkModule::set_model_constraint_pos_ort(garlic_boma, *LINK_NO_CONSTRAINT, Hash40::new("have"), Hash40::new("haver"), (*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE | *CONSTRAINT_FLAG_OFFSET_ROT) as u32, true);
        }
    }
    frame(agent.lua_state_agent, 60.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_GARLIC, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Win 1 Effect
unsafe extern "C" fn ssbexo_wario_win_1_effect(_agent: &mut L2CAgentBase) {}

//Win 1 Sound
unsafe extern "C" fn ssbexo_wario_win_1_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("vc_wario_007"));
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("vc_wario_005"));
    }
    frame(agent.lua_state_agent, 115.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("vc_wario_final05"));
    }
}

//Win 1 Expression
unsafe extern "C" fn ssbexo_wario_win_1_expression(_agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .effect_acmd("effect_walkslow", ssbexo_wario_slow_walk_effect, Low)
    .sound_acmd("sound_walkslow", ssbexo_wario_slow_walk_sound, Low)
    .expression_acmd("expression_walkslow", ssbexo_wario_slow_walk_expression, Low)
    .effect_acmd("effect_walkfast", ssbexo_wario_fast_walk_effect, Low)
    .sound_acmd("sound_walkfast", ssbexo_wario_fast_walk_sound, Low)
    .expression_acmd("expression_walkfast", ssbexo_wario_fast_walk_expression, Low)
    .effect_acmd("effect_appealhil", ssbexo_wario_up_taunt_effect, Low)
    .sound_acmd("sound_appealhil", ssbexo_wario_up_taunt_sound, Low)
    .effect_acmd("effect_appealhir", ssbexo_wario_up_taunt_effect, Low)
    .sound_acmd("sound_appealhir", ssbexo_wario_up_taunt_sound, Low)
    .effect_acmd("effect_appeallwl", ssbexo_wario_down_taunt_effect, Low)
    .sound_acmd("sound_appeallwl", ssbexo_wario_down_taunt_sound, Low)
    .effect_acmd("effect_appeallwr", ssbexo_wario_down_taunt_effect, Low)
    .sound_acmd("sound_appeallwr", ssbexo_wario_down_taunt_sound, Low)
    .game_acmd("game_appealgas", ssbexo_wario_toot_taunt_acmd, Low)
    .effect_acmd("effect_appealgas", ssbexo_wario_toot_taunt_effect, Low)
    .sound_acmd("sound_appealgas", ssbexo_wario_toot_taunt_sound, Low)
    .expression_acmd("expression_appealgas", ssbexo_wario_toot_taunt_expression, Low)
    .game_acmd("game_win1", ssbexo_wario_win_1_acmd, Low)
    .effect_acmd("effect_win1", ssbexo_wario_win_1_effect, Low)
    .sound_acmd("sound_win1", ssbexo_wario_win_1_sound, Low)
    .expression_acmd("expression_win1", ssbexo_wario_win_1_expression, Low)
    .install()
    ;
}