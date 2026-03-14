use super::*;

//Rising Tiger Knee Start ACMD
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_start_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 367, 20, 20, 0, 7.0, 0.0, 10.0, 3.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
}

//Grounded Rising Tiger Knee Start Effect
unsafe extern "C" fn ssbexo_miifighter_grounded_rising_tiger_knee_start_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 10, 8, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_arc_c"), Hash40::new("sys_attack_arc_c"), Hash40::new("top"), 0, 11.5, 3, -62, 9, 90, 0.8, true, *EF_FLIP_YZ, 0.3);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}

//Aerial Rising Tiger Knee Start Effect
unsafe extern "C" fn ssbexo_miifighter_aerial_rising_tiger_knee_start_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 10, 8, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_arc_c"), Hash40::new("sys_attack_arc_c"), Hash40::new("top"), 0, 11.5, 3, -62, 9, 90, 0.8, true, *EF_FLIP_YZ, 0.3);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}

//Rising Tiger Knee Start Sound
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_start_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_miifighter_rnd_jump"));
    }
}

//Grounded Rising Tiger Knee Start Expression
unsafe extern "C" fn ssbexo_miifighter_grounded_rising_tiger_knee_start_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
}

//Aerial Rising Tiger Knee Start Expression
unsafe extern "C" fn ssbexo_miifighter_aerial_rising_tiger_knee_start_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
}

pub fn install() {
    Agent::new("miifighter")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialn3", ssbexo_miifighter_rising_tiger_knee_start_acmd, Low)
    .game_acmd("game_specialairn3", ssbexo_miifighter_rising_tiger_knee_start_acmd, Low)
    .effect_acmd("effect_specialn3", ssbexo_miifighter_grounded_rising_tiger_knee_start_effect, Low)
    .effect_acmd("effect_specialairn3", ssbexo_miifighter_aerial_rising_tiger_knee_start_effect, Low)
    .sound_acmd("sound_specialn3", ssbexo_miifighter_rising_tiger_knee_start_sound, Low)
    .sound_acmd("sound_specialairn3", ssbexo_miifighter_rising_tiger_knee_start_sound, Low)
    .expression_acmd("expression_specialn3", ssbexo_miifighter_grounded_rising_tiger_knee_start_expression, Low)
    .expression_acmd("expression_specialairn3", ssbexo_miifighter_aerial_rising_tiger_knee_start_expression, Low)
    .install()
    ;
}