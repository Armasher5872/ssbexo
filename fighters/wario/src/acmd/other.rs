use super::*;

//Slow Walk Effect
unsafe extern "C" fn ssbexo_wario_slow_walk_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    loop {
        frame(lua_state, 6.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
        }
        frame(lua_state, 40.0);
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
        wait_loop_clear(agent);
    }
}

//Slow Walk Sound
unsafe extern "C" fn ssbexo_wario_slow_walk_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    loop {
        frame(lua_state, 6.0);
        if is_excute(agent) {
            PLAY_STEP(agent, Hash40::new("se_wario_step_right_s"));
        }
        frame(lua_state, 40.0);
        PLAY_STEP(agent, Hash40::new("se_wario_step_left_s"));
        wait_loop_clear(agent);
    }
}

//Slow Walk Expression
unsafe extern "C" fn ssbexo_wario_slow_walk_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    loop {
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
        }
        frame(lua_state, 6.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 40.0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        wait_loop_clear(agent);
    }
}

//Middle Walk Effect
unsafe extern "C" fn ssbexo_wario_middle_walk_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    loop {
        frame(lua_state, 6.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
        }
        frame(lua_state, 40.0);
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
        wait_loop_clear(agent);
    }
}

//Middle Walk Sound
unsafe extern "C" fn ssbexo_wario_middle_walk_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    loop {
        frame(lua_state, 6.0);
        if is_excute(agent) {
            PLAY_STEP(agent, Hash40::new("se_wario_step_right_s"));
        }
        frame(lua_state, 40.0);
        PLAY_STEP(agent, Hash40::new("se_wario_step_left_s"));
        wait_loop_clear(agent);
    }
}

//Middle Walk Expression
unsafe extern "C" fn ssbexo_wario_middle_walk_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    loop {
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
        }
        frame(lua_state, 6.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 40.0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        wait_loop_clear(agent);
    }
}

//Fast Walk Effect
unsafe extern "C" fn ssbexo_wario_fast_walk_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    loop {
        frame(lua_state, 6.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
        }
        frame(lua_state, 24.0);
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
        wait_loop_clear(agent);
    }
}

//Fast Walk Sound
unsafe extern "C" fn ssbexo_wario_fast_walk_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    loop {
        frame(lua_state, 6.0);
        if is_excute(agent) {
            PLAY_STEP(agent, Hash40::new("se_wario_step_right_s"));
        }
        frame(lua_state, 24.0);
        PLAY_STEP(agent, Hash40::new("se_wario_step_left_s"));
        wait_loop_clear(agent);
    }
}

//Fast Walk Expression
unsafe extern "C" fn ssbexo_wario_fast_walk_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    loop {
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
        }
        frame(lua_state, 6.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 24.0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        wait_loop_clear(agent);
    }
}

//Dash ACMD
unsafe extern "C" fn ssbexo_wario_dash_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 5.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

//Dash Effect
unsafe extern "C" fn ssbexo_wario_dash_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Dash Sound
unsafe extern "C" fn ssbexo_wario_dash_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_dash_start"));
        SET_PLAY_INHIVIT(agent, Hash40::new("se_wario_dash_start"), 23);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_wario_step_right_l"));
    }
}

//Dash Expression
unsafe extern "C" fn ssbexo_wario_dash_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Run Effect
unsafe extern "C" fn ssbexo_wario_run_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    loop {
        frame(lua_state, 5.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(lua_state, 17.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        wait_loop_clear(agent);
    }
}

//Run Sound
unsafe extern "C" fn ssbexo_wario_run_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    loop {
        frame(lua_state, 5.0);
        if is_excute(agent) {
            PLAY_STEP(agent, Hash40::new("se_wario_step_left_m"));
        }
        frame(lua_state, 17.0);
        PLAY_STEP(agent, Hash40::new("se_wario_step_right_m"));
        wait_loop_clear(agent);
    }
}

//Run Expression
unsafe extern "C" fn ssbexo_wario_run_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    loop {
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
        }
        frame(lua_state, 5.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 17.0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        wait_loop_clear(agent);
    }
}

//Up Taunt Effect
unsafe extern "C" fn ssbexo_wario_up_taunt_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 71.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("wario_kamitsuki_end"), Hash40::new("top"), 0, 10, -2, 180, 0, 0, 0.7, false);
    }
}

//Up Taunt Sound
unsafe extern "C" fn ssbexo_wario_up_taunt_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("vc_wario_appeal01"));
    }
    frame(lua_state, 65.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_appeal_h01"));
    }
}

