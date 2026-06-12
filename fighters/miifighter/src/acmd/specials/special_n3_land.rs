use super::*;

//Rising Tiger Knee Land ACMD
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_land_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 65, 100, 0, 50, 4.0, 0.0, 5.5, 10.0, Some(0.0), Some(5.5), Some(-10.0), 2.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//Rising Tiger Knee Land Effect
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_land_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0.0, 0.0, 0, 0.0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.75);
        EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 0.0, 0.0, 0, 0.0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.75);
    }
}

//Rising Tiger Knee Land Sound
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_land_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_heavy_hit_m"));
    }
}

//Rising Tiger Knee Land Expression
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_land_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

pub fn install() {
    Agent::new("miifighter")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialn3land", ssbexo_miifighter_rising_tiger_knee_land_acmd, Low)
    .acmd("effect_specialn3land", ssbexo_miifighter_rising_tiger_knee_land_effect, Low)
    .acmd("sound_specialn3land", ssbexo_miifighter_rising_tiger_knee_land_sound, Low)
    .acmd("expression_specialn3land", ssbexo_miifighter_rising_tiger_knee_land_expression, Low)
    .install()
    ;
}