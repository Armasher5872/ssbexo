use super::*;

//Rapid Jab ACMD
unsafe extern "C" fn ssbexo_zelda_rapid_jab_acmd(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 20, 0, 7, 6.0, 0.0, 8.5, 8.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 20, 0, 7, 6.0, 0.0, 8.5, 13.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 8.0);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 20, 0, 7, 6.0, 0.0, 8.5, 8.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 20, 0, 7, 6.0, 0.0, 8.5, 13.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 8.0);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 3.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 20, 0, 7, 6.0, 0.0, 8.5, 8.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 20, 0, 7, 6.0, 0.0, 8.5, 13.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 8.0);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 5.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 20, 0, 7, 6.0, 0.0, 8.5, 8.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 20, 0, 7, 6.0, 0.0, 8.5, 13.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 8.0);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 7.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 20, 0, 7, 6.0, 0.0, 8.5, 8.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 20, 0, 7, 6.0, 0.0, 8.5, 13.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 8.0);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 9.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 20, 0, 7, 6.0, 0.0, 8.5, 8.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 20, 0, 7, 6.0, 0.0, 8.5, 13.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 8.0);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 11.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 20, 0, 7, 6.0, 0.0, 8.5, 8.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 20, 0, 7, 6.0, 0.0, 8.5, 13.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 8.0);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 13.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 20, 0, 7, 6.0, 0.0, 8.5, 8.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 20, 0, 7, 6.0, 0.0, 8.5, 13.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 8.0);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
    }
}

//Rapid Jab Sub ACMD
unsafe extern "C" fn ssbexo_zelda_rapid_jab_sub_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 20, 0, 7, 6.0, 0.0, 8.5, 8.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 20, 0, 7, 6.0, 0.0, 8.5, 13.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 8.0);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_zelda_dash_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 50, 70, 0, 85, 2.8, 0.0, 9.0, 12.6, Some(0.0), Some(9.0), Some(13.2), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 50, 50, 0, 80, 5.2, 0.0, 9.0, 13.6, Some(0.0), Some(9.0), Some(7.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 50, 50, 0, 80, 5.0, 0.0, 9.0, 13.6, Some(0.0), Some(9.0), Some(7.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    wait(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("zelda")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attack100", ssbexo_zelda_rapid_jab_acmd, Low)
    .game_acmd("game_attack100sub", ssbexo_zelda_rapid_jab_sub_acmd, Low)
    .game_acmd("game_attackdash", ssbexo_zelda_dash_attack_acmd, Low)
    .install()
    ;
}