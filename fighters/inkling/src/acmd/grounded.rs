use super::*;

//Rapid Jab ACMD
unsafe extern "C" fn ssbexo_inkling_rapid_jab_acmd(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 3.5, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 7, 4.5, 0.0, 8.0, 19.5, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
                AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
                ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 6.0);
                AttackModule::set_ink_value(agent.module_accessor, 0, 10.0);
                AttackModule::set_ink_value(agent.module_accessor, 1, 10.0);
            }
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 2.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 3.5, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 7, 4.5, 0.0, 8.0, 19.5, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
                AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
                ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 6.0);
                AttackModule::set_ink_value(agent.module_accessor, 0, 10.0);
                AttackModule::set_ink_value(agent.module_accessor, 1, 10.0);
            }
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 4.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 3.5, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 7, 4.5, 0.0, 8.0, 19.5, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
                AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
                ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 6.0);
                AttackModule::set_ink_value(agent.module_accessor, 0, 10.0);
                AttackModule::set_ink_value(agent.module_accessor, 1, 10.0);
            }
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 6.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 3.5, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 7, 4.5, 0.0, 8.0, 19.5, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
                AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
                ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 6.0);
                AttackModule::set_ink_value(agent.module_accessor, 0, 10.0);
                AttackModule::set_ink_value(agent.module_accessor, 1, 10.0);
            }
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 8.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 3.5, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 7, 4.5, 0.0, 8.0, 19.5, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
                AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
                ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 6.0);
                AttackModule::set_ink_value(agent.module_accessor, 0, 10.0);
                AttackModule::set_ink_value(agent.module_accessor, 1, 10.0);
            }
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 10.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 3.5, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 7, 4.5, 0.0, 8.0, 19.5, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
                AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
                ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 6.0);
                AttackModule::set_ink_value(agent.module_accessor, 0, 10.0);
                AttackModule::set_ink_value(agent.module_accessor, 1, 10.0);
            }
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 12.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 3.5, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 7, 4.5, 0.0, 8.0, 19.5, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
                AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
                ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 6.0);
                AttackModule::set_ink_value(agent.module_accessor, 0, 10.0);
                AttackModule::set_ink_value(agent.module_accessor, 1, 10.0);
            }
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 14.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 3.5, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 7, 4.5, 0.0, 8.0, 19.5, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
                AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
                ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 6.0);
                AttackModule::set_ink_value(agent.module_accessor, 0, 10.0);
                AttackModule::set_ink_value(agent.module_accessor, 1, 10.0);
            }
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
    }
}

//Rapid Jab Sub ACMD
unsafe extern "C" fn ssbexo_inkling_rapid_jab_sub_acmd(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 3.5, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 7, 4.5, 0.0, 8.0, 19.5, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
            ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 6.0);
            AttackModule::set_ink_value(agent.module_accessor, 0, 10.0);
            AttackModule::set_ink_value(agent.module_accessor, 1, 10.0);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_inkling_dash_attack_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        inkling_generate_squid_helper(agent)
    }
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, true);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 60, 43, 0, 110, 4.0, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ELBOW);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.5);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 60, 43, 0, 100, 3.0, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ELBOW);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.5);
    }
    wait(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("inkling")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attack100", ssbexo_inkling_rapid_jab_acmd, Low)
    .game_acmd("game_attack100sub", ssbexo_inkling_rapid_jab_sub_acmd, Low)
    .game_acmd("game_attackdash", ssbexo_inkling_dash_attack_acmd, Low)
    .install()
    ;
}