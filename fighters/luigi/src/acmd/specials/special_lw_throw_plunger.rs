use super::*;

//Down Special Plunger Throw ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_throw_plunger_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 30, 65, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 18, 4);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.0);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 12.0, y: -1.0, z: 0.0});
        FT_CATCH_STOP(agent, 5, 1);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_THROW);
    }
}

//Down Special Plunger Throw Effect
unsafe extern "C" fn ssbexo_luigi_down_special_throw_plunger_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, -3, 3, 0, 20, -90, 1.7, true, *EF_FLIP_YZ, 0.4);
        EffectModule::set_disable_render_offset_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 17, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 17, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_spin_wind"), false, false);
    }
}

//Down Special Plunger Throw Sound
unsafe extern "C" fn ssbexo_luigi_down_special_throw_plunger_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    wait(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_02"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_luigi_rnd_attack"));
    }
}

//Down Special Plunger Throw Expression
unsafe extern "C" fn ssbexo_luigi_down_special_throw_plunger_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallwthrowplunger", ssbexo_luigi_down_special_throw_plunger_acmd, Low)
    .game_acmd("game_specialairlwthrowplunger", ssbexo_luigi_down_special_throw_plunger_acmd, Low)
    .effect_acmd("effect_speciallwthrowplunger", ssbexo_luigi_down_special_throw_plunger_effect, Low)
    .effect_acmd("effect_specialairlwthrowplunger", ssbexo_luigi_down_special_throw_plunger_effect, Low)
    .sound_acmd("sound_speciallwthrowplunger", ssbexo_luigi_down_special_throw_plunger_sound, Low)
    .sound_acmd("sound_specialairlwthrowplunger", ssbexo_luigi_down_special_throw_plunger_sound, Low)
    .expression_acmd("expression_speciallwthrowplunger", ssbexo_luigi_down_special_throw_plunger_expression, Low)
    .expression_acmd("expression_specialairlwthrowplunger", ssbexo_luigi_down_special_throw_plunger_expression, Low)
    .install()
    ;
}