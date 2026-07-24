use super::*;

//Fujin Uraken ACMD
unsafe extern "C" fn ssbexo_demon_fujin_uraken_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 41, 20, 0, 70, 6.0, 0.0, 9.5, 5.0, Some(0.0), Some(9.5), Some(8.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//Fujin Uraken Effect
unsafe extern "C" fn ssbexo_demon_fujin_uraken_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("demon_attack_arc_d"), Hash40::new("top"), 0, 8.2, 4, 12, 0, -177, 0.85, 0, 0, 1, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}

//Fujin Uraken Sound
unsafe extern "C" fn ssbexo_demon_fujin_uraken_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_demon_swing_short02"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_demon_rnd_attack_s_03"));
    }
}

//Fujin Uraken Expression
unsafe extern "C" fn ssbexo_demon_fujin_uraken_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        RUMBLE_HIT(agent, Hash40::new("rbkind_80_attackm"), 0);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_80_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_attacklw3attack", ssbexo_demon_fujin_uraken_acmd, Low)
    .acmd("effect_attacklw3attack", ssbexo_demon_fujin_uraken_effect, Low)
    .acmd("sound_attacklw3attack", ssbexo_demon_fujin_uraken_sound, Low)
    .acmd("expression_attacklw3attack", ssbexo_demon_fujin_uraken_expression, Low)
    .install()
    ;
}