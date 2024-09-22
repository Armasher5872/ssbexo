use super::*;

//Jab 1 ACMD
unsafe extern "C" fn ssbexo_demon_jab_1_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 361, 10, 30, 0, 2.0, 0.0, 14.5, 9.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 10, 30, 0, 3.2, 0.0, 13.5, 8.75, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 361, 10, 30, 0, 3.2, 0.0, 13.5, 3.25, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 3.0, 361, 10, 30, 0, 3.2, 0.0, 11.0, 8.75, Some(0.0), Some(13.5), Some(8.75), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 3.0, 361, 10, 30, 0, 3.2, 0.0, 11.0, 3.25, Some(0.0), Some(13.5), Some(3.25), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 3.0, 33, 15, 35, 0, 3.6, 0.0, 13.2, 3.25, Some(0.0), Some(13.2), Some(9.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 4, 8.0, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_FLASH_PUNCH);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_FLASH_PUNCH);
    }
}

//Jab 2 ACMD
unsafe extern "C" fn ssbexo_demon_jab_2_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        JostleModule::set_push_speed_x_overlap_rate(agent.module_accessor, 20.0);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_ENABLE_COMBO);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 361, 10, 30, 0, 2.0, 0.0, 14.25, 9.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 10, 30, 0, 3.4, 0.0, 13.0, 8.75, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 361, 10, 30, 0, 3.4, 0.0, 13.0, 3.25, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 3.0, 361, 10, 30, 0, 3.4, 0.0, 10.25, 8.75, Some(0.0), Some(13.0), Some(8.75), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 3.0, 361, 10, 30, 0, 3.4, 0.0, 10.25, 3.25, Some(0.0), Some(13.0), Some(3.25), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 3.0, 30, 15, 30, 0, 3.6, 0.0, 12.9, 3.25, Some(0.0), Some(12.9), Some(9.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 11.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 11.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 11.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 11.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 4, 11.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 5, 11.0, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        JostleModule::set_push_speed_x_overlap_rate(agent.module_accessor, 0.0);
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_FLASH_PUNCH);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_FLASH_PUNCH);
    }
}

