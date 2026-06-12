use super::*;

//Devils Steel Pedal ACMD
unsafe extern "C" fn ssbexo_demon_devils_steel_pedal_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        JostleModule::set_push_speed_x_overlap_rate_status(boma, 0.1);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 90, 0, 0, 80, 4.2, 0.0, 17.0, 11.0, Some(0.0), Some(4.2), Some(3.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 90, 0, 0, 80, 4.2, 0.0, 17.0, 11.0, Some(0.0), Some(4.2), Some(3.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("top"), 13.0, 65, 30, 0, 80, 4.2, 0.0, 17.0, 11.0, Some(0.0), Some(4.2), Some(3.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_down_only(boma, 0, false);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        JostleModule::set_push_speed_x_overlap_rate_status(boma, 0.0);
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

//Devils Steel Pedal Effect
unsafe extern "C" fn ssbexo_demon_devils_steel_pedal_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("demon_attack_speedline_kick"), Hash40::new("top"), 0, 9, 6, -55, 13, 0, 0.55, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.8);
    }
}

//Devils Steel Pedal Sound
unsafe extern "C" fn ssbexo_demon_devils_steel_pedal_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_demon_swing_short02"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_demon_rnd_attack_s_01"));
    }
}

//Devils Steel Pedal Expression
unsafe extern "C" fn ssbexo_demon_devils_steel_pedal_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        RUMBLE_HIT(agent, Hash40::new("rbkind_80_attacks2"), 0);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 4);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_80_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 47.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_attackstand1", ssbexo_demon_devils_steel_pedal_acmd, Low)
    .acmd("effect_attackstand1", ssbexo_demon_devils_steel_pedal_effect, Low)
    .acmd("sound_attackstand1", ssbexo_demon_devils_steel_pedal_sound, Low)
    .acmd("expression_attackstand1", ssbexo_demon_devils_steel_pedal_expression, Low)
    .install()
    ;
}