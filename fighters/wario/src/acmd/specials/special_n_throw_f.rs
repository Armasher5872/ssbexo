use super::*;

//Neutral Special Forward Throw ACMD
unsafe extern "C" fn ssbexo_wario_neutral_special_forward_throw_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 56, 55, 0, 73, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        FT_MOTION_RATE(agent, 16.0/18.0);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 16, 9);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        WorkModule::on_flag(boma, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_N_THROW);
    }
}

//Neutral Special Forward Throw Effect
unsafe extern "C" fn ssbexo_wario_neutral_special_forward_throw_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 5, 10, 1, 0, -90, -130, 1, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 15, 12, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

//Neutral Special Forward Throw Sound
unsafe extern "C" fn ssbexo_wario_neutral_special_forward_throw_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_02"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_wario_rnd_attack"));
    }
}

//Neutral Special Forward Throw Expression
unsafe extern "C" fn ssbexo_wario_neutral_special_forward_throw_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialnthrowf", ssbexo_wario_neutral_special_forward_throw_acmd, Low)
    .acmd("effect_specialnthrowf", ssbexo_wario_neutral_special_forward_throw_effect, Low)
    .acmd("sound_specialnthrowf", ssbexo_wario_neutral_special_forward_throw_sound, Low)
    .acmd("expression_specialnthrowf", ssbexo_wario_neutral_special_forward_throw_expression, Low)
    .acmd("game_specialairnthrowf", ssbexo_wario_neutral_special_forward_throw_acmd, Low)
    .acmd("effect_specialairnthrowf", ssbexo_wario_neutral_special_forward_throw_effect, Low)
    .acmd("sound_specialairnthrowf", ssbexo_wario_neutral_special_forward_throw_sound, Low)
    .acmd("expression_specialairnthrowf", ssbexo_wario_neutral_special_forward_throw_expression, Low)
    .install()
    ;
}