use super::*;

//Revenge Up Throw ACMD
unsafe extern "C" fn ssbexo_gaogaen_revenge_up_throw_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 90, 70, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 46.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_THROW_CRITICAL_ZOOM);
        FT_CATCH_STOP(agent, 30, 1);
        CHECK_FINISH_CAMERA(agent, 1, 20);
    }
    frame(lua_state, 47.0);
    if is_excute(agent) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

//Revenge Up Throw Effect
unsafe extern "C" fn ssbexo_gaogaen_revenge_up_throw_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 8.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("bust"), 0.0, 0, 0, 0, 0, 0, 0.5, true);
        LAST_EFFECT_SET_RATE(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 1.0, 0, 0, 0, 0, 0, 0.5, true);
        LAST_EFFECT_SET_RATE(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("arml"), 7.0, 0, 0, 0, 0, 0, 0.5, true);
        LAST_EFFECT_SET_RATE(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 1.0, 0, 0, 0, 0, 0, 0.5, true);
        LAST_EFFECT_SET_RATE(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("armr"), 7.0, 0, 0, 0, 0, 0, 0.5, true);
        LAST_EFFECT_SET_RATE(agent, 0.5);
        EFFECT_FOLLOW(agent, Hash40::new("gaogaen_belt_fire_appeal"), Hash40::new("feeler"), 0, 3, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_RATE(agent, 1.25);
        EFFECT(agent, Hash40::new("sys_hit_fire"), Hash40::new("hip"), 0.0, 8.0, 10.0, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.25);
        EFFECT(agent, Hash40::new("gaogaen_revenge_start"), Hash40::new("hip"), -2, 10, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        EFFECT_FLIP_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 64, 0, 90, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.5);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("gaogaen_throw_hi"), Hash40::new("top"), 0, 16, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Revenge Up Throw Sound
unsafe extern "C" fn ssbexo_gaogaen_revenge_up_throw_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_gaogaen_rnd_attack"));
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_gaogaen_rnd_attackappeal01"));
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_kick_hit_ll"));
    }
}

//Revenge Up Throw Expression
unsafe extern "C" fn ssbexo_gaogaen_revenge_up_throw_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        agent.clear_lua_stack();
        lua_args!(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, *CAMERA_QUAKE_KIND_NONE);
        sv_animcmd::FT_ATTACK_ABS_CAMERA_QUAKE(lua_state);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
    frame(lua_state, 47.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattack"), 16, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("gaogaen")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_throwhirevenge", ssbexo_gaogaen_revenge_up_throw_acmd, Low)
    .acmd("effect_throwhirevenge", ssbexo_gaogaen_revenge_up_throw_effect, Low)
    .acmd("sound_throwhirevenge", ssbexo_gaogaen_revenge_up_throw_sound, Low)
    .acmd("expression_throwhirevenge", ssbexo_gaogaen_revenge_up_throw_expression, Low)
    .install()
    ;
}