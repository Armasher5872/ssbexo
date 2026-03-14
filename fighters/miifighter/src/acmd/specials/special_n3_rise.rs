use super::*;

//Rising Tiger Knee Rise ACMD
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_rise_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        CORRECT(agent, *GROUND_CORRECT_KIND_AIR);
        SA_SET(agent, *SITUATION_KIND_AIR);
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 367, 80, 70, 0, 7.0, 0.0, 10.0, 3.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 290, 20, 20, 0, 7.0, 0.0, 10.0, 3.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legr"), 1.0, 367, 100, 10, 0, 4.0, 3.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 1.0, 367, 100, 10, 0, 5.0, 5.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
}

//Rising Tiger Knee Rise Effect
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_rise_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_machstamp"), Hash40::new("top"), 0, 7, 0, -30, 0, 0, 0.85, true);
    }
}

//Rising Tiger Knee Rise Sound
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_rise_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miifighter_swing_ll"));
        PLAY_SE(agent, Hash40::new("vc_mii_attack08"));
    }
}

//Rising Tiger Knee Rise Expression
unsafe extern "C" fn ssbexo_miifighter_rising_tiger_knee_rise_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

pub fn install() {
    Agent::new("miifighter")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialn3rise", ssbexo_miifighter_rising_tiger_knee_rise_acmd, Low)
    .effect_acmd("effect_specialn3rise", ssbexo_miifighter_rising_tiger_knee_rise_effect, Low)
    .sound_acmd("sound_specialn3rise", ssbexo_miifighter_rising_tiger_knee_rise_sound, Low)
    .expression_acmd("expression_specialn3rise", ssbexo_miifighter_rising_tiger_knee_rise_expression, Low)
    .install()
    ;
}