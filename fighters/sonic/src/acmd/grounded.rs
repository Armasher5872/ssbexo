use super::*;

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_sonic_dash_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 62, 62, 0, 55, 4.0, 0.0, 3.0, 0.0, Some(0.0), Some(3.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 88, 62, 0, 55, 4.0, 0.0, 3.0, 0.0, Some(0.0), Some(3.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Dash Attack Effect
unsafe extern "C" fn ssbexo_sonic_dash_attack_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 33.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    wait(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Dash Attack Sound
unsafe extern "C" fn ssbexo_sonic_dash_attack_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_sonic_rnd_attack"));
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_sonic_swing_ll"));
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_sonic_step_right_m"));
    }
    frame(agent.lua_state_agent, 45.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_sonic_step_left_m"));
    }
}

//Dash Attack Expression
unsafe extern "C" fn ssbexo_sonic_dash_attack_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("sonic")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attackdash", ssbexo_sonic_dash_attack_acmd, Low)
    .effect_acmd("effect_attackdash", ssbexo_sonic_dash_attack_effect, Low)
    .sound_acmd("sound_attackdash", ssbexo_sonic_dash_attack_sound, Low)
    .expression_acmd("expression_attackdash", ssbexo_sonic_dash_attack_expression, Low)
    .install()
    ;
}