use super::*;

//Neutral Special ACMD
unsafe extern "C" fn ssbexo_wario_neutral_special_acmd(agent: &mut L2CAgentBase) {
    FT_MOTION_RATE(agent, 13.0/14.0);
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        SEARCH(agent, 0, 0, Hash40::new("top"), 8.0, 0.0, 8.0, 10.0, None, None, None, *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
        SEARCH(agent, 1, 0, Hash40::new("top"), 8.0, 0.0, 8.0, 10.0, None, None, None, *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ITEM, *COLLISION_PART_MASK_ALL, false);
        search!(agent, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
        CATCH(agent, 0, Hash40::new("top"), 8.0, 0.0, 8.0, 10.0, None, None, None, *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_START, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Neutral Special Effect
unsafe extern "C" fn ssbexo_wario_neutral_special_effect(_agent: &mut L2CAgentBase) {
}

//Neutral Special Sound
unsafe extern "C" fn ssbexo_wario_neutral_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_swing_l"));
    }
}

//Neutral Special Expression
unsafe extern "C" fn ssbexo_wario_neutral_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 53.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialn", ssbexo_wario_neutral_special_acmd, Low)
    .effect_acmd("effect_specialn", ssbexo_wario_neutral_special_effect, Low)
    .sound_acmd("sound_specialn", ssbexo_wario_neutral_special_sound, Low)
    .expression_acmd("expression_specialn", ssbexo_wario_neutral_special_expression, Low)
    .game_acmd("game_specialairn", ssbexo_wario_neutral_special_acmd, Low)
    .effect_acmd("effect_specialairn", ssbexo_wario_neutral_special_effect, Low)
    .sound_acmd("sound_specialairn", ssbexo_wario_neutral_special_sound, Low)
    .expression_acmd("expression_specialairn", ssbexo_wario_neutral_special_expression, Low)
    .install()
    ;
}