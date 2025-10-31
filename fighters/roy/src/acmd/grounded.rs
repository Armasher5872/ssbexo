use super::*;

//Jab 1 Effect
unsafe extern "C" fn ssbexo_roy_jab_1_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 7, Hash40::new("sword1"), 0.0, 0.0, -0.8, Hash40::new("sword1"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("roy_sword"), false, false);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_roy_dash_attack_acmd(agent: &mut L2CAgentBase) {
    MotionModule::set_rate(agent.module_accessor, 1.444);
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 52, 85, 0, 65, 4.4, 0.0, 6.5, 12.5, Some(0.0), Some(6.5), Some(6.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 361, 60, 0, 60, 4.4, 0.0, 6.5, 18.5, Some(0.0), Some(6.5), Some(6.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        ATK_SET_SHIELD_SETOFF_MUL_arg4(agent, 0, 1, 2, 1.25);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Dash Attack Effect
unsafe extern "C" fn ssbexo_roy_dash_attack_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 9, Hash40::new("sword1"), 0, 0, 0.7, Hash40::new("sword1"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("roy_sword_light"), Hash40::new("sword1"), 0, 0, 10.55, 0, 0, 0, 0.7, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        LAST_EFFECT_SET_ALPHA(agent, 0.6);
        EFFECT_FOLLOW(agent, Hash40::new("roy_sword_light"), Hash40::new("sword1"), 0, 0, 7, 0, 0, 0, 0.7, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        LAST_EFFECT_SET_ALPHA(agent, 0.6);
        EFFECT_FOLLOW(agent, Hash40::new("roy_sword_light"), Hash40::new("sword1"), 0, 0, 8.5, 0, 0, 0, 0.7, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
}

pub fn install() {
    Agent::new("roy")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .effect_acmd("effect_attack11", ssbexo_roy_jab_1_effect, Low)
    .game_acmd("game_attackdash", ssbexo_roy_dash_attack_acmd, Low)
    .effect_acmd("effect_attackdash", ssbexo_roy_dash_attack_effect, Low)
    .install()
    ;
}