use super::*;

//Punisher Turn Effect
unsafe extern "C" fn ssbexo_cloud_punisher_turn_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Punisher Turn Sound
unsafe extern "C" fn ssbexo_cloud_punisher_turn_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_STEP(agent, Hash40::new("se_cloud_step_right_m"));
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_STEP(agent, Hash40::new("se_cloud_step_left_m"));
    }
}

//Punisher Turn Expression
unsafe extern "C" fn ssbexo_cloud_punisher_turn_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

//Punisher Slow Walk Effect
unsafe extern "C" fn ssbexo_cloud_punisher_slow_walk_effect(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 51.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footl"), 4, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 36.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footr"), 4, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
        }
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

//Punisher Slow Walk Sound
unsafe extern "C" fn ssbexo_cloud_punisher_slow_walk_sound(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 47.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP(agent, Hash40::new("se_cloud_step_left_s"));
        }
        wait(agent.lua_state_agent, 36.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP(agent, Hash40::new("se_cloud_step_right_s"));
        }
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

//Punisher Slow Walk Expression
unsafe extern "C" fn ssbexo_cloud_punisher_slow_walk_expression(agent: &mut L2CAgentBase) {
    loop {
        if macros::is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
        }
        frame(agent.lua_state_agent, 37.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(agent.lua_state_agent, 21.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

//Punisher Middle Walk Effect
unsafe extern "C" fn ssbexo_cloud_punisher_middle_walk_effect(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 35.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footl"), 4, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 27.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footr"), 4, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
        }
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

//Punisher Middle Walk Sound
unsafe extern "C" fn ssbexo_cloud_punisher_middle_walk_sound(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 34.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP(agent, Hash40::new("se_cloud_step_left_s"));
        }
        wait(agent.lua_state_agent, 26.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP(agent, Hash40::new("se_cloud_step_right_s"));
        }
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

//Punisher Middle Walk Expression
unsafe extern "C" fn ssbexo_cloud_punisher_middle_walk_expression(agent: &mut L2CAgentBase) {
    loop {
        if macros::is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
        }
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

//Punisher Fast Walk Effect
unsafe extern "C" fn ssbexo_cloud_punisher_fast_walk_effect(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 21.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footl"), 4, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 43.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footr"), 4, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
        }
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

//Punisher Fast Walk Sound
unsafe extern "C" fn ssbexo_cloud_punisher_fast_walk_sound(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 23.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP(agent, Hash40::new("se_cloud_step_left_s"));
        }
        wait(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP(agent, Hash40::new("se_cloud_step_right_s"));
        }
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

//Punisher Fast Walk Expression
unsafe extern "C" fn ssbexo_cloud_punisher_fast_walk_expression(agent: &mut L2CAgentBase) {
    loop {
        if macros::is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
        }
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(agent.lua_state_agent, 16.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

//Punisher Dash ACMD
unsafe extern "C" fn ssbexo_cloud_punisher_dash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        WorkModule::enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

//Punisher Dash Effect
unsafe extern "C" fn ssbexo_cloud_punisher_dash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Punisher Dash Sound
unsafe extern "C" fn ssbexo_cloud_punisher_dash_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_cloud_dash_start"));
        macros::SET_PLAY_INHIVIT(agent, Hash40::new("se_cloud_dash_start"), 20);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::PLAY_STEP(agent, Hash40::new("se_cloud_step_left_l"));
    }
    frame(agent.lua_state_agent, 55.0);
    if macros::is_excute(agent) {
        macros::PLAY_STEP(agent, Hash40::new("se_cloud_step_left_r"));
    }
    frame(agent.lua_state_agent, 76.0);
    if macros::is_excute(agent) {
        macros::PLAY_STEP(agent, Hash40::new("se_cloud_step_left_l"));
    }
}

//Punisher Dash Expression
unsafe extern "C" fn ssbexo_cloud_punisher_dash_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

//Punisher Jump Squat Expression
unsafe extern "C" fn ssbexo_cloud_punisher_jumpsquat_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

//Punisher Forward Shorthop Effect
unsafe extern "C" fn ssbexo_cloud_punisher_forward_shorthop_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
}

//Punisher Forward Shorthop Sound
unsafe extern "C" fn ssbexo_cloud_punisher_forward_shorthop_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_cloud_jump03"));
    }
}

//Punisher Forward Fullhop Effect
unsafe extern "C" fn ssbexo_cloud_punisher_forward_fullhop_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
}

//Punisher Forward Fullhop Sound
unsafe extern "C" fn ssbexo_cloud_punisher_forward_fullhop_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GIMMICK_SPRING_JUMP_FROM_RING) {
        if macros::is_excute(agent) {
            macros::PLAY_SEQUENCE(agent, Hash40::new("seq_cloud_rnd_jump"));
            macros::PLAY_STATUS(agent, Hash40::new("se_cloud_jump01"));
        }
    }
}

//Punisher Backward Shorthop Effect
unsafe extern "C" fn ssbexo_cloud_punisher_backward_shorthop_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
}

//Punisher Backward Shorthop Sound
unsafe extern "C" fn ssbexo_cloud_punisher_backward_shorthop_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_cloud_jump03"));
    }
}

//Punisher Backward Fullhop Effect
unsafe extern "C" fn ssbexo_cloud_punisher_backward_fullhop_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
}

//Punisher Backward Fullhop Sound
unsafe extern "C" fn ssbexo_cloud_punisher_backward_fullhop_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GIMMICK_SPRING_JUMP_FROM_RING) {
        if macros::is_excute(agent) {
            macros::PLAY_SEQUENCE(agent, Hash40::new("seq_cloud_rnd_jump"));
            macros::PLAY_STATUS(agent, Hash40::new("se_cloud_jump01"));
        }
    }
}

