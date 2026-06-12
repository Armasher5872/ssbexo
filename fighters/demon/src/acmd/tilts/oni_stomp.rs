use super::*;

//Oni Stomp ACMD
unsafe extern "C" fn ssbexo_demon_oni_stomp_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 270, 0, 0, 210, 5.0, 0.0, -2.0, 8.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_ALL, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//Oni Stomp Effect
unsafe extern "C" fn ssbexo_demon_oni_stomp_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 360, true, 0.2);
        LAST_EFFECT_SET_RATE(agent, 2);
    }
}

//Oni Stomp Sound
unsafe extern "C" fn ssbexo_demon_oni_stomp_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 9.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_demon_swing_short02"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_demon_rnd_attack_s_01"));
    }
}

//Oni Stomp Expression
unsafe extern "C" fn ssbexo_demon_oni_stomp_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
        RUMBLE_HIT(agent, Hash40::new("rbkind_80_attacks2"), 0);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_80_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(lua_state, 56.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_attacklw3edge", ssbexo_demon_oni_stomp_acmd, Low)
    .acmd("effect_attacklw3edge", ssbexo_demon_oni_stomp_effect, Low)
    .acmd("sound_attacklw3edge", ssbexo_demon_oni_stomp_sound, Low)
    .acmd("expression_attacklw3edge", ssbexo_demon_oni_stomp_expression, Low)
    .install()
    ;
}