//Jab 3 ACMD
unsafe extern "C" fn ssbexo_demon_jab_3_acmd(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ONE_TWO_PUNCH) {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 30, 0, 30, 2.0, 0.0, 14.5, 10.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 361, 30, 0, 30, 3.5, 0.0, 13.5, 9.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 361, 30, 0, 30, 3.5, 0.0, 13.5, 3.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 4.0, 361, 30, 0, 30, 3.5, 0.0, 8.75, 9.0, Some(0.0), Some(13.5), Some(9.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 4, 0, Hash40::new("top"), 4.0, 361, 30, 0, 30, 3.5, 0.0, 9.0, 3.0, Some(0.0), Some(13.5), Some(3.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 5, 0, Hash40::new("top"), 4.0, 361, 30, 0, 30, 3.6, 0.0, 15.0, 3.0, Some(0.0), Some(15.0), Some(9.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.5);
            macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 0.5);
            macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 0.5);
            macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 0.5);
            macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 4, 0.5);
            macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 5, 0.5);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 15.0, false);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 15.0, false);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 15.0, false);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 15.0, false);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 4, 15.0, false);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 5, 15.0, false);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_FLASH_PUNCH);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_FLASH_PUNCH);
            WorkModule::off_flag(agent.module_accessor, FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEMON_SLAYER);
            WorkModule::off_flag(agent.module_accessor, FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TWIN_FANG_STATURE_SMASH);
            WorkModule::off_flag(agent.module_accessor, FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TWIN_FANG_DOUBLE_KICK);
        }
    }
    else {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 1, Hash40::new("top"), 2.0, 361, 10, 35, 0, 2.0, 0.0, 14.5, 10.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 1, Hash40::new("top"), 2.0, 361, 10, 35, 0, 3.5, 0.0, 13.5, 9.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 2, 1, Hash40::new("top"), 2.0, 361, 10, 35, 0, 3.5, 0.0, 13.5, 3.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 3, 1, Hash40::new("top"), 2.0, 361, 10, 35, 0, 3.5, 0.0, 8.75, 9.0, Some(0.0), Some(13.5), Some(9.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 4, 1, Hash40::new("top"), 2.0, 361, 10, 35, 0, 3.5, 0.0, 9.0, 3.0, Some(0.0), Some(13.5), Some(3.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 5, 1, Hash40::new("top"), 2.0, 33, 15, 35, 0, 3.6, 0.0, 15.0, 3.0, Some(0.0), Some(15.0), Some(9.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
        }
    }
}

//Flash Punch ACMD
unsafe extern "C" fn ssbexo_demon_flash_punch_acmd(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEMON_SLAYER) {
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 1, Hash40::new("top"), 0.0, 361, 100, 35, 0, 4.0, 0.0, 5.0, 4.0, Some(0.0), Some(5.0), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 75, 50, 0, 20, 2.0, 0.0, 9.5, 9.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 75, 50, 0, 20, 4.0, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(8.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 13.0, 75, 50, 0, 20, 4.0, 0.0, 10.5, 4.0, Some(0.0), Some(10.5), Some(8.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 18.0, false);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 18.0, false);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 18.0, false);
        }
        wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            WorkModule::off_flag(agent.module_accessor, FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ONE_TWO_PUNCH);
            WorkModule::off_flag(agent.module_accessor, FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEMON_SLAYER);
            WorkModule::off_flag(agent.module_accessor, FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TWIN_FANG_STATURE_SMASH);
            WorkModule::off_flag(agent.module_accessor, FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TWIN_FANG_DOUBLE_KICK);
        }
    }
    else {
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 1, Hash40::new("top"), 0.0, 361, 100, 35, 0, 4.0, 0.0, 5.0, 4.0, Some(0.0), Some(5.0), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 43, 70, 0, 76, 2.0, 0.0, 9.5, 9.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 43, 70, 0, 76, 4.0, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(8.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 43, 70, 0, 76, 4.0, 0.0, 10.5, 4.0, Some(0.0), Some(10.5), Some(8.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        }
        wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

//Jab 4 ACMD
unsafe extern "C" fn ssbexo_demon_jab_4_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 361, 80, 0, 10, 2.0, 0.0, 14.0, 10.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 80, 0, 10, 3.6, 0.0, 13.0, 9.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 361, 80, 0, 10, 3.6, 0.0, 13.0, 4.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 3.0, 361, 80, 0, 10, 3.6, 0.0, 9.5, 9.0, Some(0.0), Some(13.0), Some(9.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 3.0, 361, 80, 0, 10, 3.6, 0.0, 9.5, 4.0, Some(0.0), Some(13.0), Some(4.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 3.0, 36, 80, 0, 10, 3.6, 0.0, 14.0, 4.0, Some(0.0), Some(14.0), Some(9.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 4, 8.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 5, 8.0, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
    }
}

//Jab 5 ACMD
unsafe extern "C" fn ssbexo_demon_jab_5_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 42, 75, 0, 10, 4.0, 0.0, 13.0, 4.0, Some(0.0), Some(15.0), Some(11.0), 0.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 0, 75, 0, 10, 3.5, 0.0, 10.0, 5.0, None, None, None, 0.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 0, 75, 0, 10, 3.5, 0.0, 6.0, 5.0, None, None, None, 0.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 3.0, 0, 75, 0, 10, 3.6, 0.0, 9.75, 7.5, None, None, None, 0.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 3.0, 0, 75, 0, 10, 3.6, 0.0, 13.5, 10.75, None, None, None, 0.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 3.0, 0, 75, 0, 10, 2.5, 0.0, 14.5, 10.5, None, None, None, 0.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
    }
}

//Jab 6 ACMD
unsafe extern "C" fn ssbexo_demon_jab_6_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 0, 5, 30, 0, 3.9, 0.0, 12.0, 6.0, None, None, None, 0.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 0, 5, 30, 0, 3.9, 0.0, 10.0, 7.25, None, None, None, 0.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 0, 5, 30, 0, 3.9, 0.0, 8.0, 4.0, None, None, None, 0.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 4.0, 0, 5, 30, 0, 2.5, 0.0, 15.0, 11.5, None, None, None, 0.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 4.0, 0, 5, 30, 0, 4.0, 0.0, 12.0, 10.5, None, None, None, 0.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 9.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 9.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 9.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 9.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 4, 9.0, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 0, 5, 30, 0, 3.9, 0.0, 10.0, 6.0, None, None, None, 0.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 0, 5, 30, 0, 3.9, 0.0, 8.0, 7.25, None, None, None, 0.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 0, 5, 30, 0, 3.9, 0.0, 8.0, 4.0, None, None, None, 0.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 4.0, 0, 5, 30, 0, 2.5, 0.0, 10.0, 10.5, Some(0.0), Some(10.1), Some(10.5), 0.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 4.0, 0, 5, 30, 0, 4.0, 0.0, 8.0, 10.5, None, None, None, 0.4, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 9.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 9.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 9.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 9.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 4, 9.0, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
    }
}

