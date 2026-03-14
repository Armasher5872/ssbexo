use super::*;

//Rising Tiger Knee Dive ACMD
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_dive_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legr"), 1.0, 367, 100, 120, 0, 4.0, 3.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 1.0, 367, 100, 120, 0, 5.0, 5.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
}

//Rising Tiger Knee Dive Effect
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_dive_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_machstamp"), false, false);
        EFFECT_FOLLOW(agent, Hash40::new("sys_machstamp"), Hash40::new("top"), 0, 7, 0, -30, 0, 0, 0.85, true);
    }
}

//Rising Tiger Knee Dive Expression
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_dive_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

pub fn install() {
    Agent::new("miifighter")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialn3dive", ssbexo_miifighter_rising_tiger_knee_dive_acmd, Low)
    .effect_acmd("effect_specialn3dive", ssbexo_miifighter_rising_tiger_knee_dive_effect, Low)
    .expression_acmd("expression_specialn3dive", ssbexo_miifighter_rising_tiger_knee_dive_expression, Low)
    .install()
    ;
}