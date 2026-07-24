use super::*;

//Up Special ACMD
unsafe extern "C" fn ssbexo_mariod_up_special_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 90, 105, 0, 30, 6.0, 0.0, 6.0, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 90, 66, 0, 64, 6.0, 0.0, 9.5, 4.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

//Grounded Up Special Effect
unsafe extern "C" fn ssbexo_mariod_grounded_up_special_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 1.2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_aura"), Hash40::new("handl"), 2.0, 0.0, 0.0, 0, 0, 0, 1.0, true);
    }
    frame(lua_state, 3.0);
    if sv_animcmd::get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mariod_superjump_fnish"), Hash40::new("handl"), 2.5, 0, 1, 0, 0, 0, 1.05, true);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mariod_superjump_fnish"), Hash40::new("handl"), 2.5, 1, 0, 0, 0, 0, 1.05, true);
        }
    }
    if is_excute(agent) {
        LAST_EFFECT_SET_RATE(agent, 0.9);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("handl"), 1.0, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mariod_superjump_power"), Hash40::new("handl"), 1.2, 0, 0, 0, 0, 0, 1.45, true);
        LAST_EFFECT_SET_RATE(agent, 2);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_DETACH_KIND(agent, Hash40::new("mariod_superjump_fnish"), -1);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_superjump_power"), false, true);
    }
}

//Aerial Up Special Effect
unsafe extern "C" fn ssbexo_mariod_aerial_up_special_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 1.2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("mariod_smash_aura"), Hash40::new("handl"), 2.0, 0.0, 0.0, 0, 0, 0, 0.6, true);
    }
    frame(lua_state, 3.0);
    if sv_animcmd::get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mariod_superjump_fnish"), Hash40::new("handl"), 2.5, 0, 1, 0, 0, 0, 1.05, true);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mariod_superjump_fnish"), Hash40::new("handl"), 2.5, 1, 0, 0, 0, 0, 1.05, true);
        }
    }
    if is_excute(agent) {
        LAST_EFFECT_SET_RATE(agent, 0.9);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("handl"), 1.0, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("mariod_superjump_power"), Hash40::new("handl"), 1.2, 0, 0, 0, 0, 0, 1.45, true);
        LAST_EFFECT_SET_RATE(agent, 2);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("mariod_superjump_fnish"), -1);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mariod_superjump_power"), false, true);
    }
}

//Up Special Sound
unsafe extern "C" fn ssbexo_mariod_up_special_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark2"));
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark1"));
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mariod_special_h03"));
    }
}

pub fn install() {
    Agent::new("mariod")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialhi", ssbexo_mariod_up_special_acmd, Low)
    .acmd("game_specialairhi", ssbexo_mariod_up_special_acmd, Low)
    .acmd("effect_specialhi", ssbexo_mariod_grounded_up_special_effect, Low)
    .acmd("effect_specialairhi", ssbexo_mariod_aerial_up_special_effect, Low)
    .acmd("sound_specialhi", ssbexo_mariod_up_special_sound, Low)
    .acmd("sound_specialairhi", ssbexo_mariod_up_special_sound, Low)
    .install()
    ;
}