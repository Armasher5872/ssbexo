use super::*;

//Cannonball Shoot ACMD
unsafe extern "C" fn ssbexo_koopajr_cannonball_shoot_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 361, 60, 0, 90, 3.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bomb"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}

//Cannonball Shoot Effect
unsafe extern "C" fn ssbexo_koopajr_cannonball_shoot_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new_raw(0x13BF90E65D), Hash40::new("top"), 0, 0, 0.0, 90, 0, 0, 0.1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new_raw(0x13BF90E65D), false, true);
        EFFECT_FOLLOW(agent, Hash40::new_raw(0x13BF90E65D), Hash40::new("top"), 0, 0, 0.0, 90, 0, 0, 0.1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 3.0);
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            EFFECT_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 0.0, -3.0, 0.0, 180.0, 0.0, 0.35, 0.0, 5.0, 15.0, 0.0, 0.0, 0.0, true, 0.30000001192092896);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 4.0);
    }
}

//Down Special ACMD
unsafe extern "C" fn ssbexo_koopajr_down_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        ArticleModule::generate_article_enable(agent.module_accessor, *FIGHTER_KOOPAJR_GENERATE_ARTICLE_MECHAKOOPA, false, -1);
    }
}

pub fn install() {
    Agent::new("koopajr")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallw", ssbexo_koopajr_down_special_acmd, Low)
    .game_acmd("game_specialairlw", ssbexo_koopajr_down_special_acmd, Low)
    .install()
    ;
    Agent::new("koopajr_cannonball")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_shoot", ssbexo_koopajr_cannonball_shoot_acmd, Low)
    .effect_acmd("effect_shoot", ssbexo_koopajr_cannonball_shoot_effect, Low)
    .install()
    ;
}