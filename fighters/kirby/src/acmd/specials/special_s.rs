use super::*;

//Side Special ACMD
unsafe extern "C" fn ssbexo_kirby_side_special_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 13.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 46, 71, 0, 82, 5.0, 0.0, 5.0, 3.0, Some(0.0), Some(5.0), Some(0.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 76, 50, 0, 80, 3.5, 0.0, 5.0, 4.0, Some(0.0), Some(5.0), Some(-0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 76, 50, 0, 80, 3.5, 0.0, 5.0, 4.0, Some(0.0), Some(5.0), Some(0.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
}

//Side Special Effect
unsafe extern "C" fn ssbexo_kirby_side_special_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 9.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("kirby_dash"), Hash40::new("top"), 0, 6, 5, -90, 0, 160, 0.7, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        BURN_COLOR(agent, 2, 0.059, 0.008, 0);
        BURN_COLOR_FRAME(agent, 4, 2, 0.059, 0.008, 0.9);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        BURN_COLOR(agent, 2, 0.059, 0.008, 0.9);
        BURN_COLOR_FRAME(agent, 12, 2, 0.059, 0.008, 0);
        EFFECT_OFF_KIND(agent, Hash40::new("kirby_dash"), false, true);
    }
    frame(lua_state, 46.0);
    if is_excute(agent) {
        BURN_COLOR_NORMAL(agent);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

//Side Special Sound
unsafe extern "C" fn ssbexo_kirby_side_special_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
	frame(lua_state, 13.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_kirby_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_kirby_attackdash"));
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        PLAY_LANDING_SE(agent, Hash40::new("se_kirby_landing02"));
    }
}

//Side Special Expression
unsafe extern "C" fn ssbexo_kirby_side_special_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 15);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("kirby")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specials", ssbexo_kirby_side_special_acmd, Low)
    .acmd("game_specialairs", ssbexo_kirby_side_special_acmd, Low)
    .acmd("effect_specials", ssbexo_kirby_side_special_effect, Low)
    .acmd("effect_specialairs", ssbexo_kirby_side_special_effect, Low)
    .acmd("sound_specials", ssbexo_kirby_side_special_sound, Low)
    .acmd("sound_specialairs", ssbexo_kirby_side_special_sound, Low)
    .acmd("expression_specials", ssbexo_kirby_side_special_expression, Low)
    .acmd("expression_specialairs", ssbexo_kirby_side_special_expression, Low)
    .install()
    ;
}