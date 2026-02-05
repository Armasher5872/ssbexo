use super::*;

//Down Taunt Expression
unsafe extern "C" fn ssbexo_dedede_down_taunt_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 4);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lands_hv"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_R, 4);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lands_hv"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 37.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 4);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lands_hv"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 50.0);
    if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_MASK) {
        if is_excute(agent) {
            ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_MASK, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_MASK, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_R, 4);
        }
    }
    else {
        if is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_MASK, false, -1);
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_R, 4);
        }
    }
    frame(agent.lua_state_agent, 55.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lands_hv"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 70.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 4);
    }
    frame(agent.lua_state_agent, 73.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_landl_hv"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 95.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

//Rapid Jab ACMD
unsafe extern "C" fn ssbexo_dedede_rapid_jab_acmd(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 361, 12, 0, 10, 3.5, 0.0, 7.5, 10.0, Some(0.0), Some(7.5), Some(15.0), 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 361, 12, 0, 8, 7.0, 0.0, 7.5, 22.0, None, None, None, 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 5);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 5);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 2.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 361, 12, 0, 10, 3.5, 0.0, 7.5, 10.0, Some(0.0), Some(7.5), Some(15.0), 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 361, 12, 0, 8, 7.0, 0.0, 7.5, 22.0, None, None, None, 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 5);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 5);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 5.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 361, 12, 0, 10, 3.5, 0.0, 7.5, 10.0, Some(0.0), Some(7.5), Some(15.0), 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 361, 12, 0, 8, 7.0, 0.0, 7.5, 22.0, None, None, None, 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 5);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 5);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 8.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 361, 12, 0, 10, 3.5, 0.0, 7.5, 10.0, Some(0.0), Some(7.5), Some(15.0), 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 361, 12, 0, 8, 7.0, 0.0, 7.5, 22.0, None, None, None, 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 5);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 5);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 11.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 361, 12, 0, 10, 3.5, 0.0, 7.5, 10.0, Some(0.0), Some(7.5), Some(15.0), 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 361, 12, 0, 8, 7.0, 0.0, 7.5, 22.0, None, None, None, 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 5);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 5);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 14.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 361, 12, 0, 10, 3.5, 0.0, 7.5, 10.0, Some(0.0), Some(7.5), Some(15.0), 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 361, 12, 0, 8, 7.0, 0.0, 7.5, 22.0, None, None, None, 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 5);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 5);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 17.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 361, 12, 0, 10, 3.5, 0.0, 7.5, 10.0, Some(0.0), Some(7.5), Some(15.0), 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 361, 12, 0, 8, 7.0, 0.0, 7.5, 22.0, None, None, None, 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 5);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 5);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
    }
}

//Rapid Jab Sub ACMD
unsafe extern "C" fn ssbexo_dedede_rapid_jab_sub_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 361, 12, 0, 10, 3.5, 0.0, 7.5, 10.0, Some(0.0), Some(7.5), Some(15.0), 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 361, 12, 0, 8, 7.0, 0.0, 7.5, 22.0, None, None, None, 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 5);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 5);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_dedede_dash_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 361, 92, 0, 65, 7.0, 0.0, 5.0, 11.8, Some(0.0), Some(5.0), Some(5.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 27.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 361, 92, 0, 55, 6.0, 0.0, 4.0, 11.8, Some(0.0), Some(4.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 36.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 361, 92, 0, 55, 4.5, 0.0, 3.5, 10.5, Some(0.0), Some(3.5), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 39.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 361, 92, 0, 55, 3.0, 0.0, 3.0, 9.5, Some(0.0), Some(3.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 42.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("dedede")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .expression_acmd("expression_appeallwr", ssbexo_dedede_down_taunt_expression, Low)
    .expression_acmd("expression_appeallwl", ssbexo_dedede_down_taunt_expression, Low)
    .game_acmd("game_attack100", ssbexo_dedede_rapid_jab_acmd, Low)
    .game_acmd("game_attack100sub", ssbexo_dedede_rapid_jab_sub_acmd, Low)
    .game_acmd("game_attackdash", ssbexo_dedede_dash_attack_acmd, Low)
    .install()
    ;
}