//Punisher Squat Effect
unsafe extern "C" fn ssbexo_cloud_punisher_squat_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHANGE_STATUS_DLAY_MOTION);
    }
}

//Punisher Squat Sound
unsafe extern "C" fn ssbexo_cloud_punisher_squat_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_cloud_squat"));
    }
}

//Punisher Squat Expression
unsafe extern "C" fn ssbexo_cloud_punisher_squat_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_l") as i64);
    }
}

//Punisher Squat Wait Sound
unsafe extern "C" fn ssbexo_cloud_punisher_squat_wait_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_SQUAT_FLAG_REQUEST_SQUAT_SE) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_cloud_squat"));
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_SQUAT_FLAG_REQUEST_SQUAT_SE);
        }
    }
}

//Punisher Squat Wait Expression
unsafe extern "C" fn ssbexo_cloud_punisher_squat_wait_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_l") as i64);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

//Punisher Squat Rv Sound
unsafe extern "C" fn ssbexo_cloud_punisher_squat_rv_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_cloud_rise"));
    }
}

//Punisher Squat Rv Expression
unsafe extern "C" fn ssbexo_cloud_punisher_squat_rv_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_l") as i64);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_r") as i64);
    }
}

pub fn install() {
    Agent::new("cloud")
    .effect_acmd("effect_punishturn", ssbexo_cloud_punisher_turn_effect, Priority::Low)
    .sound_acmd("sound_punishturn", ssbexo_cloud_punisher_turn_sound, Priority::Low)
    .expression_acmd("expression_punishturn", ssbexo_cloud_punisher_turn_expression, Priority::Low)
    .effect_acmd("effect_punishwalkslow", ssbexo_cloud_punisher_slow_walk_effect, Priority::Low)
    .sound_acmd("sound_punishwalkslow", ssbexo_cloud_punisher_slow_walk_sound, Priority::Low)
    .expression_acmd("expression_punishwalkslow", ssbexo_cloud_punisher_slow_walk_expression, Priority::Low)
    .effect_acmd("effect_punishwalkmiddle", ssbexo_cloud_punisher_middle_walk_effect, Priority::Low)
    .sound_acmd("sound_punishwalkmiddle", ssbexo_cloud_punisher_middle_walk_sound, Priority::Low)
    .expression_acmd("expression_punishwalkmiddle", ssbexo_cloud_punisher_middle_walk_expression, Priority::Low)
    .effect_acmd("effect_punishwalkfast", ssbexo_cloud_punisher_fast_walk_effect, Priority::Low)
    .sound_acmd("sound_punishwalkfast", ssbexo_cloud_punisher_fast_walk_sound, Priority::Low)
    .expression_acmd("expression_punishwalkfast", ssbexo_cloud_punisher_fast_walk_expression, Priority::Low)
    .game_acmd("game_punishdash", ssbexo_cloud_punisher_dash_acmd, Priority::Low)
    .effect_acmd("effect_punishdash", ssbexo_cloud_punisher_dash_effect, Priority::Low)
    .sound_acmd("sound_punishdash", ssbexo_cloud_punisher_dash_sound, Priority::Low)
    .expression_acmd("expression_punishdash", ssbexo_cloud_punisher_dash_expression, Priority::Low)
    .expression_acmd("expression_punishjumpsquat", ssbexo_cloud_punisher_jumpsquat_expression, Priority::Low)
    .effect_acmd("effect_punishjumpfrontmini", ssbexo_cloud_punisher_forward_shorthop_effect, Priority::Low)
    .sound_acmd("sound_punishjumpfrontmini", ssbexo_cloud_punisher_forward_shorthop_sound, Priority::Low)
    .effect_acmd("effect_punishjumpfront", ssbexo_cloud_punisher_forward_fullhop_effect, Priority::Low)
    .sound_acmd("sound_punishjumpfront", ssbexo_cloud_punisher_forward_fullhop_sound, Priority::Low)
    .effect_acmd("effect_punishjumpbackmini", ssbexo_cloud_punisher_backward_shorthop_effect, Priority::Low)
    .sound_acmd("sound_punishjumpbackmini", ssbexo_cloud_punisher_backward_shorthop_sound, Priority::Low)
    .effect_acmd("effect_punishjumpback", ssbexo_cloud_punisher_backward_fullhop_effect, Priority::Low)
    .sound_acmd("sound_punishjumpback", ssbexo_cloud_punisher_backward_fullhop_sound, Priority::Low)
    .effect_acmd("effect_punishsquat", ssbexo_cloud_punisher_squat_effect, Priority::Low)
    .sound_acmd("sound_punishsquat", ssbexo_cloud_punisher_squat_sound, Priority::Low)
    .expression_acmd("expression_punishsquat", ssbexo_cloud_punisher_squat_expression, Priority::Low)
    .sound_acmd("sound_punishsquatwait1", ssbexo_cloud_punisher_squat_wait_sound, Priority::Low)
    .expression_acmd("expression_punishsquatwait1", ssbexo_cloud_punisher_squat_wait_expression, Priority::Low)
    .sound_acmd("sound_punishsquatrv", ssbexo_cloud_punisher_squat_rv_sound, Priority::Low)
    .expression_acmd("expression_punishsquatrv", ssbexo_cloud_punisher_squat_rv_expression, Priority::Low)
    .install()
    ;
}