//Jab 7 ACMD
unsafe extern "C" fn ssbexo_demon_jab_7_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 10, 30, 0, 3.3, 0.0, 6.25, 6.5, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_slip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 0, 10, 30, 0, 3.3, 0.0, 7.5, 2.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_slip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.5, 0, 10, 30, 0, 2.0, 0.0, 6.0, 11.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_slip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 1.5, 0, 10, 30, 0, 3.5, 0.0, 5.0, 11.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_slip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 13.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 13.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 13.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 13.0, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
    }
}

//Jab 8 ACMD
unsafe extern "C" fn ssbexo_demon_jab_8_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 0, 10, 30, 0, 3.3, 0.0, 6.25, 8.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 0, 10, 30, 0, 3.3, 0.0, 6.75, 5.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.5, 0, 10, 30, 0, 3.3, 0.0, 7.25, 2.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 1.5, 0, 10, 30, 0, 2.0, 0.0, 6.25, 10.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 1.5, 0, 10, 30, 0, 3.5, 0.0, 5.75, 11.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_auto_shift"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 14.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 14.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 14.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 14.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 4, 14.0, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
    }
}

//Jab 9 ACMD
unsafe extern "C" fn ssbexo_demon_jab_9_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 50, 0, 60, 2.0, 0.0, 11.5, 7.5, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fist_down3"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 361, 50, 0, 60, 4.0, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(7.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fist_down3"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_level(agent.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_2 as u8);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 1, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
    }
}

//Demon God Fist ACMD
unsafe extern "C" fn ssbexo_demon_demon_god_fist_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 361, 50, 0, 60, 2.0, 0.0, 11.25, 9.0, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fist_down2"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_ATTACKSQUAT4, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 361, 50, 0, 60, 4.0, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(9.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fist_down2"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_ATTACKSQUAT4, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 11.0, 361, 60, 0, 40, 4.0, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(9.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fist_down2"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_ATTACKSQUAT4, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 2, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 1, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 2, true, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Left Splits Kick ACMD
unsafe extern "C" fn ssbexo_demon_left_splits_kick_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_DEMON_REFLECTOR_KIND_ATTACK_STAND1, *FIGHTER_DEMON_REFLECTOR_GROUP_ATTACK_STAND1);
        macros::HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 84, 40, 0, 78, 3.8, 0.0, 10.5, 10.5, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 18.0, 84, 40, 0, 78, 3.8, 0.0, 11.5, 5.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 18.0, 84, 40, 0, 78, 4.2, 0.0, 10.5, 2.5, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 10.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.1);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.1);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 1.1);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 84, 40, 0, 78, 3.8, 0.0, 7.25, 7.5, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 18.0, 84, 40, 0, 78, 3.8, 0.0, 11.5, 5.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 18.0, 84, 40, 0, 78, 4.2, 0.0, 10.5, 2.5, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 10.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.1);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.1);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 1.1);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_DEMON_REFLECTOR_KIND_ATTACK_STAND1, *FIGHTER_DEMON_REFLECTOR_GROUP_ATTACK_STAND1);
    }
}

//Twin Fang Double Kick ACMD
unsafe extern "C" fn ssbexo_demon_twin_fang_double_kick_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("footl"), 13.0, 361, 40, 0, 80, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("footl"), 13.0, 361, 40, 0, 80, 4.25, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("kneel"), 13.0, 361, 40, 0, 80, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("legl"), 13.0, 361, 40, 0, 80, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ONE_TWO_PUNCH);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEMON_SLAYER);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TWIN_FANG_STATURE_SMASH);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TWIN_FANG_DOUBLE_KICK);
    }
}

//Twin Fang Double Kick Effect
unsafe extern "C" fn ssbexo_demon_twin_fang_double_kick_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("demon_attack_arc_d"), Hash40::new("top"), 0, 13, 1.5, 0, 0, 187, 0.98, true, 0.4);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}

//Twin Fang Double Kick Sound
unsafe extern "C" fn ssbexo_demon_twin_fang_double_kick_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_demon_swing_long02"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_demon_rnd_attack_s_02"));
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::PLAY_LANDING_SE(agent, Hash40::new("se_demon_landing02"));
    }
}

//Twin Fang Double Kick Expression
unsafe extern "C" fn ssbexo_demon_twin_fang_double_kick_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_80_attackm"), 0);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 3);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_80_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

