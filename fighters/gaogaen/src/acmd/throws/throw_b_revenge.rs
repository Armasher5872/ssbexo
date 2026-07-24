use super::*;

//Revenge Back Throw ACMD
unsafe extern "C" fn ssbexo_gaogaen_revenge_back_throw_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 0, 100, 0, 10, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_lay"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_THROW_CRITICAL_ZOOM);
        FT_CATCH_STOP(agent, 20, 1);
        CHECK_FINISH_CAMERA(agent, -10, 4);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
    }
}

//Revenge Back Throw Effect
unsafe extern "C" fn ssbexo_gaogaen_revenge_back_throw_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 20.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("hip"), 0, 3, 0.5, 0, 0, -90, 0.6, true, 1.7);
        LAST_EFFECT_SET_RATE(agent, 1.4);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("hip"), 0, 3, 0.5, 0, 90, -90, 0.8, false, 1.5);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 37.0);
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
    }
}

//Revenge Back Throw Sound
unsafe extern "C" fn ssbexo_gaogaen_revenge_back_throw_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_gaogaen_throw_b01"));
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_gaogaen_attack06"));
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_gaogaen_throw_b02"));
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        PLAY_DOWN_SE(agent, Hash40::new("se_gaogaen_throw_mat"));
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("se_gaogaen_react_success"));
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_gaogaen_throw_b03"));
    }
}

//Revenge Back Throw Expression
unsafe extern "C" fn ssbexo_gaogaen_revenge_back_throw_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x1f20a9d549), false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x24772eddef), false);
        agent.clear_lua_stack();
        lua_args!(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, *CAMERA_QUAKE_KIND_NONE);
        sv_animcmd::FT_ATTACK_ABS_CAMERA_QUAKE(lua_state);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
    }
}

pub fn install() {
    Agent::new("gaogaen")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_throwbrevenge", ssbexo_gaogaen_revenge_back_throw_acmd, Low)
    .acmd("effect_throwbrevenge", ssbexo_gaogaen_revenge_back_throw_effect, Low)
    .acmd("sound_throwbrevenge", ssbexo_gaogaen_revenge_back_throw_sound, Low)
    .acmd("expression_throwbrevenge", ssbexo_gaogaen_revenge_back_throw_expression, Low)
    .install()
    ;
}