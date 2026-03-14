use super::*;

//Rising Tiger Knee Land ACMD
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_land_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 65, 100, 0, 50, 4.0, 0.0, 5.5, 10.0, Some(0.0), Some(5.5), Some(-10.0), 2.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
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
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

pub fn install() {
    Agent::new("miifighter")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialn3land", ssbexo_miifighter_rising_tiger_knee_land_acmd, Low)
    .effect_acmd("effect_specialn3land", ssbexo_miifighter_rising_tiger_knee_land_effect, Low)
    .sound_acmd("sound_specialn3land", ssbexo_miifighter_rising_tiger_knee_land_sound, Low)
    .expression_acmd("expression_specialn3land", ssbexo_miifighter_rising_tiger_knee_land_expression, Low)
    .install()
    ;
}