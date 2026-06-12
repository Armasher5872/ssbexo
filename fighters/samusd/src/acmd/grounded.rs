use super::*;

//Entry Effect
unsafe extern "C" fn ssbexo_samusd_entry_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_water_walk"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.8);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        ColorBlendModule::set_disable_camera_depth_influence(boma, true);
        FLASH(agent, 0.012, 0.012, 0.032, 0.9);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 25, 0.012, 0.012, 0.032, 0);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_ripple"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.8);
    }
    frame(lua_state, 53.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        ColorBlendModule::set_disable_camera_depth_influence(boma, false);
    }
    frame(lua_state, 75.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_splash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.8);
    }
}

//Down Taunt Effect
unsafe extern "C" fn ssbexo_samusd_down_taunt_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), -2.0, 0.0, 0.0, 0, 0, 0, 2.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new_raw(0x09aee445d1), 2.0, 0.0, 0.5, 0, 0, 0, 2.0, true);
        EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), 0.0, 0.0, -0.5, 0, 0, 0, 1.70000005, true);
        EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), 0.0, 0.0, 0.0, 0, 0, 0, 2.0999999, true);
        EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0.0, 0.0, 0.0, 0, 0, 0, 1.89999998, true);
        EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0.0, 0.0, 0.0, 0, 0, 0, 2.0, true);
        EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new_raw(0x0954eb78b2), 2.0, 0.0, -0.5, 0, 0, 0, 2.0, true);
        EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), 0.0, 0.0, 0.0, 0, 0, 0, 1.70000005, true);
        EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), 0.0, 0.0, 0.0, 0, 0, 0, 2.0999999, true);
        EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0.0, 0.0, 0.0, 0, 0, 0, 1.89999998, true);
        EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0.0, 0.0, 0.0, 0, 0, 0, 1.89999998, true);
        BURN_COLOR(agent, 0.26, 0.71, 1.5, 0.7);
    }
    frame(lua_state, 80.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("samusd_win3_aura"), false, true);
        BURN_COLOR_FRAME(agent, 20, 1, 1, 1, 0);
        BURN_COLOR_NORMAL(agent);
    }
}

//Down Taunt Sound
unsafe extern "C" fn ssbexo_samusd_down_taunt_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_samusd_appeal_l02"));
    }
}

//Jab Effect
unsafe extern "C" fn ssbexo_samusd_jab_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0.0, 0.0, 0.0, 0, 0, 0, 1.89999998, true);
        EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0.0, 0.0, 0.0, 0, 0, 0, 2.0, true);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 0.95, true);
        LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.8);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), -1, 10, 15, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 360, true);
        LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.8);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("samusd_win3_aura"), false, true);
        BURN_COLOR_FRAME(agent, 20, 1, 1, 1, 0);
        BURN_COLOR_NORMAL(agent);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_samusd_dash_attack_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("bust"), 1.0, 364, 80, 40, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 1.0, 364, 80, 40, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 2, 0, Hash40::new("arml"), 1.0, 364, 80, 40, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("bust"), 1.0, 364, 80, 40, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 1.0, 364, 80, 40, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 2, 0, Hash40::new("arml"), 1.0, 364, 80, 40, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("bust"), 1.0, 364, 80, 40, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 1.0, 364, 80, 40, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 2, 0, Hash40::new("arml"), 1.0, 364, 80, 40, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("bust"), 1.0, 364, 80, 40, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 1.0, 364, 80, 40, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        ATTACK(agent, 2, 0, Hash40::new("arml"), 1.0, 364, 80, 40, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("bust"), 3.0, 70, 80, 0, 60, 8.0, 0.0, -4.0, 0.0, Some(0.0), Some(4.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        AttackModule::set_poison_param(boma, 0, 300, 15, 0.5, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MotionModule::set_rate(boma, 0.75);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
    }
}

//Dash Attack Effect
unsafe extern "C" fn ssbexo_samusd_dash_attack_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), -2.0, 0.0, 0.0, 0, 0, 0, 2.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new_raw(0x09aee445d1), 2.0, 0.0, 0.5, 0, 0, 0, 2.0, true);
        EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), 0.0, 0.0, -0.5, 0, 0, 0, 1.70000005, true);
        EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), 0.0, 0.0, 0.0, 0, 0, 0, 2.0999999, true);
        EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0.0, 0.0, 0.0, 0, 0, 0, 1.89999998, true);
        EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0.0, 0.0, 0.0, 0, 0, 0, 2.0, true);
        EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new_raw(0x0954eb78b2), 2.0, 0.0, -0.5, 0, 0, 0, 2.0, true);
        EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), 0.0, 0.0, 0.0, 0, 0, 0, 1.70000005, true);
        EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), 0.0, 0.0, 0.0, 0, 0, 0, 2.0999999, true);
        EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0.0, 0.0, 0.0, 0, 0, 0, 1.89999998, true);
        EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0.0, 0.0, 0.0, 0, 0, 0, 1.89999998, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(agent, Hash40::new("samusd_dash_attack"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.8);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("samusd_atk_bomb"), Hash40::new("bust"), 0.0, 0.0, 0.0, 0, 0, 0, 1.65, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("samusd_win3_aura"), false, true);
        BURN_COLOR_FRAME(agent, 20, 1, 1, 1, 0);
        BURN_COLOR_NORMAL(agent);
    }
}

pub fn install() {
    Agent::new("samusd")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("effect_entryr", ssbexo_samusd_entry_effect, Low)
    .acmd("effect_entryl", ssbexo_samusd_entry_effect, Low)
    .acmd("effect_appeallwr", ssbexo_samusd_down_taunt_effect, Low)
    .acmd("effect_appeallwl", ssbexo_samusd_down_taunt_effect, Low)
    .acmd("sound_appeallwr", ssbexo_samusd_down_taunt_sound, Low)
    .acmd("sound_appeallwl", ssbexo_samusd_down_taunt_sound, Low)
    .acmd("effect_attack11", ssbexo_samusd_jab_effect, Low)
    .acmd("game_attackdash", ssbexo_samusd_dash_attack_acmd, Low)
    .acmd("effect_attackdash", ssbexo_samusd_dash_attack_effect, Low)
    .install()
    ;
}