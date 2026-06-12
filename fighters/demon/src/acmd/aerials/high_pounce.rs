use super::*;

//High Pounce ACMD
unsafe extern "C" fn ssbexo_demon_high_pounce_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 0, 0, 0, 25, 4.5, 0.0, 6.0, 9.25, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 0, 0, 0, 25, 3.0, 0.0, 9.0, 6.25, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.0);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.0);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 1.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 1.0, false);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//High Pounce Effect
unsafe extern "C" fn ssbexo_demon_high_pounce_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("demon_combo_elec"), Hash40::new("havel"), 0.5, 0, -0.5, 0, 0, 0, 1, true);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("demon_attack_speedline"), Hash40::new("top"), 0, 9, 5.5, 45, 5, 0, 1, true);
        LAST_EFFECT_SET_ALPHA(agent, 0.3);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("demon_combo_elec"), false, false);
    }
}

//High Pounce Sound
unsafe extern "C" fn ssbexo_demon_high_pounce_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_demon_rnd_attack_s_02"));
        PLAY_LANDING_SE(agent, Hash40::new("se_demon_landing02"));
    }
}

//High Pounce Expression
unsafe extern "C" fn ssbexo_demon_high_pounce_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        RUMBLE_HIT(agent, Hash40::new("rbkind_80_attacks2"), 0);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_80_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_highpounce", ssbexo_demon_high_pounce_acmd, Low)
    .acmd("effect_highpounce", ssbexo_demon_high_pounce_effect, Low)
    .acmd("sound_highpounce", ssbexo_demon_high_pounce_sound, Low)
    .acmd("expression_highpounce", ssbexo_demon_high_pounce_expression, Low)
    .install()
    ;
}