//Down Taunt Effect
unsafe extern "C" fn ssbexo_wario_down_taunt_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("wario_kamitsuki_end"), Hash40::new("top"), 4, 8.5, 8, -90, 0, 0, 0.7, false);
        EFFECT_FOLLOW(agent, Hash40::new("wario_kamitsuki_end"), Hash40::new("top"), 0, 6.5, 0, -90, 0, 0, 0.7, false);
    }
}

//Down Taunt Sound
unsafe extern "C" fn ssbexo_wario_down_taunt_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 18.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_wario_011"));
    }
}

//Toot Taunt ACMD
unsafe extern "C" fn ssbexo_wario_toot_taunt_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 1, 0, 0, 9.0, 0.0, 4.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 1.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_slip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//Toot Taunt Effect
unsafe extern "C" fn ssbexo_wario_toot_taunt_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 13.0);
    if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
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
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_special_l01"));
    }
    wait(lua_state, 30.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_wario_012"));
    }
}

//Toot Taunt Expression
unsafe extern "C" fn ssbexo_wario_toot_taunt_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attacks"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        agent.clear_lua_stack();
        lua_args!(agent, 1, 0.8, 0.02, 1000, 1, 0, 4, 14);
        sv_animcmd::AREA_WIND_2ND_RAD(lua_state);
         agent.pop_lua_stack(1);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 1);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
    }
}

//Toot Kamikaze ACMD
unsafe extern "C" fn ssbexo_wario_toot_kamikaze_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(lua_state, 55.0);
    if is_excute(agent) {
        SlowModule::set_whole(boma, 20, 30);
        FT_SET_FINAL_FEAR_FACE(agent, 30);
    }
    frame(lua_state, 57.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, FIGHTER_WARIO_GENERATE_ARTICLE_KAMIKAZE, false, -1);
    }
    frame(lua_state, 58.0);
    if is_excute(agent) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DEAD, false);
    }
}

//Toot Kamikaze Effect
unsafe extern "C" fn ssbexo_wario_toot_kamikaze_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let lr_check = get_value_float(lua_state, *SO_VAR_FLOAT_LR);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("wario_kamikaze_start"), Hash40::new("top"), 0, 6, 0, 0, -60, 0, 1, false);
    }
    if lr_check < 0.0 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("wario_kamikaze"), Hash40::new("top"), 0, 7, -1, 0, 180, 0, 1, true);
            LAST_EFFECT_SET_RATE(agent, 0.8);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("wario_kamikaze"), Hash40::new("top"), 0, 7, -0.5, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_RATE(agent, 0.8);
        }
    }
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, true);
        sv_animcmd::LAST_EFFECT_SET_DISABLE_SYSTEM_SLOW(lua_state);
    }
    frame(lua_state, 5.0);
    if lr_check < 0.0 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wario_ppe_s"), Hash40::new("top"), 0, 0, 0, 0, 100, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wario_ppe_s"), Hash40::new("top"), 0, 0, 0, 0, -100, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 20.0);
    if lr_check < 0.0 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wario_ppe_s"), Hash40::new("top"), 0, 0, 0, 0, 100, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wario_ppe_s"), Hash40::new("top"), 0, 0, 0, 0, -100, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 35.0);
    if lr_check < 0.0 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wario_ppe_s"), Hash40::new("top"), 0, 0, 0, 0, 100, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wario_ppe_s"), Hash40::new("top"), 0, 0, 0, 0, -100, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 45.0);
    
    if is_excute(agent) {
        FLASH(agent, 0, 0, 0, 0);
        ColorBlendModule::set_disable_camera_depth_influence(boma, true);
        BURN_COLOR(agent, 4, 4, 1.2, 0);
        ColorBlendModule::set_disable_camera_depth_influence(boma, true);
    }
    frame(lua_state, 50.0);
    if lr_check < 0.0 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wario_ppe_s"), Hash40::new("top"), 0, 0, 0, 0, 100, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wario_ppe_s"), Hash40::new("top"), 0, 0, 0, 0, -100, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, 1.0, 1.0, 1.0, 1);
        sv_animcmd::FLASH_NO_STOP(lua_state);
        FLASH_FRM(agent, 50, 1, 1, 1, 1);
        BURN_COLOR_FRAME(agent, 50, 4, 4, 4, 1);
    }
    frame(lua_state, 57.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("wario_shock"), Hash40::new("top"), 3, 0, -4, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("wario_ground_crack"), Hash40::new("top"), 3, 0, -7, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.5);
    }
}

//Toot Kamikaze Sound
unsafe extern "C" fn ssbexo_wario_toot_kamikaze_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_special_s01"));
        PLAY_SE(agent, Hash40::new("se_wario_special_s07"));
    }
}

