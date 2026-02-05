use super::*;

//Rapid Jab ACMD
unsafe extern "C" fn ssbexo_tantan_rapid_jab_acmd(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 361, 16, 0, 12, 3.8, 0.0, 5.4, 7.0, Some(0.0), Some(11.6), Some(7.0), 1.0, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.5, 361, 16, 0, 8, 5.6, 0.0, 5.4, 14.0, Some(0.0), Some(11.6), Some(14.0), 1.0, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 9);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 9);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        if is_excute(agent) {
            FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 3.0, 4.0);
        }
        frame(agent.lua_state_agent, 2.0);
        if is_excute(agent) {
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 361, 16, 0, 12, 3.8, 0.0, 5.4, 7.0, Some(0.0), Some(11.6), Some(7.0), 1.0, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.5, 361, 16, 0, 8, 5.6, 0.0, 5.4, 14.0, Some(0.0), Some(11.6), Some(14.0), 1.0, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 9);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 9);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 4.0);
        if is_excute(agent) {
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 361, 16, 0, 12, 3.8, 0.0, 5.4, 7.0, Some(0.0), Some(11.6), Some(7.0), 1.0, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.5, 361, 16, 0, 8, 5.6, 0.0, 5.4, 14.0, Some(0.0), Some(11.6), Some(14.0), 1.0, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 9);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 9);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 6.0);
        if is_excute(agent) {
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 361, 16, 0, 12, 3.8, 0.0, 5.4, 7.0, Some(0.0), Some(11.6), Some(7.0), 1.0, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.5, 361, 16, 0, 8, 5.6, 0.0, 5.4, 14.0, Some(0.0), Some(11.6), Some(14.0), 1.0, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 9);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 9);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 8.0);
        if is_excute(agent) {
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 361, 16, 0, 12, 3.8, 0.0, 5.4, 7.0, Some(0.0), Some(11.6), Some(7.0), 1.0, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.5, 361, 16, 0, 8, 5.6, 0.0, 5.4, 14.0, Some(0.0), Some(11.6), Some(14.0), 1.0, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 9);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 9);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 10.0);
        if is_excute(agent) {
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 361, 16, 0, 12, 3.8, 0.0, 5.4, 7.0, Some(0.0), Some(11.6), Some(7.0), 1.0, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.5, 361, 16, 0, 8, 5.6, 0.0, 5.4, 14.0, Some(0.0), Some(11.6), Some(14.0), 1.0, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 9);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 9);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 12.0);
        if is_excute(agent) {
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 361, 16, 0, 12, 3.8, 0.0, 5.4, 7.0, Some(0.0), Some(11.6), Some(7.0), 1.0, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.5, 361, 16, 0, 8, 5.6, 0.0, 5.4, 14.0, Some(0.0), Some(11.6), Some(14.0), 1.0, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 9);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 9);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
    }
}

//Rapid Jab Sub ACMD
unsafe extern "C" fn ssbexo_tantan_rapid_jab_sub_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 361, 16, 0, 12, 3.8, 0.0, 5.4, 7.0, Some(0.0), Some(11.6), Some(7.0), 1.0, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.5, 361, 16, 0, 8, 5.6, 0.0, 5.4, 14.0, Some(0.0), Some(11.6), Some(14.0), 1.0, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 9);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 9);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_tantan_dash_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 5.5, 6.5, 9.0, 1.5);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legl"), 8.0, 40, 50, 0, 85, 4.0, 0.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 10.0, 40, 50, 0, 85, 3.5, 0.0, 0.0, -1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("footl"), 10.0, 40, 50, 0, 85, 3.5, 0.0, 0.0, -1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legl"), 6.0, 40, 50, 0, 85, 4.0, 0.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 8.0, 40, 50, 0, 85, 3.0, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("footl"), 8.0, 40, 50, 0, 85, 3.0, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 4.0, 2.9, 7.2, 7.2);
    }
}

pub fn install() {
    Agent::new("tantan")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attack100", ssbexo_tantan_rapid_jab_acmd, Low)
    .game_acmd("game_attack100sub", ssbexo_tantan_rapid_jab_sub_acmd, Low)
    .game_acmd("game_attackdash", ssbexo_tantan_dash_attack_acmd, Low)
    .install()
    ;
}