use super::*;

//Up Special ACMD
unsafe extern "C" fn ssbexo_donkey_up_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 270, 100, 0, 30, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bomb"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 80, 50, 0, 50, 10.0, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        ATK_POWER(agent, 1, 6.0);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_BARREL_CANNON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ATK_POWER(agent, 1, 4.0);
    }
    frame(agent.lua_state_agent, 45.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Up Special Effect
unsafe extern "C" fn ssbexo_donkey_up_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("donkey_entry"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//Up Special Sound
unsafe extern "C" fn ssbexo_donkey_up_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_donkey_appear01"));
    }
    wait(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
    wait(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
    wait(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
    wait(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
    wait(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
}

pub fn install() {
    Agent::new("donkey")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialhi", ssbexo_donkey_up_special_acmd, Low)
    .game_acmd("game_specialairhi", ssbexo_donkey_up_special_acmd, Low)
    .effect_acmd("effect_specialhi", ssbexo_donkey_up_special_effect, Low)
    .effect_acmd("effect_specialairhi", ssbexo_donkey_up_special_effect, Low)
    .sound_acmd("sound_specialhi", ssbexo_donkey_up_special_sound, Low)
    .sound_acmd("sound_specialairhi", ssbexo_donkey_up_special_sound, Low)
    .install()
    ;
}