//Wind God Fist ACMD
unsafe extern "C" fn ssbexo_demon_wind_god_fist_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.7);
    frame(agent.lua_state_agent, 7.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 9.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 13.5, 88, 5, 0, 102, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 13.5, 88, 5, 0, 102, 5.0, 0.0, 13.0, 6.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 13.5, 88, 5, 0, 102, 3.0, -1.0, 9.0, 3.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 3, 0, Hash40::new("handr"), 13.0, 73, 5, 0, 90, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 13.0, 73, 5, 0, 90, 5.0, 0.0, 13.0, 6.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 13.0, 73, 5, 0, 90, 3.0, -1.0, 9.0, 3.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.2);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.2);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 1.2);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 4, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 5, 10.0, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 0, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 1, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 2, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 3, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 4, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 5, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 13.5, 88, 5, 0, 102, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 13.5, 88, 5, 0, 102, 5.0, 0.0, 18.0, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 13.5, 88, 5, 0, 102, 3.0, -1.0, 13.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 3, 0, Hash40::new("handr"), 13.0, 73, 5, 0, 92, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 13.0, 73, 5, 0, 92, 5.0, 0.0, 18.0, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 13.0, 73, 5, 0, 92, 3.0, -1.0, 13.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.2);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.2);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 1.2);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 4, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 5, 10.0, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 0, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 1, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 2, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 3, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 4, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 5, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 13.5, 88, 5, 0, 102, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 13.5, 88, 5, 0, 102, 5.0, 0.0, 19.0, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 13.5, 88, 5, 0, 102, 3.0, 0.0, 15.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 3, 0, Hash40::new("handr"), 13.0, 73, 5, 0, 92, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 13.0, 73, 5, 0, 92, 5.0, 0.0, 19.0, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 13.0, 73, 5, 0, 92, 3.0, 0.0, 15.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.2);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.2);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 1.2);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 4, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 5, 10.0, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 0, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 1, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 2, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 3, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 4, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 5, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(agent.lua_state_agent, 14.0);
    macros::FT_MOTION_RATE(agent, 0.9);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(agent.module_accessor);
    }
}

//I'm not going to sugarcoat it ACMD
unsafe extern "C" fn ssbexo_demon_electric_wind_god_fist_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.7);
    frame(agent.lua_state_agent, 7.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 9.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("handr"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.0, 0.01, 0.01, 0.01, false, 1.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 14.5, 88, 5, 0, 102, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.5, 88, 5, 0, 102, 5.0, 0.0, 13.0, 6.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 14.5, 88, 5, 0, 102, 3.0, -1.0, 9.0, 3.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 3, 0, Hash40::new("handr"), 14.0, 81, 5, 0, 92, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 14.0, 81, 5, 0, 92, 5.0, 0.0, 13.0, 6.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 14.0, 81, 5, 0, 92, 3.0, -1.0, 9.0, 3.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.2);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.2);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 1.2);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 4, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 5, 10.0, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 2, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 3, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 4, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 5, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 14.5, 88, 5, 0, 102, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.5, 88, 5, 0, 102, 5.0, 0.0, 18.0, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 14.5, 88, 5, 0, 102, 3.0, -1.0, 13.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 3, 0, Hash40::new("handr"), 14.0, 81, 5, 0, 92, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 14.0, 81, 5, 0, 92, 5.0, 0.0, 18.0, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 14.0, 81, 5, 0, 92, 3.0, -1.0, 13.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.2);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.2);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 1.2);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 4, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 5, 10.0, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 2, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 3, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 4, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 5, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(agent.lua_state_agent, 15.0);
    macros::FT_MOTION_RATE(agent, 0.9);
}

//Dragon God Fist ACMD
unsafe extern "C" fn ssbexo_demon_dragon_god_fist_acmd(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.9);
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 19.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        AttackModule::set_damage_shake_scale(agent.module_accessor, 1.5);
        macros::ATTACK(agent, 0, 0, Hash40::new("handl"), 20.0, 60, 55, 0, 80, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 20.0, 60, 55, 0, 80, 5.25, 0.0, 10.5, 4.75, Some(0.0), Some(16.0), Some(7.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 20.0, 60, 55, 0, 80, 5.25, 0.0, 5.0, 2.5, Some(0.0), Some(10.5), Some(4.75), 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.5);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 0.5);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 0.5);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 2, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 2, false);
        AttackModule::set_damage_shake_scale(agent.module_accessor, 1.0);
        macros::ATTACK(agent, 0, 0, Hash40::new("handl"), 18.0, 70, 60, 0, 80, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.1), 0.17, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 18.0, 70, 60, 0, 80, 5.0, 0.0, 13.0, 3.0, Some(0.0), Some(23.0), Some(3.0), 0.17, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.5);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 0.5);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handl"), 16.0, 70, 50, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.17, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 70, 50, 0, 60, 5.0, 0.0, 14.0, 3.0, Some(0.0), Some(24.0), Some(3.0), 0.17, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.5);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 0.5);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        macros::ATTACK(agent, 0, 0, Hash40::new("handl"), 14.0, 70, 50, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 70, 50, 0, 60, 5.0, 0.0, 21.0, 3.0, Some(0.0), Some(24.0), Some(3.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 0, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 1, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 70, 50, 0, 60, 5.0, 0.0, 21.5, 3.0, Some(0.0), Some(24.5), Some(3.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 0, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 1, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 70, 50, 0, 60, 5.0, 0.0, 22.0, 3.0, Some(0.0), Some(25.0), Some(3.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 0, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 1, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(agent.lua_state_agent, 28.0);
    macros::FT_MOTION_RATE(agent, 0.9);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Spinning Demon to Left Hook ACMD
