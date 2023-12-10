use super::*;

//Taunt Effect
unsafe extern "C" fn ssbexo_armstrong_taunt_effect(_agent: &mut L2CAgentBase) {}

//Up Taunt Sound
unsafe extern "C" fn ssbexo_armstrong_up_taunt_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ganon_appeal_h01"));
    }
}

//Side Taunt Sound
unsafe extern "C" fn ssbexo_armstrong_side_taunt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 53.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ganon_appeal_s01"));
    }
}

//Down Taunt ACMD
unsafe extern "C" fn ssbexo_armstrong_down_taunt_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0.0);
        HitModule::set_check_catch(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 80.0);
    if macros::is_excute(agent) {
        HitModule::set_check_catch(agent.module_accessor, true, 0);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
    }
}

//Down Taunt Sound
unsafe extern "C" fn ssbexo_armstrong_down_taunt_sound(_agent: &mut L2CAgentBase) {}

//Ledge Attack Effect
unsafe extern "C" fn ssbexo_armstrong_ledge_attack_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 0, 0, 4, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 1, 10, 4, 0, 25, 25, 1.25, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        macros::LAST_EFFECT_SET_RATE(agent, 2.0);
    }
}

//Face Down Getup Attack Effect
unsafe extern "C" fn ssbexo_armstrong_face_down_getup_attack_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 2.5, 5, -15, 180, 0, 1.4, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        macros::LAST_EFFECT_SET_RATE(agent, 2.0);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -7, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 8, -18.5, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true, 1.5);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -0.5, 10, 0.8, 180, -230, 90, 1.4, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 14, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_attack_arc_d"), false, false);
    }
}

//Face Up Getup Attack Effect
unsafe extern "C" fn ssbexo_armstrong_face_up_getup_attack_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -3, 6, -1, 0, 180, 15, 1.6, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        macros::LAST_EFFECT_SET_RATE(agent, 1.7);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_attack_arc_d"), false, false);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 5.5, 0, 0, 40, 13, 1.5, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
    }
}

//Jab ACMD
unsafe extern "C" fn ssbexo_armstrong_jab_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 1.2);
        DamageModule::set_reaction_mul(agent.module_accessor, 0.85);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 361, 74, 0, 41, 4.4, 0.0, 12.5, 11.0, Some(0.0), Some(9.5), Some(11.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 361, 74, 0, 41, 3.5, 0.0, 12.5, 7.0, Some(0.0), Some(9.5), Some(7.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        DamageModule::set_reaction_mul(agent.module_accessor, 1.0);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
    }
}

//Jab Effect
unsafe extern "C" fn ssbexo_armstrong_jab_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 16.0, -1.5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true, 1.5);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_armstrong_dash_attack_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 6.0);
        DamageModule::set_reaction_mul(agent.module_accessor, 0.85);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 70, 85, 0, 50, 7.0, 0.0, 9.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 80, 60, 0, 45, 4.0, 0.0, 9.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.3);
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        DamageModule::set_reaction_mul(agent.module_accessor, 1.0);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
    }
}

//Dash Attack Effect
unsafe extern "C" fn ssbexo_armstrong_dash_attack_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("ganon_attack_impact"), Hash40::new("top"), 0, 9, 6.5, 0, 0, 0, 1.6, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, 2, 0, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(agent, 0.88, 0.35, 0.13);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, 2, 0, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(agent, 0.88, 0.35, 0.13);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -2.8, 0, 7.5, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install() {
    Agent::new("ganon")
    .effect_acmd("effect_appealhir", ssbexo_armstrong_taunt_effect)
    .effect_acmd("effect_appealhil", ssbexo_armstrong_taunt_effect)
    .effect_acmd("effect_appealsr", ssbexo_armstrong_taunt_effect)
    .effect_acmd("effect_appealsl", ssbexo_armstrong_taunt_effect)
    .effect_acmd("effect_appeallwr", ssbexo_armstrong_taunt_effect)
    .effect_acmd("effect_appeallwl", ssbexo_armstrong_taunt_effect)
    .sound_acmd("sound_appealhir", ssbexo_armstrong_up_taunt_sound)
    .sound_acmd("sound_appealhil", ssbexo_armstrong_up_taunt_sound)
    .sound_acmd("sound_appealsr", ssbexo_armstrong_side_taunt_sound)
    .sound_acmd("sound_appealsl", ssbexo_armstrong_side_taunt_sound)
    .game_acmd("game_appeallwr", ssbexo_armstrong_down_taunt_acmd)
    .game_acmd("game_appeallwl", ssbexo_armstrong_down_taunt_acmd)
    .sound_acmd("sound_appeallwr", ssbexo_armstrong_down_taunt_sound)
    .sound_acmd("sound_appeallwl", ssbexo_armstrong_down_taunt_sound)
    .effect_acmd("effect_cliffattack", ssbexo_armstrong_ledge_attack_effect)
    .effect_acmd("effect_attackdownd", ssbexo_armstrong_face_down_getup_attack_effect)
    .effect_acmd("effect_attackdownu", ssbexo_armstrong_face_up_getup_attack_effect)
    .game_acmd("game_attack11", ssbexo_armstrong_jab_acmd)
    .effect_acmd("effect_attack11", ssbexo_armstrong_jab_effect)
    .game_acmd("game_attackdash", ssbexo_armstrong_dash_attack_acmd)
    .effect_acmd("effect_attackdash", ssbexo_armstrong_dash_attack_effect)
    .install()
    ;
}