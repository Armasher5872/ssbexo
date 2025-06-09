use super::*;

//Shield Special Effect
unsafe extern "C" fn ssbexo_pzenigame_shield_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 50.0);
    for _ in 0..5 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_steam3"), Hash40::new("top"), 0, 20, 0, 0, 0, 0, 2.0, true);
            LAST_EFFECT_SET_COLOR(agent, 0.2, 0.2, 0.2);
            EFFECT_FOLLOW(agent, Hash40::new("sys_steam3"), Hash40::new("top"), 0, 20, -10, 0, 0, 0, 2.0, true);
            LAST_EFFECT_SET_COLOR(agent, 0.2, 0.2, 0.2);
            EFFECT_FOLLOW(agent, Hash40::new("sys_steam3"), Hash40::new("top"), 0, 20, 10, 0, 0, 0, 2.0, true);
            LAST_EFFECT_SET_COLOR(agent, 0.2, 0.2, 0.2);
            EFFECT_FOLLOW(agent, Hash40::new("sys_atk_speedline"), Hash40::new("top"), 0, 15, 0, -90, 0, 0, 1.5, true);
            LAST_EFFECT_SET_COLOR(agent, 0.698, 1.0, 1.0);
        }
        wait(agent.lua_state_agent, 6.0);
    }
}

//Shield Special Sound
unsafe extern "C" fn ssbexo_pzenigame_shield_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pzenigame_jump03"));
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pzenigame_landing01"));
    }
    frame(agent.lua_state_agent, 44.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_pzenigame_attack06"));
    }
}

//Up Special ACMD
unsafe extern "C" fn ssbexo_pzenigame_up_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.4, 367, 100, 110, 0, 9.0, 0.0, 1.0, 9.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS); 
    }
    frame(agent.lua_state_agent, 42.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 43.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 55, 152, 0, 70, 10.0, 0.0, 1.0, 9.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install() {
    Agent::new("pzenigame")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .effect_acmd("effect_specialshield", ssbexo_pzenigame_shield_special_effect, Low)
    .sound_acmd("sound_shieldspecial", ssbexo_pzenigame_shield_special_sound, Low)
    .game_acmd("game_specialhi", ssbexo_pzenigame_up_special_acmd, Low)
    .game_acmd("game_specialairhi", ssbexo_pzenigame_up_special_acmd, Low)
    .install()
    ;
}