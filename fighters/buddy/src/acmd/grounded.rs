use super::*;

//Rapid Jab ACMD
unsafe extern "C" fn ssbexo_buddy_rapid_jab_acmd(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 16, 3.8, 0.0, 6.8, 4.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 3.8, 0.0, 6.8, 10.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.4, 361, 10, 0, 10, 3.8, 0.0, 6.8, 16.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 3, 0, Hash40::new("k_neck"), 0.4, 361, 10, 0, 10, 3.2, 3.0, 0.0, 0.0, Some(12.0), Some(0.0), Some(0.0), 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 3, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 0, 1, 2, 3, 6.0);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 16, 3.8, 0.0, 6.8, 4.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 3.8, 0.0, 6.8, 10.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.4, 361, 10, 0, 10, 3.8, 0.0, 6.8, 16.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 3, 0, Hash40::new("k_neck"), 0.4, 361, 10, 0, 10, 3.2, 3.0, 0.0, 0.0, Some(12.0), Some(0.0), Some(0.0), 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 3, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 0, 1, 2, 3, 6.0);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 3.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 16, 3.8, 0.0, 6.8, 4.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 3.8, 0.0, 6.8, 10.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.4, 361, 10, 0, 10, 3.8, 0.0, 6.8, 16.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 3, 0, Hash40::new("k_neck"), 0.4, 361, 10, 0, 10, 3.2, 3.0, 0.0, 0.0, Some(12.0), Some(0.0), Some(0.0), 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 3, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 0, 1, 2, 3, 6.0);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 5.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 16, 3.8, 0.0, 6.8, 4.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 3.8, 0.0, 6.8, 10.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.4, 361, 10, 0, 10, 3.8, 0.0, 6.8, 16.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 3, 0, Hash40::new("k_neck"), 0.4, 361, 10, 0, 10, 3.2, 3.0, 0.0, 0.0, Some(12.0), Some(0.0), Some(0.0), 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 3, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 0, 1, 2, 3, 6.0);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 7.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 16, 3.8, 0.0, 6.8, 4.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 3.8, 0.0, 6.8, 10.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.4, 361, 10, 0, 10, 3.8, 0.0, 6.8, 16.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 3, 0, Hash40::new("k_neck"), 0.4, 361, 10, 0, 10, 3.2, 3.0, 0.0, 0.0, Some(12.0), Some(0.0), Some(0.0), 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 3, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 0, 1, 2, 3, 6.0);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 9.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 16, 3.8, 0.0, 6.8, 4.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 3.8, 0.0, 6.8, 10.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.4, 361, 10, 0, 10, 3.8, 0.0, 6.8, 16.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 3, 0, Hash40::new("k_neck"), 0.4, 361, 10, 0, 10, 3.2, 3.0, 0.0, 0.0, Some(12.0), Some(0.0), Some(0.0), 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 3, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 0, 1, 2, 3, 6.0);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 11.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 16, 3.8, 0.0, 6.8, 4.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 3.8, 0.0, 6.8, 10.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.4, 361, 10, 0, 10, 3.8, 0.0, 6.8, 16.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 3, 0, Hash40::new("k_neck"), 0.4, 361, 10, 0, 10, 3.2, 3.0, 0.0, 0.0, Some(12.0), Some(0.0), Some(0.0), 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 3, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 0, 1, 2, 3, 6.0);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        frame(agent.lua_state_agent, 13.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 16, 3.8, 0.0, 6.8, 4.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 3.8, 0.0, 6.8, 10.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.4, 361, 10, 0, 10, 3.8, 0.0, 6.8, 16.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            ATTACK(agent, 3, 0, Hash40::new("k_neck"), 0.4, 361, 10, 0, 10, 3.2, 3.0, 0.0, 0.0, Some(12.0), Some(0.0), Some(0.0), 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 3.0, false);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 3, 3.0, false);
            ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 0, 1, 2, 3, 6.0);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
    }
}

//Rapid Jab Sub ACMD
unsafe extern "C" fn ssbexo_buddy_rapid_jab_sub_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 16, 3.8, 0.0, 6.8, 4.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 3.8, 0.0, 6.8, 10.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.4, 361, 10, 0, 10, 3.8, 0.0, 6.8, 16.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(agent, 3, 0, Hash40::new("k_neck"), 0.4, 361, 10, 0, 10, 3.2, 3.0, 0.0, 0.0, Some(12.0), Some(0.0), Some(0.0), 0.6, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 3.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 3, 3.0, false);
        ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 0, 1, 2, 3, 6.0);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_buddy_dash_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 62, 72, 0, 74, 8.0, 0.0, 5.8, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 62, 42, 0, 58, 8.0, 0.0, 5.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("buddy")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attack100", ssbexo_buddy_rapid_jab_acmd, Low)
    .game_acmd("game_attack100sub", ssbexo_buddy_rapid_jab_sub_acmd, Low)
    .game_acmd("game_attackdash", ssbexo_buddy_dash_attack_acmd, Low)
    .install()
    ;
}