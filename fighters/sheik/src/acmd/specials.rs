use super::*;

//Side Special ACMD
unsafe extern "C" fn ssbexo_sheik_side_special_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_SHEIK_GENERATE_ARTICLE_KNIFE, false, -1);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_SHEIK_GENERATE_ARTICLE_KNIFE, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("throw"), 3.0, 0, 0, 0, 0, 2.0, 0.0, 0.0, 0.0, Some(0.0), Some(4.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("throw"), 6.0, 166, 25, 0, 65, 6.0, 0.0, 0.0, 0.0, Some(0.0), Some(4.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 58.0);
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_SHEIK_GENERATE_ARTICLE_KNIFE, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::remove_exist(boma, *FIGHTER_SHEIK_GENERATE_ARTICLE_KNIFE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Grounded Side Special Effect
unsafe extern "C" fn ssbexo_sheik_grounded_side_special_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sheik_sakuretu_pin_bomb"), Hash40::new("throw"), 0, 0.6600000262260437, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sheik_sakuretu_pin_bomb"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("sheik_sakuretu_pin"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Side Special Effect
unsafe extern "C" fn ssbexo_sheik_aerial_side_special_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sheik_sakuretu_pin_bomb"), Hash40::new("throw"), 0, 0.6600000262260437, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sheik_sakuretu_pin_bomb"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("sheik_sakuretu_pin"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
    }
}

//Side Special Sound
unsafe extern "C" fn ssbexo_sheik_side_special_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_sheik_special_s01"));
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_sheik_special_s03"));
    }
}

//Grounded Side Special Expression
unsafe extern "C" fn ssbexo_sheik_grounded_side_special_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_grapple"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(lua_state, 46.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(lua_state, 49.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
}

//Aerial Side Special Expression
unsafe extern "C" fn ssbexo_sheik_aerial_side_special_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 24.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_grapple"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("sheik")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specials", ssbexo_sheik_side_special_acmd, Low)
    .acmd("effect_specials", ssbexo_sheik_grounded_side_special_effect, Low)
    .acmd("sound_specials", ssbexo_sheik_side_special_sound, Low)
    .acmd("expression_specials", ssbexo_sheik_grounded_side_special_expression, Low)
    .acmd("game_specialairs", ssbexo_sheik_side_special_acmd, Low)
    .acmd("effect_specialairs", ssbexo_sheik_aerial_side_special_effect, Low)
    .acmd("sound_specialairs", ssbexo_sheik_side_special_sound, Low)
    .acmd("expression_specialairs", ssbexo_sheik_aerial_side_special_expression, Low)
    .install()
    ;
}