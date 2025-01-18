use super::*;

//Forward Smash ACMD
unsafe extern "C" fn ssbexo_robot_forward_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_POWER_BOOST);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_EARLY_CANCEL);
        if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST) {
            MotionModule::set_rate(agent.module_accessor, 2.0);
        }
    }
    frame(agent.lua_state_agent, 16.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST) {
        if macros::is_excute(agent) {
            WorkModule::set_float(agent.module_accessor, 0.0, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BEAM_ENERGY_VALUE);
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST);
            MotionModule::set_rate(agent.module_accessor, 1.0);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 41, 85, 0, 32, 5.3, 0.0, 10.3, 21.0, Some(0.0), Some(10.3), Some(8.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 41, 85, 0, 32, 4.0, 0.0, 10.3, 36.0, Some(0.0), Some(10.3), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 41, 85, 0, 32, 2.8, 0.0, 10.3, 51.0, Some(0.0), Some(10.3), Some(6.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        }
    }
    else {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.0);
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_POWER_BOOST);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 361, 105, 0, 40, 5.3, 0.0, 10.3, 14.0, Some(0.0), Some(10.3), Some(8.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 361, 110, 0, 40, 4.0, 0.0, 10.3, 24.0, Some(0.0), Some(10.3), Some(7.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 6.0, 361, 110, 0, 40, 2.8, 0.0, 10.3, 34.0, Some(0.0), Some(10.3), Some(6.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        }
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Smash Effect
unsafe extern "C" fn ssbexo_robot_forward_smash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("head1"), 1.6, 7, 0.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST) {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("robot_eye_flash"), Hash40::new("head1"), 1.6, 4.5, 0.0, -90, -90, 0, 1.5, true);
            macros::EFFECT_FLW_POS(agent, Hash40::new("robot_attack_s4s"), Hash40::new("head1"), 1.55, 5, 0, 0, 0, 0, 1.5, true);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("robot_eye_flash"), Hash40::new("head1"), 1.6, 4.5, 0.0, -90, -90, 0, 1, true);
            macros::EFFECT_FLW_POS(agent, Hash40::new("robot_attack_s4s"), Hash40::new("head1"), 1.55, 5, 0, 0, 0, 0, 1, true);
        }
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -10, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

//Forward Smash Hi ACMD
unsafe extern "C" fn ssbexo_robot_forward_smash_hi_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_POWER_BOOST);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_EARLY_CANCEL);
        if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST) {
            MotionModule::set_rate(agent.module_accessor, 2.0);
        }
    }
    frame(agent.lua_state_agent, 16.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST) {
        if macros::is_excute(agent) {
            WorkModule::set_float(agent.module_accessor, 0.0, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BEAM_ENERGY_VALUE);
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST);
            MotionModule::set_rate(agent.module_accessor, 1.0);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 41, 85, 0, 32, 5.3, 0.0, 12.7, 19.95, Some(0.0), Some(10.8), Some(8.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 41, 85, 0, 32, 4.0, 0.0, 15.9, 34.8, Some(0.0), Some(10.6), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 41, 85, 0, 32, 2.8, 0.0, 18.8, 49.5, Some(0.0), Some(10.3), Some(6.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        }
    }
    else {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.0);
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_POWER_BOOST);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 361, 105, 0, 40, 5.3, 0.0, 12.7, 13.3, Some(0.0), Some(10.8), Some(8.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 361, 110, 0, 40, 4.0, 0.0, 15.9, 23.2, Some(0.0), Some(10.6), Some(7.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 6.0, 361, 110, 0, 40, 2.8, 0.0, 18.8, 33.0, Some(0.0), Some(10.3), Some(6.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        }
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Smash Hi Effect
unsafe extern "C" fn ssbexo_robot_forward_smash_hi_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("head1"), 1.6, 7, 0.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST) {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("robot_eye_flash"), Hash40::new("head1"), 1.7, 4.5, 0.0, -90, -90, 0, 1.5, true);
            macros::EFFECT_FLW_POS(agent, Hash40::new("robot_attack_s4s"), Hash40::new("head1"), 1.7, 5, 0.0, -17, 0, 0, 1.5, true);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("robot_eye_flash"), Hash40::new("head1"), 1.7, 4.5, 0.0, -90, -90, 0, 1, true);
            macros::EFFECT_FLW_POS(agent, Hash40::new("robot_attack_s4s"), Hash40::new("head1"), 1.7, 5, 0.0, -17, 0, 0, 1, true);
        }
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -10, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