unsafe extern "C" fn ssbexo_demon_spinning_demon_to_left_hook_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 65, 100, 55, 0, 1.0, 0.0, 8.25, 0.5, Some(0.0), Some(2.25), Some(0.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 80, 100, 55, 0, 4.0, 0.0, 5.25, 3.0, Some(0.0), Some(5.25), Some(8.25), 0.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 65, 100, 55, 0, 4.0, 0.0, 7.5, 3.0, Some(0.0), Some(7.5), Some(7.25), 0.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.4);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.4);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 1.4);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 7.0, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 65, 100, 55, 0, 1.0, 0.0, 8.25, 0.5, Some(0.0), Some(2.25), Some(0.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 80, 100, 55, 0, 4.0, 0.0, 5.25, 3.0, Some(0.0), Some(5.25), Some(12.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.4);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.4);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 7.0, false);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 65, 100, 55, 0, 1.0, 0.0, 8.25, 0.5, Some(0.0), Some(2.25), Some(0.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 80, 100, 55, 0, 4.0, 0.0, 5.25, 3.0, Some(0.0), Some(5.25), Some(11.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.4);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.4);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 7.0, false);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.5, 44, 65, 0, 75, 2.5, 0.0, 15.0, 7.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.5, 44, 65, 0, 75, 4.0, 0.0, 14.0, 8.0, Some(0.0), Some(11.5), Some(8.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 14.5, 44, 65, 0, 75, 3.5, 0.0, 13.0, 3.5, Some(0.0), Some(8.5), Some(3.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.4);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.4);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 1.4);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 2, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("demon")
    .game_acmd("game_attack11", ssbexo_demon_jab_1_acmd, Priority::Low)
    .game_acmd("game_attack12", ssbexo_demon_jab_2_acmd, Priority::Low)
    .game_acmd("game_attack13", ssbexo_demon_jab_3_acmd, Priority::Low)
    .game_acmd("game_flashpunch", ssbexo_demon_flash_punch_acmd, Priority::Low)
    .game_acmd("game_attack14", ssbexo_demon_jab_4_acmd, Priority::Low)
    .game_acmd("game_attack15", ssbexo_demon_jab_5_acmd, Priority::Low)
    .game_acmd("game_attack16", ssbexo_demon_jab_6_acmd, Priority::Low)
    .game_acmd("game_attack17", ssbexo_demon_jab_7_acmd, Priority::Low)
    .game_acmd("game_attack18", ssbexo_demon_jab_8_acmd, Priority::Low)
    .game_acmd("game_attack19", ssbexo_demon_jab_9_acmd, Priority::Low)
    .game_acmd("game_attacksquat4", ssbexo_demon_demon_god_fist_acmd, Priority::Low)
    .game_acmd("game_attackstand1", ssbexo_demon_left_splits_kick_acmd, Priority::Low)
    .game_acmd("game_twinfangdoublekick", ssbexo_demon_twin_fang_double_kick_acmd, Priority::Low)
    .effect_acmd("effect_twinfangdoublekick", ssbexo_demon_twin_fang_double_kick_effect, Priority::Low)
    .sound_acmd("sound_twinfangdoublekick", ssbexo_demon_twin_fang_double_kick_sound, Priority::Low)
    .expression_acmd("expression_twinfangdoublekick", ssbexo_demon_twin_fang_double_kick_expression, Priority::Low)
    .game_acmd("game_attackstep2", ssbexo_demon_wind_god_fist_acmd, Priority::Low)
    .game_acmd("game_attackstep2f", ssbexo_demon_electric_wind_god_fist_acmd, Priority::Low)
    .game_acmd("game_attackstep2l", ssbexo_demon_dragon_god_fist_acmd, Priority::Low)
    .game_acmd("game_attackstep2s", ssbexo_demon_spinning_demon_to_left_hook_acmd, Priority::Low)
    .install()
    ;
}