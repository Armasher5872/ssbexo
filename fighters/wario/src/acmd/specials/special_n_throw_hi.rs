use super::*;

//Neutral Special Up Throw ACMD
unsafe extern "C" fn ssbexo_wario_neutral_special_up_throw_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.8, 70, 65, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 0, 18);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_N_THROW);
    }
}

//Neutral Special Up Throw Effect
unsafe extern "C" fn ssbexo_wario_neutral_special_up_throw_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 15, 12, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}

//Neutral Special Up Throw Sound
unsafe extern "C" fn ssbexo_wario_neutral_special_up_throw_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_02"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_wario_rnd_attack"));
    }
}

//Neutral Special Up Throw Expression
unsafe extern "C" fn ssbexo_wario_neutral_special_up_throw_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialnthrowhi", ssbexo_wario_neutral_special_up_throw_acmd, Low)
    .effect_acmd("effect_specialnthrowhi", ssbexo_wario_neutral_special_up_throw_effect, Low)
    .sound_acmd("sound_specialnthrowhi", ssbexo_wario_neutral_special_up_throw_sound, Low)
    .expression_acmd("expression_specialnthrowhi", ssbexo_wario_neutral_special_up_throw_expression, Low)
    .game_acmd("game_specialairnthrowhi", ssbexo_wario_neutral_special_up_throw_acmd, Low)
    .effect_acmd("effect_specialairnthrowhi", ssbexo_wario_neutral_special_up_throw_effect, Low)
    .sound_acmd("sound_specialairnthrowhi", ssbexo_wario_neutral_special_up_throw_sound, Low)
    .expression_acmd("expression_specialairnthrowhi", ssbexo_wario_neutral_special_up_throw_expression, Low)
    .install()
    ;
}