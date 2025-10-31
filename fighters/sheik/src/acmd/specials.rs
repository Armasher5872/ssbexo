use super::*;

//Side Special ACMD
unsafe extern "C" fn ssbexo_sheik_side_special_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SHEIK_GENERATE_ARTICLE_KNIFE, false, -1);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_SHEIK_GENERATE_ARTICLE_KNIFE, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("throw"), 3.0, 0, 0, 0, 0, 2.0, 0.0, 0.0, 0.0, Some(0.0), Some(4.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("throw"), 6.0, 166, 25, 0, 65, 6.0, 0.0, 0.0, 0.0, Some(0.0), Some(4.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 31.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_SHEIK_GENERATE_ARTICLE_KNIFE, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_SHEIK_GENERATE_ARTICLE_KNIFE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Grounded Side Special Effect
unsafe extern "C" fn ssbexo_sheik_grounded_side_special_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sheik_sakuretu_pin_bomb"), Hash40::new("throw"), 0, 0.6600000262260437, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sheik_sakuretu_pin_bomb"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("sheik_sakuretu_pin"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 33.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Side Special Effect
unsafe extern "C" fn ssbexo_sheik_aerial_side_special_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sheik_sakuretu_pin_bomb"), Hash40::new("throw"), 0, 0.6600000262260437, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sheik_sakuretu_pin_bomb"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("sheik_sakuretu_pin"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
    }
}

//Side Special Sound
unsafe extern "C" fn ssbexo_sheik_side_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_sheik_special_s01"));
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_sheik_special_s03"));
    }
}

//Grounded Side Special Expression
unsafe extern "C" fn ssbexo_sheik_grounded_side_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_grapple"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 33.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(agent.lua_state_agent, 38.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(agent.lua_state_agent, 41.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
}

//Aerial Side Special Expression
unsafe extern "C" fn ssbexo_sheik_aerial_side_special_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_grapple"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("sheik")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specials", ssbexo_sheik_side_special_acmd, Low)
    .effect_acmd("effect_specials", ssbexo_sheik_grounded_side_special_effect, Low)
    .sound_acmd("sound_specials", ssbexo_sheik_side_special_sound, Low)
    .expression_acmd("expression_specials", ssbexo_sheik_grounded_side_special_expression, Low)
    .game_acmd("game_specialairs", ssbexo_sheik_side_special_acmd, Low)
    .effect_acmd("effect_specialairs", ssbexo_sheik_aerial_side_special_effect, Low)
    .sound_acmd("sound_specialairs", ssbexo_sheik_side_special_sound, Low)
    .expression_acmd("expression_specialairs", ssbexo_sheik_aerial_side_special_expression, Low)
    .install()
    ;
}