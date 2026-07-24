use super::*;

//Iron Tail ACMD
unsafe extern "C" fn ssbexo_pikachu_side_special_attack_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 5.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 30, 60, 0, 60, 7.0, 0.0, 8.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_TAIL);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 30, 60, 0, 60, 7.0, 0.0, 6.0, 2.0, Some(0.0), Some(6.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_TAIL);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 30, 60, 0, 60, 7.0, 0.0, 4.0, 4.0, Some(0.0), Some(2.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_TAIL);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, FIGHTER_PIKACHU_GENERATE_ARTICLE_TAIL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        WorkModule::on_flag(boma, *FIGHTER_PIKACHU_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ENABLE_LANDING);
    }
}

//Grounded Iron Tail Effect
unsafe extern "C" fn ssbexo_pikachu_grounded_side_special_attack_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, FIGHTER_PIKACHU_GENERATE_ARTICLE_TAIL) {
            let tail_boma = get_article_boma(boma, FIGHTER_PIKACHU_GENERATE_ARTICLE_TAIL);
            let tail_agent = get_weapon_common_from_accessor(&mut *tail_boma);
            EFFECT(tail_agent, Hash40::new("sys_smash_flash_s"), Hash40::new("tail3"), 4, 0, 4, 0, 0, 0, 1.5, 4, 4, 4, 0, 0, 0, true);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 6.0, 0, 0, 0, -90, 0.6, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 2.0);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 6.0, 0, 0, 0, -90, 0.8, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 2.0);
        if ArticleModule::is_exist(boma, FIGHTER_PIKACHU_GENERATE_ARTICLE_TAIL) {
            let tail_boma = get_article_boma(boma, FIGHTER_PIKACHU_GENERATE_ARTICLE_TAIL);
            let tail_agent = get_weapon_common_from_accessor(&mut *tail_boma);
            AFTER_IMAGE4_ON_arg29(tail_agent, Hash40::new("tex_pikachu_met_tail1"), Hash40::new("tex_pikachu_met_tail2"), 6, Hash40::new("tail4"), 0, 0, 0, Hash40::new("tail4"), 0, 0, 6.0, true, Hash40::new("null"), Hash40::new("tail4"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, FIGHTER_PIKACHU_GENERATE_ARTICLE_TAIL) {
            let tail_boma = get_article_boma(boma, FIGHTER_PIKACHU_GENERATE_ARTICLE_TAIL);
            let tail_agent = get_weapon_common_from_accessor(&mut *tail_boma);
            AFTER_IMAGE_OFF(tail_agent, 2);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Iron Tail Effect
unsafe extern "C" fn ssbexo_pikachu_aerial_side_special_attack_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, FIGHTER_PIKACHU_GENERATE_ARTICLE_TAIL) {
            let tail_boma = get_article_boma(boma, FIGHTER_PIKACHU_GENERATE_ARTICLE_TAIL);
            let tail_agent = get_weapon_common_from_accessor(&mut *tail_boma);
            EFFECT(tail_agent, Hash40::new("sys_smash_flash_s"), Hash40::new("tail3"), 4, 0, 4, 0, 0, 0, 1.5, 4, 4, 4, 0, 0, 0, true);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 6.0, 0, 0, 0, -90, 0.6, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 2.0);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 6.0, 0, 0, 0, -90, 0.8, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 2.0);
        if ArticleModule::is_exist(boma, FIGHTER_PIKACHU_GENERATE_ARTICLE_TAIL) {
            let tail_boma = get_article_boma(boma, FIGHTER_PIKACHU_GENERATE_ARTICLE_TAIL);
            let tail_agent = get_weapon_common_from_accessor(&mut *tail_boma);
            AFTER_IMAGE4_ON_arg29(tail_agent, Hash40::new("tex_pikachu_met_tail1"), Hash40::new("tex_pikachu_met_tail2"), 6, Hash40::new("tail4"), 0, 0, 0, Hash40::new("tail4"), 0, 0, 6.0, true, Hash40::new("null"), Hash40::new("tail4"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, FIGHTER_PIKACHU_GENERATE_ARTICLE_TAIL) {
            let tail_boma = get_article_boma(boma, FIGHTER_PIKACHU_GENERATE_ARTICLE_TAIL);
            let tail_agent = get_weapon_common_from_accessor(&mut *tail_boma);
            AFTER_IMAGE_OFF(tail_agent, 2);
        }
    }
}

//Grounded Iron Tail Sound
unsafe extern "C" fn ssbexo_pikachu_grounded_side_special_attack_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pikachu_special_s01"));
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_pikachu_attack06"));
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        PLAY_LANDING_SE(agent, Hash40::new("se_pikachu_landing02"));
    }
}

//Aerial Iron Tail Sound
unsafe extern "C" fn ssbexo_pikachu_aerial_side_special_attack_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pikachu_special_s01"));
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_pikachu_attack06"));
    }
}

//Grounded Iron Tail Expression
unsafe extern "C" fn ssbexo_pikachu_grounded_side_special_attack_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 7, true, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 7, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Aerial Iron Tail Expression
unsafe extern "C" fn ssbexo_pikachu_aerial_side_special_attack_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 7, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 7, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

pub fn install() {
    Agent::new("pikachu")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specials", ssbexo_pikachu_side_special_attack_acmd, Low)
    .acmd("game_specialairs", ssbexo_pikachu_side_special_attack_acmd, Low)
    .acmd("effect_specials", ssbexo_pikachu_grounded_side_special_attack_effect, Low)
    .acmd("effect_specialairs", ssbexo_pikachu_aerial_side_special_attack_effect, Low)
    .acmd("sound_specials", ssbexo_pikachu_grounded_side_special_attack_sound, Low)
    .acmd("sound_specialairs", ssbexo_pikachu_aerial_side_special_attack_sound, Low)
    .acmd("expression_specials", ssbexo_pikachu_grounded_side_special_attack_expression, Low)
    .acmd("expression_specialairs", ssbexo_pikachu_aerial_side_special_attack_expression, Low)
    .install()
    ;
}