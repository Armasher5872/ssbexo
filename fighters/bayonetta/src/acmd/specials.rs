use super::*;

//Heel Slide Bullet Art On ACMD
unsafe extern "C" fn ssbexo_bayonetta_heel_slide_bullet_art_on_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01 as u64, 1, Hash40::new("top"), 1.0, 14, 0, 0, 0, 2.5, 0.0, 6.0, 10.0, Some(0.0), Some(20.5), Some(68.2), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_63_bullet"), 4, true, *BATTLE_OBJECT_ID_INVALID as u32);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x36db1a34c9), FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01, 8, 4);
    }
}

//Heel Slide Hold Bullet Art On ACMD
unsafe extern "C" fn ssbexo_bayonetta_heel_slide_hold_bullet_art_on_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01 as u64, 1, Hash40::new("top"), 1.0, 14, 0, 0, 0, 2.5, 0.0, 6.0, 10.0, Some(0.0), Some(20.5), Some(68.2), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_63_bullet"), 4, true, *BATTLE_OBJECT_ID_INVALID as u32);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x36db1a34c9), FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01, 8, 4);
    }
}

//Afterburner Kick Up Bullet Art On Left ACMD
unsafe extern "C" fn ssbexo_bayonetta_afterburner_kick_up_bullet_art_on_left_acmd(agent: &mut L2CAgentBase) {
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK) {
        if is_excute(agent) {
            ATTACK(agent, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00 as u64, 0, Hash40::new("top"), 1.0, 340, 100, 0, 10, 2.5, 0.0, 2.5, 3.0, Some(0.0), Some(1.1), Some(6.8), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        }
    }
    if is_excute(agent) {
        ATTACK(agent, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01 as u64, 0, Hash40::new("top"), 1.0, 340, 0, 0, 0, 2.5, 0.0, 2.5, 3.0, Some(0.0), Some(-18.0), Some(59.4), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_63_bullet"), 4, true, *BATTLE_OBJECT_ID_INVALID as u32);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x36db1a34c9), FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01, 6, 4.05);
    }
}

//Afterburner Kick Up Bullet Art On Right ACMD
unsafe extern "C" fn ssbexo_bayonetta_afterburner_kick_up_bullet_art_on_right_acmd(agent: &mut L2CAgentBase) {
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK) {
        if is_excute(agent) {
            ATTACK(agent, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00 as u64, 0, Hash40::new("top"), 1.0, 20, 100, 0, 10, 2.5, 0.0, 13.0, 12.5, Some(0.0), Some(14.4), Some(16.3), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        }
    }
    if is_excute(agent) {
        ATTACK(agent, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01 as u64, 0, Hash40::new("top"), 1.0, 20, 0, 0, 0, 2.5, 0.0, 13.0, 12.5, Some(0.0), Some(33.5), Some(68.9), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_63_bullet"), 4, true, *BATTLE_OBJECT_ID_INVALID as u32);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x36db1a34c9), FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01, 12, 4.05);
    }
}

//Grounded Witch Twist Bullet Art On Left Arm ACMD
unsafe extern "C" fn ssbexo_bayonetta_grounded_witch_twist_bullet_art_on_left_arm_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01 as u64, 1, Hash40::new("top"), 1.0, 361, 0, 0, 0, 11.0, 0.0, 11.0, 0.0, Some(0.0), Some(11.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
    }
}

//Grounded Witch Twist Bullet Art On Right Arm ACMD
unsafe extern "C" fn ssbexo_bayonetta_grounded_witch_twist_bullet_art_on_right_arm_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01 as u64, 1, Hash40::new("top"), 1.0, 90, 0, 0, 0, 2.5, 0.0, 26.0, 0.0, Some(0.0), Some(86.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_63_bullet"), 4, true, *BATTLE_OBJECT_ID_INVALID as u32);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x36db1a34c9), FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01, 20, 4);
    }
}

//Grounded Witch Twist Bullet Art On Left Leg ACMD
unsafe extern "C" fn ssbexo_bayonetta_grounded_witch_twist_bullet_art_on_left_leg_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_03 as u64, 1, Hash40::new("top"), 1.0, 270, 0, 0, 0, 2.5, 0.0, 2.0, -0.5, Some(0.0), Some(-58.0), Some(-0.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_63_bullet"), 4, true, *BATTLE_OBJECT_ID_INVALID as u32);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x36db1a34c9), FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_03, 10, 4);
    }
}

//Grounded Witch Twist Bullet Art On Right Leg ACMD
unsafe extern "C" fn ssbexo_bayonetta_grounded_witch_twist_bullet_art_on_right_leg_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_03 as u64, 1, Hash40::new("top"), 1.0, 361, 0, 0, 0, 11.0, 0.0, 11.0, 0.0, Some(0.0), Some(11.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
    }
}

