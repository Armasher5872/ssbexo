use super::*;

//Grounded Neutral Special Attack ACMD
unsafe extern "C" fn ssbexo_mario_grounded_neutral_special_attack_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("arml"), 12.0, 361, 95, 0, 25, 2.0, -1.0, -3.0, 0.0, Some(-3.0), Some(-3.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 16.0, 361, 90, 0, 25, 5.0, 5.4, -3.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//Aerial Neutral Special Attack ACMD
unsafe extern "C" fn ssbexo_mario_aerial_neutral_special_attack_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("arml"), 12.0, 361, 95, 0, 25, 2.0, -1.0, -3.0, 0.0, Some(-3.0), Some(-3.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("arml"), 16.0, 361, 90, 0, 25, 5.0, 5.4, -3.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        KineticModule::add_speed(boma, &Vector3f{x: -1.0, y: 0.0, z: 0.0});
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//Grounded Neutral Special Attack Effect
unsafe extern "C" fn ssbexo_mario_grounded_neutral_special_attack_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 2.0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 7, -1, 0, -2, 0, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, -45, 0, 1, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_flame"), Hash40::new("top"), 0, 8, 12, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 360, true);
        EFFECT(agent, Hash40::new("mario_fb_bullet_r"), Hash40::new("top"), 0, 8, 12, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 360, true);
        FLASH(agent, 1, 0, 0, 0.353);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mario_fb_bullet_r"), false, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mario_fb_shoot"), false, true);
    }
}

//Aerial Neutral Special Attack Effect
unsafe extern "C" fn ssbexo_mario_aerial_neutral_special_attack_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 2.0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 7, -1, 0, -2, 0, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, -45, 0, 1, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_flame"), Hash40::new("top"), 0, 8, 12, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 360, true);
        EFFECT_FOLLOW(agent, Hash40::new("mario_fb_bullet_r"), Hash40::new("handl"), 0.75, -1, 0, 0, 0, 0, 1.6, true);
        FLASH(agent, 1, 0, 0, 0.353);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mario_fb_bullet_r"), false, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("mario_fb_shoot"), false, true);
    }
}

//Neutral Special Attack Sound
unsafe extern "C" fn ssbexo_mario_neutral_special_attack_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_mario_attack05"));
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mario_smash_s01"));
    }
}

//Neutral Special Attack Expression
unsafe extern "C" fn ssbexo_mario_neutral_special_attack_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_explosion"), 16, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

pub fn install() {
    Agent::new("mario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialnattack", ssbexo_mario_grounded_neutral_special_attack_acmd, Low)
    .game_acmd("game_specialairnattack", ssbexo_mario_aerial_neutral_special_attack_acmd, Low)
    .effect_acmd("effect_specialnattack", ssbexo_mario_grounded_neutral_special_attack_effect, Low)
    .effect_acmd("effect_specialairnattack", ssbexo_mario_aerial_neutral_special_attack_effect, Low)
    .sound_acmd("sound_specialnattack", ssbexo_mario_neutral_special_attack_sound, Low)
    .sound_acmd("sound_specialairnattack", ssbexo_mario_neutral_special_attack_sound, Low)
    .expression_acmd("expression_specialnattack", ssbexo_mario_neutral_special_attack_expression, Low)
    .expression_acmd("expression_specialairnattack", ssbexo_mario_neutral_special_attack_expression, Low)
    .install()
    ;
}