//Forward Smash Lw ACMD
unsafe extern "C" fn ssbexo_robot_forward_smash_lw_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_POWER_BOOST);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_EARLY_CANCEL);
        if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST) {
            MotionModule::set_rate(agent.module_accessor, 2.0);
        }
    }
    frame(agent.lua_state_agent, 16.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST) {
        if macros::is_excute(agent) {
            WorkModule::set_float(agent.module_accessor, 0.0, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BEAM_ENERGY_VALUE);
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST);
            MotionModule::set_rate(agent.module_accessor, 1.0);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 41, 85, 0, 32, 5.3, 0.0, 7.9, 19.95, Some(0.0), Some(9.8), Some(8.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 41, 85, 0, 32, 4.0, 0.0, 4.7, 34.8, Some(0.0), Some(10.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 41, 85, 0, 32, 2.8, 0.0, 1.8, 49.5, Some(0.0), Some(10.3), Some(6.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        }
    }
    else {
        if macros::is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_POWER_BOOST);
            MotionModule::set_rate(agent.module_accessor, 1.0);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 361, 105, 0, 40, 5.3, 0.0, 7.9, 13.3, Some(0.0), Some(9.8), Some(8.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 361, 110, 0, 40, 4.0, 0.0, 4.7, 23.2, Some(0.0), Some(10.0), Some(7.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 6.0, 361, 110, 0, 40, 2.8, 0.0, 1.8, 33.0, Some(0.0), Some(10.3), Some(6.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        }
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Forward Smash Lw Effect
unsafe extern "C" fn ssbexo_robot_forward_smash_lw_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("head1"), 1.6, 7, 0.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST) {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("robot_eye_flash"), Hash40::new("head1"), 1.6, 4.5, 0.0, -90, -90, 0, 1.5, true);
            macros::EFFECT_FLW_POS(agent, Hash40::new("robot_attack_s4s"), Hash40::new("head1"), 1.55, 5, 0.0, 17, 0, 0, 1.5, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 0.0);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("robot_eye_flash"), Hash40::new("head1"), 1.6, 4.5, 0.0, -90, -90, 0, 1, true);
            macros::EFFECT_FLW_POS(agent, Hash40::new("robot_attack_s4s"), Hash40::new("head1"), 1.55, 5, 0.0, 17, 0, 0, 1, true);
        }
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -10, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

//Up Smash ACMD
unsafe extern "C" fn ssbexo_robot_up_smash_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_POWER_BOOST);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_EARLY_CANCEL);
        if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST) {
            MotionModule::set_rate(agent.module_accessor, 1.25);
        }
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_POWER_BOOST);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 368, 100, 135, 0, 4.0, 0.0, 4.0, 9.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BOMB);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 368, 100, 135, 0, 4.0, 0.0, 4.0, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BOMB);
        AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 0.0, y: 22.0}, 6, false);
        AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 0.0, y: 22.0}, 6, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 14.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST) {
        if macros::is_excute(agent) {
            WorkModule::sub_float(agent.module_accessor, 96.0, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
            MotionModule::set_rate(agent.module_accessor, 1.0);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 17.0, 90, 90, 0, 40, 13.05, 0.0, 25.5, 0.0, Some(0.0), Some(38.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 90, 90, 0, 40, 9.3, 0.0, 34.0, 0.0, Some(0.0), Some(55.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        }
    }
    else {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.0);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 80, 85, 0, 40, 8.7, 0.0, 25.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 85, 85, 0, 40, 6.2, 0.0, 34.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        }
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST);
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Up Smash Effect
unsafe extern "C" fn ssbexo_robot_up_smash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -0.0, 27, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("robot_nozzle_flare"), Hash40::new("knee"), 0, 0, 0, 90, -90, 0, 1, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST) {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("robot_atk_hi4_jet"), Hash40::new("knee"), 1.5, 0, 0, 80, 90, 0, 1.65, 0, 0, 0, 0, 0, 0, true);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("robot_atk_hi4_jet"), Hash40::new("knee"), 1.5, 0, 0, 80, 90, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("robot_nozzle_flare"), false, false);
    }
}

pub fn install() {
    Agent::new("robot")
    .game_acmd("game_attacks4", ssbexo_robot_forward_smash_acmd, Priority::Low)
    .effect_acmd("effect_attacks4", ssbexo_robot_forward_smash_effect, Priority::Low)
    .game_acmd("game_attacks4hi", ssbexo_robot_forward_smash_hi_acmd, Priority::Low)
    .effect_acmd("effect_attacks4hi", ssbexo_robot_forward_smash_hi_effect, Priority::Low)
    .game_acmd("game_attacks4lw", ssbexo_robot_forward_smash_lw_acmd, Priority::Low)
    .effect_acmd("effect_attacks4lw", ssbexo_robot_forward_smash_lw_effect, Priority::Low)
    .game_acmd("game_attackhi4", ssbexo_robot_up_smash_acmd, Priority::Low)
    .effect_acmd("effect_attackhi4", ssbexo_robot_up_smash_effect, Priority::Low)
    .install()
    ;
}