//Aerial Witch Twist Bullet Art On Left Arm ACMD
unsafe extern "C" fn ssbexo_bayonetta_aerial_witch_twist_bullet_art_on_left_arm_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01 as u64, 1, Hash40::new("top"), 0.5, 361, 0, 0, 0, 11.0, 0.0, 11.0, 0.0, Some(0.0), Some(11.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
    }
}

//Aerial Witch Twist Bullet Art On Right Arm ACMD
unsafe extern "C" fn ssbexo_bayonetta_aerial_witch_twist_bullet_art_on_right_arm_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01 as u64, 1, Hash40::new("top"), 0.5, 90, 0, 0, 0, 2.5, 0.0, 26.0, 0.0, Some(0.0), Some(86.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_63_bullet"), 4, true, *BATTLE_OBJECT_ID_INVALID as u32);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x36db1a34c9), FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01, 20, 4);
    }
}

//Aerial Witch Twist Bullet Art On Left Leg ACMD
unsafe extern "C" fn ssbexo_bayonetta_aerial_witch_twist_bullet_art_on_left_leg_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_03 as u64, 1, Hash40::new("top"), 0.5, 270, 0, 0, 0, 2.5, 0.0, 2.0, -0.5, Some(0.0), Some(-58.0), Some(-0.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_63_bullet"), 4, true, *BATTLE_OBJECT_ID_INVALID as u32);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x36db1a34c9), FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_03, 10, 4);
    }
}

//Aerial Witch Twist Bullet Art On Right Leg ACMD
unsafe extern "C" fn ssbexo_bayonetta_aerial_witch_twist_bullet_art_on_right_leg_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_03 as u64, 1, Hash40::new("top"), 0.5, 361, 0, 0, 0, 11.0, 0.0, 11.0, 0.0, Some(0.0), Some(11.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
    }
}

//Grounded Witch Time ACMD
unsafe extern "C" fn ssbexo_bayonetta_grounded_witch_time_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_ENABLE_NEXT_NO_COMP);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME);
        SEARCH(agent, 0, 0, Hash40::new("top"), 11.5, -2.0, 10.0, 0.0, None, None, None, *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_BODY_HEAD, false);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(agent.lua_state_agent, 44.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_ENABLE_NEXT_NO_COMP);
    }
}

//Aerial Witch Time ACMD
unsafe extern "C" fn ssbexo_bayonetta_aerial_witch_time_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_ENABLE_NEXT_NO_COMP);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME);
        SEARCH(agent, 0, 0, Hash40::new("top"), 11.5, -2.0, 10.0, 0.0, None, None, None,  *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_BODY_HEAD, false);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(agent.lua_state_agent, 37.0);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME_SUCCESS) {
        if is_excute(agent) {
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }
    }
    frame(agent.lua_state_agent, 44.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_ENABLE_NEXT_NO_COMP);
    }
}

pub fn install() {
    Agent::new("bayonetta")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_shootinglegr_atkon_specials", ssbexo_bayonetta_heel_slide_bullet_art_on_acmd, Low)
    .game_acmd("game_shootinglegr_atkon_specials", ssbexo_bayonetta_heel_slide_hold_bullet_art_on_acmd, Low)
    .game_acmd("game_shootinglegl_atkon_specialairsu", ssbexo_bayonetta_afterburner_kick_up_bullet_art_on_left_acmd, Low)
    .game_acmd("game_shootinglegr_atkon_specialairsu", ssbexo_bayonetta_afterburner_kick_up_bullet_art_on_right_acmd, Low)
    .game_acmd("game_shootingarml_atkon_specialhi", ssbexo_bayonetta_grounded_witch_twist_bullet_art_on_left_arm_acmd, Low)
    .game_acmd("game_shootingarmr_atkon_specialhi", ssbexo_bayonetta_grounded_witch_twist_bullet_art_on_right_arm_acmd, Low)
    .game_acmd("game_shootinglegl_atkon_specialhi", ssbexo_bayonetta_grounded_witch_twist_bullet_art_on_left_leg_acmd, Low)
    .game_acmd("game_shootinglegr_atkon_specialhi", ssbexo_bayonetta_grounded_witch_twist_bullet_art_on_right_leg_acmd, Low)
    .game_acmd("game_shootingarml_atkon_specialairhi", ssbexo_bayonetta_aerial_witch_twist_bullet_art_on_left_arm_acmd, Low)
    .game_acmd("game_shootingarmr_atkon_specialairhi", ssbexo_bayonetta_aerial_witch_twist_bullet_art_on_right_arm_acmd, Low)
    .game_acmd("game_shootinglegl_atkon_specialairhi", ssbexo_bayonetta_aerial_witch_twist_bullet_art_on_left_leg_acmd, Low)
    .game_acmd("game_shootinglegr_atkon_specialairhi", ssbexo_bayonetta_aerial_witch_twist_bullet_art_on_right_leg_acmd, Low)
    .game_acmd("game_speciallw", ssbexo_bayonetta_grounded_witch_time_acmd, Low)
    .game_acmd("game_specialairlw", ssbexo_bayonetta_aerial_witch_time_acmd, Low)
    .install()
    ;
}