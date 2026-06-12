use super::*;

//Jawbreaker ACMD
unsafe extern "C" fn ssbexo_demon_jawbreaker_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 361, 10, 10, 0, 6.0, 0.0, 9.5, 5.0, Some(0.0), Some(9.5), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 300, 40, 50, 0, 6.0, 0.0, 9.5, 5.0, Some(0.0), Some(9.5), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//Jawbreaker Effect
unsafe extern "C" fn ssbexo_demon_jawbreaker_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("demon_attack_arc"), Hash40::new("top"), -1, 14.8, 2.4, 5, -15, 0, 0.7, true, 1);
        LAST_EFFECT_SET_RATE(agent, 2);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("demon_attack_arc"), false, false);
    }
}

//Jawbreaker Sound
unsafe extern "C" fn ssbexo_demon_jawbreaker_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 11.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_demon_swing_short02"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_demon_rnd_attack_s_03"));
    }
}

//Jawbreaker Expression
unsafe extern "C" fn ssbexo_demon_jawbreaker_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        RUMBLE_HIT(agent, Hash40::new("rbkind_80_attackm"), 0);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_80_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_escapeattack", ssbexo_demon_jawbreaker_acmd, Low)
    .acmd("effect_escapeattack", ssbexo_demon_jawbreaker_effect, Low)
    .acmd("sound_escapeattack", ssbexo_demon_jawbreaker_sound, Low)
    .acmd("expression_escapeattack", ssbexo_demon_jawbreaker_expression, Low)
    .install()
    ;
}