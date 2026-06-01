use super::*;

unsafe extern "C" fn ssbexo_wario_kamikaze_burst_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 50.0, 60, 45, 0, 30, 30.0, 0.0, 0.0, 0.0, None, None, None, 4.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 361, 80, 150, 0, 35.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 1.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_MAGIC);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn ssbexo_wario_kamikaze_burst_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("wario_ppe_flash"), Hash40::new("top"), 0, 0, 0, 0, -90, 0, 0.35, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("wario_kamikaze_bomb"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("wario_kamikaze_bomb_ember"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("wario_kamikaze_bomb_ember"), Hash40::new("top"), 0, 0, 12, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("wario_kamikaze_bomb_ember"), Hash40::new("top"), 0, 0, 24, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("wario_kamikaze_bomb_ember"), Hash40::new("top"), 0, 0, -12, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("wario_kamikaze_bomb_ember"), Hash40::new("top"), 0, 0, -24, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        FILL_SCREEN_MODEL_COLOR(agent, 1, 4, 0, 0, 0, 0, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::CHAR, *EFFECT_SCREEN_PRIO_FINAL);
        FILL_SCREEN_MODEL_COLOR(agent, 2, 4, 0, 0, 0, 1, 1, 1, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, *EFFECT_SCREEN_PRIO_FINAL);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("wario_ppe_fly_3"), Hash40::new("top"), 0, 0, 0, 0, 90, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("wario_ppe_fly_2"), Hash40::new("top"), 0, 5, 0, 0, 90, 0, 1.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("wario_ppe_fly_2"), Hash40::new("top"), 0, 7.5, 0, 0, 90, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("wario_ppe_fly_2"), Hash40::new("top"), 0, 10, 0, 0, 90, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("wario_ppe_fly_2"), Hash40::new("top"), 0, 12.5, 0, 0, 90, 0, 1.4, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("wario_ppe_fly_2"), Hash40::new("top"), 0, 15, 0, 0, 90, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("wario_ppe_fly_2"), Hash40::new("top"), 0, 17.5, 0, 0, 90, 0, 1.35, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("wario_ppe_fly_2"), Hash40::new("top"), 0, 20, 0, 0, 90, 0, 1.4, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("wario_ppe_fly_2"), Hash40::new("top"), 0, 22.5, 0, 0, 90, 0, 1.45, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("wario_ppe_fly_2"), Hash40::new("top"), 0, 25, 0, 0, 90, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("wario_ppe_fly"), Hash40::new("top"), 0, 25, 0, 0, 90, 0, 1.25, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("wario_ppe_fly"), Hash40::new("top"), 0, 30, 0, 0, 90, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("wario_ppe_fly"), Hash40::new("top"), 0, 35, 0, 0, 90, 0, 1.25, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("wario_ppe_fly"), Hash40::new("top"), 0, 40, 0, 0, 90, 0, 1, 0, 0, 0, 0, 0, 0, true);
        CANCEL_FILL_SCREEN(agent, 1, 4);
        CANCEL_FILL_SCREEN(agent, 2, 4);
    }
}

unsafe extern "C" fn ssbexo_wario_kamikaze_burst_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_special_n01"));
    }
}

pub fn install() {
    Agent::new("wario_fireballcloned")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_kamikaze", ssbexo_wario_kamikaze_burst_acmd, Low)
    .effect_acmd("effect_kamikaze", ssbexo_wario_kamikaze_burst_effect, Low)
    .sound_acmd("sound_kamikaze", ssbexo_wario_kamikaze_burst_sound, Low)
    .install()
    ;
}