//Toot Kamikaze Expression
unsafe extern "C" fn ssbexo_wario_toot_kamikaze_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattack"), 40, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Win 1 ACMD
unsafe extern "C" fn ssbexo_wario_win_1_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(boma, *FIGHTER_WARIO_GENERATE_ARTICLE_GARLIC, true, -1);
        if ArticleModule::is_exist(boma, *FIGHTER_WARIO_GENERATE_ARTICLE_GARLIC) {
            let garlic_boma = get_article_boma(boma, *FIGHTER_WARIO_GENERATE_ARTICLE_GARLIC);
            LinkModule::set_model_constraint_pos_ort(garlic_boma, *LINK_NO_CONSTRAINT, Hash40::new("have"), Hash40::new("haver"), (*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE | *CONSTRAINT_FLAG_OFFSET_ROT) as u32, true);
        }
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_WARIO_GENERATE_ARTICLE_GARLIC, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Win 1 Effect
unsafe extern "C" fn ssbexo_wario_win_1_effect(_agent: &mut L2CAgentBase) {}

//Win 1 Sound
unsafe extern "C" fn ssbexo_wario_win_1_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 25.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("vc_wario_007"));
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("vc_wario_005"));
    }
    frame(lua_state, 115.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("vc_wario_win02"));
    }
}

//Win 1 Expression
unsafe extern "C" fn ssbexo_wario_win_1_expression(_agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("effect_walkslow", ssbexo_wario_slow_walk_effect, Low)
    .acmd("sound_walkslow", ssbexo_wario_slow_walk_sound, Low)
    .acmd("expression_walkslow", ssbexo_wario_slow_walk_expression, Low)
    .acmd("effect_walkmiddle", ssbexo_wario_middle_walk_effect, Low)
    .acmd("sound_walkmiddle", ssbexo_wario_middle_walk_sound, Low)
    .acmd("expression_walkmiddle", ssbexo_wario_middle_walk_expression, Low)
    .acmd("effect_walkfast", ssbexo_wario_fast_walk_effect, Low)
    .acmd("sound_walkfast", ssbexo_wario_fast_walk_sound, Low)
    .acmd("expression_walkfast", ssbexo_wario_fast_walk_expression, Low)
    .acmd("game_dash", ssbexo_wario_dash_acmd, Low)
    .acmd("effect_dash", ssbexo_wario_dash_effect, Low)
    .acmd("sound_dash", ssbexo_wario_dash_sound, Low)
    .acmd("expression_dash", ssbexo_wario_dash_expression, Low)
    .acmd("effect_run", ssbexo_wario_run_effect, Low)
    .acmd("sound_run", ssbexo_wario_run_sound, Low)
    .acmd("expression_run", ssbexo_wario_run_expression, Low)
    .acmd("effect_appealhil", ssbexo_wario_up_taunt_effect, Low)
    .acmd("sound_appealhil", ssbexo_wario_up_taunt_sound, Low)
    .acmd("effect_appealhir", ssbexo_wario_up_taunt_effect, Low)
    .acmd("sound_appealhir", ssbexo_wario_up_taunt_sound, Low)
    .acmd("effect_appeallwl", ssbexo_wario_down_taunt_effect, Low)
    .acmd("sound_appeallwl", ssbexo_wario_down_taunt_sound, Low)
    .acmd("effect_appeallwr", ssbexo_wario_down_taunt_effect, Low)
    .acmd("sound_appeallwr", ssbexo_wario_down_taunt_sound, Low)
    .acmd("game_appealgas", ssbexo_wario_toot_taunt_acmd, Low)
    .acmd("effect_appealgas", ssbexo_wario_toot_taunt_effect, Low)
    .acmd("sound_appealgas", ssbexo_wario_toot_taunt_sound, Low)
    .acmd("expression_appealgas", ssbexo_wario_toot_taunt_expression, Low)
    .acmd("game_appealkamikaze", ssbexo_wario_toot_kamikaze_acmd, Low)
    .acmd("effect_appealkamikaze", ssbexo_wario_toot_kamikaze_effect, Low)
    .acmd("sound_appealkamikaze", ssbexo_wario_toot_kamikaze_sound, Low)
    .acmd("expression_appealkamikaze", ssbexo_wario_toot_kamikaze_expression, Low)
    .acmd("game_win1", ssbexo_wario_win_1_acmd, Low)
    .acmd("effect_win1", ssbexo_wario_win_1_effect, Low)
    .acmd("sound_win1", ssbexo_wario_win_1_sound, Low)
    .acmd("expression_win1", ssbexo_wario_win_1_expression, Low)
    .install()
    ;
}