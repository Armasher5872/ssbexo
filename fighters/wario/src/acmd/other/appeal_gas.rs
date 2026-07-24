use super::*;

//Toot Taunt ACMD
unsafe extern "C" fn ssbexo_wario_toot_taunt_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 1, 0, 0, 9.0, 0.0, 4.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 1.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_slip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//Toot Taunt Effect
unsafe extern "C" fn ssbexo_wario_toot_taunt_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 13.0);
    if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wario_ppe_s"), Hash40::new("top"), 0, 0, 0, 0, 100, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("wario_ppe_s"), Hash40::new("top"), 0, 0, 0, 0, -100, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

//Toot Taunt Sound
unsafe extern "C" fn ssbexo_wario_toot_taunt_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_special_l01"));
    }
    wait(lua_state, 30.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_wario_012"));
    }
}

//Toot Taunt Expression
unsafe extern "C" fn ssbexo_wario_toot_taunt_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attacks"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        agent.clear_lua_stack();
        lua_args!(agent, 1, 0.8, 0.02, 1000, 1, 0, 4, 14);
        sv_animcmd::AREA_WIND_2ND_RAD(lua_state);
         agent.pop_lua_stack(1);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 1);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_appealgas", ssbexo_wario_toot_taunt_acmd, Low)
    .acmd("effect_appealgas", ssbexo_wario_toot_taunt_effect, Low)
    .acmd("sound_appealgas", ssbexo_wario_toot_taunt_sound, Low)
    .acmd("expression_appealgas", ssbexo_wario_toot_taunt_expression, Low)
    .install()
    ;
}