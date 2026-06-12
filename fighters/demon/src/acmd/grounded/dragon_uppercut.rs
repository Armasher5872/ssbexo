use super::*;

//Dragon Uppercut ACMD
unsafe extern "C" fn ssbexo_demon_dragon_uppercut_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    FT_MOTION_RATE(agent, 0.9);
    frame(lua_state, 7.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 19.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 22.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        AttackModule::set_damage_shake_scale(boma, 1.5);
        ATTACK(agent, 0, 0, Hash40::new("handl"), 18.0, 60, 65, 0, 80, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 18.0, 60, 65, 0, 80, 5.25, 0.0, 10.5, 4.75, Some(0.0), Some(16.0), Some(7.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 18.0, 60, 65, 0, 80, 5.25, 0.0, 5.0, 2.5, Some(0.0), Some(10.5), Some(4.75), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.5);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 0.5);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 0.5);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 2, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 2, false);
        AttackModule::set_damage_shake_scale(boma, 1.0);
        ATTACK(agent, 0, 0, Hash40::new("handl"), 16.0, 70, 60, 0, 80, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.1), 0.17, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 70, 60, 0, 80, 5.0, 0.0, 13.0, 3.0, Some(0.0), Some(23.0), Some(3.0), 0.17, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.5);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 0.5);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("handl"), 14.0, 70, 50, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.17, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 70, 50, 0, 60, 5.0, 0.0, 14.0, 3.0, Some(0.0), Some(24.0), Some(3.0), 0.17, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.5);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 0.5);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        HitModule::set_status_all(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
        ATTACK(agent, 0, 0, Hash40::new("handl"), 12.0, 70, 50, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 70, 50, 0, 60, 5.0, 0.0, 21.0, 3.0, Some(0.0), Some(24.0), Some(3.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 70, 50, 0, 60, 5.0, 0.0, 21.5, 3.0, Some(0.0), Some(24.5), Some(3.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 70, 50, 0, 60, 5.0, 0.0, 22.0, 3.0, Some(0.0), Some(25.0), Some(3.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(lua_state, 28.0);
    FT_MOTION_RATE(agent, 0.9);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//Dragon Uppercut Effect
unsafe extern "C" fn ssbexo_demon_dragon_uppercut_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("demon_atkhi3_arc"), Hash40::new("top"), 1.7, 15.5, 0.5, 0, -32, 115, 0.95, true, 1);
        LAST_EFFECT_SET_RATE(agent, 1.7);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Dragon Uppercut Sound
unsafe extern "C" fn ssbexo_demon_dragon_uppercut_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_demon_attack02"));
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_demon_step_left_l"));
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_demon_swing_short02"));
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_demon_attack09"));
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_demon_step_left_l"));
    }
}

//Dragon Uppercut Expression
unsafe extern "C" fn ssbexo_demon_dragon_uppercut_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        RUMBLE_HIT(agent, Hash40::new("rbkind_80_attack_special_t"), 0);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_80_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_attackstep2l", ssbexo_demon_dragon_uppercut_acmd, Low)
    .acmd("effect_attackstep2l", ssbexo_demon_dragon_uppercut_effect, Low)
    .acmd("sound_attackstep2l", ssbexo_demon_dragon_uppercut_sound, Low)
    .acmd("expression_attackstep2l", ssbexo_demon_dragon_uppercut_expression, Low)
    .install()
    ;
}