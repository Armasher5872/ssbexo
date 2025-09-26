use super::*;

//Wait 2 Expression
unsafe extern "C" fn ssbexo_koopa_wait_2_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, 0);
    }
    frame(agent.lua_state_agent, 56.0);
    for _ in 0..5 {
        if is_excute(agent) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
        wait(agent.lua_state_agent, 10.0);
    }
}

//Wait 3 Expression
unsafe extern "C" fn ssbexo_koopa_wait_3_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, 0);
    }
    frame(agent.lua_state_agent, 34.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
    frame(agent.lua_state_agent, 63.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
    frame(agent.lua_state_agent, 106.0);
    for _ in 0..7 {
        if is_excute(agent) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
        wait(agent.lua_state_agent, 10.0);
    }
}

//Up Taunt Effect
unsafe extern "C" fn ssbexo_koopa_up_taunt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("koopa_wait_breath"), Hash40::new("head"), 0, 4, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 90.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("koopa_wait_breath"), false, true);
    }
}

//Up Taunt Sound
unsafe extern "C" fn ssbexo_koopa_up_taunt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        PLAY_SE_REMAIN(agent, Hash40::new("vc_koopa_attack07"));
    }
}

//Up Taunt Expression
unsafe extern "C" fn ssbexo_koopa_up_taunt_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, 0);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, 0);
    }
    frame(agent.lua_state_agent, 45.0);
    for _ in 0..4 {
        if is_excute(agent) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
        wait(agent.lua_state_agent, 10.0);
    }
}

//Side Taunt ACMD
unsafe extern "C" fn ssbexo_koopa_side_taunt_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 32.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("head"), 6.0, 40, 60, 0, 80, 6.0, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    frame(agent.lua_state_agent, 36.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 56.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("head"), 6.0, 40, 60, 0, 80, 6.0, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    frame(agent.lua_state_agent, 60.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 73.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("head"), 6.0, 40, 60, 0, 80, 6.0, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    frame(agent.lua_state_agent, 77.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Side Taunt Expression
unsafe extern "C" fn ssbexo_koopa_side_taunt_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 34.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, 0);
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
    frame(agent.lua_state_agent, 56.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, 0);
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
    frame(agent.lua_state_agent, 73.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, 0);
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_koopa_dash_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 361, 45, 0, 95, 7.0, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }  
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Dash Attack Effect
unsafe extern "C" fn ssbexo_koopa_dash_attack_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Dash Attack Sound
unsafe extern "C" fn ssbexo_koopa_dash_attack_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_koopa_attackdash"));
    }
}

//Dash Attack Expression
unsafe extern "C" fn ssbexo_koopa_dash_attack_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 4);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_landl_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}

pub fn install() {
    Agent::new("koopa")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .expression_acmd("expression_wait2", ssbexo_koopa_wait_2_expression, Low)
    .expression_acmd("expression_wait3", ssbexo_koopa_wait_3_expression, Low)
    .effect_acmd("effect_appealhil", ssbexo_koopa_up_taunt_effect, Low)
    .effect_acmd("effect_appealhir", ssbexo_koopa_up_taunt_effect, Low)
    .sound_acmd("sound_appealhil", ssbexo_koopa_up_taunt_sound, Low)
    .sound_acmd("sound_appealhir", ssbexo_koopa_up_taunt_sound, Low)
    .expression_acmd("expression_appealhil", ssbexo_koopa_up_taunt_expression, Low)
    .expression_acmd("expression_appealhir", ssbexo_koopa_up_taunt_expression, Low)
    .game_acmd("game_appealsr", ssbexo_koopa_side_taunt_acmd, Low)
    .game_acmd("game_appealsl", ssbexo_koopa_side_taunt_acmd, Low)
    .expression_acmd("expression_appealsl", ssbexo_koopa_side_taunt_expression, Low)
    .expression_acmd("expression_appealsr", ssbexo_koopa_side_taunt_expression, Low)
    .game_acmd("game_attackdash", ssbexo_koopa_dash_attack_acmd, Low)
    .effect_acmd("effect_attackdash", ssbexo_koopa_dash_attack_effect, Low)
    .sound_acmd("sound_attackdash", ssbexo_koopa_dash_attack_sound, Low)
    .expression_acmd("expression_attackdash", ssbexo_koopa_dash_attack_expression, Low)
    .install()
    ;
}