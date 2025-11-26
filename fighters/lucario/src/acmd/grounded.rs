use super::*;

//Up Taunt ACMD
unsafe extern "C" fn ssbexo_lucario_up_taunt_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_BONE, Hash40::new("appeal_hi_r"), false, -1.0);
    }
}

//Up Taunt Effect
unsafe extern "C" fn ssbexo_lucario_up_taunt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, -6, 0, 0, 0, 0, 0.25, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 6, 0, 0, 0, 0, 0.25, true);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, -6, 0, 0, 0, 0, 0.25, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 6, 0, 0, 0, 0, 0.25, true);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 20.0, -1, 0, 0, 90, 1, true, 0.6);
        LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(agent.lua_state_agent, 31.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 20.0, -1, 0, 0, 90, 1, true, 0.6);
        LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(agent.lua_state_agent, 36.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 20.0, -1, 0, 0, 90, 1, true, 0.6);
        LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(agent.lua_state_agent, 45.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, -6, 0, 0, 0, 0, 0.25, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
        EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("haver"), 0, 6, 0, 0, 0, 0, 0.25, true);
        FOOT_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

//Up Taunt Sound
unsafe extern "C" fn ssbexo_lucario_up_taunt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_lucario_attackair_n"));
    }
    frame(agent.lua_state_agent, 42.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_lucario_appeal_h01"));
        PLAY_SE(agent, Hash40::new("vc_lucario_appeal01"));
    }
}

//Up Taunt Expression
unsafe extern "C" fn ssbexo_lucario_up_taunt_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 32, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_lucario_dash_attack_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_FORCE_AURAPOWER_ATTACK_POWER_MUL);
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 4.0, 5.0);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.5, 50, 59, 0, 79, 3.8, 0.0, 4.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("footl"), 10.0, 50, 79, 0, 74, 3.6, 3.2, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 60, 69, 0, 74, 3.5, 0.0, 4.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("footl"), 7.0, 60, 74, 0, 74, 2.9, 3.2, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("lucario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_appealhir", ssbexo_lucario_up_taunt_acmd, Low)
    .effect_acmd("effect_appealhir", ssbexo_lucario_up_taunt_effect, Low)
    .sound_acmd("sound_appealhir", ssbexo_lucario_up_taunt_sound, Low)
    .expression_acmd("expression_appealhir", ssbexo_lucario_up_taunt_expression, Low)
    .game_acmd("game_appealhil", ssbexo_lucario_up_taunt_acmd, Low)
    .effect_acmd("effect_appealhil", ssbexo_lucario_up_taunt_effect, Low)
    .sound_acmd("sound_appealhil", ssbexo_lucario_up_taunt_sound, Low)
    .expression_acmd("expression_appealhil", ssbexo_lucario_up_taunt_expression, Low)
    .game_acmd("game_attackdash", ssbexo_lucario_dash_attack_acmd, Low)
    .install()
    ;
}