use super::*;

//Laser Fly Hi ACMD
unsafe extern "C" fn ssbexo_demon_laser_fly_hi_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 60, 35, 0, 80, 1.0, 0.0, 0.0, 0.0, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 50, 35, 0, 80, 7.0, 0.0, -3.0, 6.0, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.1);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.1);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 0.1, false);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 1, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 60, 35, 0, 80, 2.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-10.0), 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 10.0, false);
    }
}

//Laser Fly Lw ACMD
unsafe extern "C" fn ssbexo_demon_laser_fly_lw_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 60, 35, 0, 80, 1.0, 0.0, 0.0, 0.0, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 50, 35, 0, 80, 7.0, 0.0, -3.0, 6.0, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.1);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.1);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 0.1, false);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 1, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 60, 35, 0, 80, 2.5, 0.0, 0.0, -1.0, Some(0.0), Some(0.0), Some(-11.0), 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 10.0, false);
    }
}

//Laser Fly Air N ACMD
unsafe extern "C" fn ssbexo_demon_laser_fly_air_n_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 60, 35, 0, 80, 1.0, 0.0, 0.0, 0.0, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 50, 35, 0, 80, 7.0, 0.0, -3.0, 6.0, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.1);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.1);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 0.1, false);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 1, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 60, 35, 0, 80, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 10.0, false);
    }
}

//Laser Fly Air Lw ACMD
unsafe extern "C" fn ssbexo_demon_laser_fly_air_lw_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 60, 35, 0, 80, 1.0, 0.0, 0.0, 0.0, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 50, 35, 0, 80, 7.0, 0.0, -3.0, 6.0, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.1);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.1);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 0.1, false);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 1, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 60, 35, 0, 80, 2.5, 0.0, -1.0, 0.8, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 10.0, false);
    }
}

//Grounded Side Special ACMD
unsafe extern "C" fn ssbexo_demon_grounded_side_special_acmd(agent: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 10.0);
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.5);
    frame(agent.lua_state_agent, 4.0);
    frame(agent.lua_state_agent, 5.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 5.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 12.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 4.0);
    frame(agent.lua_state_agent, 14.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 60, 50, 0, 80, 4.0, 0.0, 8.0, 3.0, Some(0.0), Some(8.0), Some(10.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fist_down2"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 60, 50, 0, 80, 4.0, 0.0, 8.0, 3.0, Some(0.0), Some(8.0), Some(10.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fist_down2"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_SPECIAL_S_FLAG_CHANGE_HIT);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 16.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 1.0);
    frame(agent.lua_state_agent, 19.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 2.0);
    frame(agent.lua_state_agent, 24.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 3.0);
    frame(agent.lua_state_agent, 41.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 5.0);
    frame(agent.lua_state_agent, 58.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 6.0);
    frame(agent.lua_state_agent, 60.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 7.0);
    frame(agent.lua_state_agent, 61.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 8.0);
    frame(agent.lua_state_agent, 62.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, false, 0.0);
}

//Aerial Side Special ACMD
unsafe extern "C" fn ssbexo_demon_aerial_side_special_acmd(agent: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 10.0);
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.5);
    frame(agent.lua_state_agent, 4.0);
    frame(agent.lua_state_agent, 5.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 5.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 4.0);
    frame(agent.lua_state_agent, 14.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 60, 50, 0, 80, 4.0, 0.0, 8.0, 3.0, Some(0.0), Some(8.0), Some(10.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fist_down2"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 60, 50, 0, 80, 4.0, 0.0, 8.0, 3.0, Some(0.0), Some(8.0), Some(10.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fist_down2"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_SPECIAL_S_FLAG_CHANGE_HIT);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 16.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 1.0);
    frame(agent.lua_state_agent, 19.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 2.0);
    frame(agent.lua_state_agent, 24.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 3.0);
    frame(agent.lua_state_agent, 41.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 5.0);
    frame(agent.lua_state_agent, 58.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 6.0);
    frame(agent.lua_state_agent, 60.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 7.0);
    frame(agent.lua_state_agent, 61.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 8.0);
    frame(agent.lua_state_agent, 62.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, false, 0.0);
}

//Grounded Down Special ACMD
unsafe extern "C" fn ssbexo_demon_grounded_down_special_acmd(agent: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 8.75, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 3.5, 0.0, 7.75, 12.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 2, Hash40::new("top"), 3.0, 0.0, 8.75, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(agent, 3, Hash40::new("top"), 2.0, 0.0, 7.75, 12.5, Some(0.0), Some(7.75), Some(9.5), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        GrabModule::set_constraint(agent.module_accessor, 0, true);
        GrabModule::set_constraint(agent.module_accessor, 1, true);
        GrabModule::set_constraint(agent.module_accessor, 2, true);
        GrabModule::set_constraint(agent.module_accessor, 3, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(8.75), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 3.5, 0.0, 19.0, 9.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 2, Hash40::new("top"), 3.0, 0.0, 14.0, 5.0, Some(0.0), Some(8.75), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(agent, 3, Hash40::new("top"), 2.0, 0.0, 19.0, 9.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        GrabModule::set_constraint(agent.module_accessor, 0, true);
        GrabModule::set_constraint(agent.module_accessor, 1, true);
        GrabModule::set_constraint(agent.module_accessor, 2, true);
        GrabModule::set_constraint(agent.module_accessor, 3, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::SET_SPEED_EX(agent, 0.1, 1.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    macros::FT_MOTION_RATE(agent, 0.9);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(agent.module_accessor, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 19.0, 4.5, Some(0.0), Some(23.0), Some(4.5), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 60, 60, 0, 80, 3.0, 0.0, 12.0, 6.0, Some(0.0), Some(16.0), Some(6.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 64, 50, 0, 75, 3.0, 0.0, 3.0, 4.0, Some(0.0), Some(3.0), Some(5.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_down_only(agent.module_accessor, 2, true);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 2, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 19.5, 4.5, Some(0.0), Some(23.5), Some(3.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 60, 60, 0, 80, 3.0, 0.0, 12.0, 6.0, Some(0.0), Some(16.0), Some(6.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 20.0, 4.5, Some(0.0), Some(24.0), Some(3.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, false, 0.0);
}

//Aerial Down Special ACMD
unsafe extern "C" fn ssbexo_demon_aerial_down_special_acmd(agent: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 8.75, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 3.5, 0.0, 7.75, 12.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 2, Hash40::new("top"), 3.0, 0.0, 8.75, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(agent, 3, Hash40::new("top"), 2.0, 0.0, 7.75, 12.5, Some(0.0), Some(7.75), Some(9.5), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        GrabModule::set_constraint(agent.module_accessor, 0, true);
        GrabModule::set_constraint(agent.module_accessor, 1, true);
        GrabModule::set_constraint(agent.module_accessor, 2, true);
        GrabModule::set_constraint(agent.module_accessor, 3, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 15.0, 5.0, Some(0.0), Some(8.75), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 3.5, 0.0, 20.0, 9.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 2, Hash40::new("top"), 3.0, 0.0, 15.0, 5.0, Some(0.0), Some(8.75), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(agent, 3, Hash40::new("top"), 2.0, 0.0, 20.0, 9.0, Some(0.0), Some(7.75), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
        GrabModule::set_constraint(agent.module_accessor, 0, true);
        GrabModule::set_constraint(agent.module_accessor, 1, true);
        GrabModule::set_constraint(agent.module_accessor, 2, true);
        GrabModule::set_constraint(agent.module_accessor, 3, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::SET_SPEED_EX(agent, 0.1, 1.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    macros::FT_MOTION_RATE(agent, 0.9);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(agent.module_accessor, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 21.0, 4.5, Some(0.0), Some(25.0), Some(4.5), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 60, 60, 0, 80, 3.0, 0.0, 14.0, 6.0, Some(0.0), Some(18.0), Some(6.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 64, 50, 0, 75, 3.0, 0.0, 3.0, 4.0, Some(0.0), Some(3.0), Some(5.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_down_only(agent.module_accessor, 2, true);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 2, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 21.5, 4.5, Some(0.0), Some(25.5), Some(3.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 60, 60, 0, 80, 3.0, 0.0, 14.0, 6.0, Some(0.0), Some(18.0), Some(6.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 60, 0, 80, 3.0, 0.0, 22.0, 4.5, Some(0.0), Some(26.0), Some(3.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, false, 0.0);
}

//Rage Drive ACMD
unsafe extern "C" fn ssbexo_demon_rage_drive_acmd(agent: &mut L2CAgentBase) {
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ATTACK_RAGE_CAPTURE) {
        FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 10.0);
        macros::FT_MOTION_RATE(agent, 0.7);
        if macros::is_excute(agent) {
            macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
        }
        frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
        frame(agent.lua_state_agent, 10.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            GrabModule::set_rebound(agent.module_accessor, true);
        }
        frame(agent.lua_state_agent, 12.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 5.0, 0.0, 8.0, 5.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 4.5, 0.0, 6.0, 8.5, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 4.0, 0.0, 8.0, 5.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 3.5, 0.0, 6.0, 8.5, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
        if WorkModule::is_flag(agent.module_accessor, FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_IS_VALID_RAGE_DRIVE) {
            AttackModule::clear_all(agent.module_accessor);
            macros::CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 8.0, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 6.0, 8.5, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 2, Hash40::new("top"), 4.0, 0.0, 8.0, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            macros::CATCH(agent, 3, Hash40::new("top"), 3.5, 0.0, 6.0, 8.5, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(agent.module_accessor, 0, true);
            GrabModule::set_constraint(agent.module_accessor, 1, true);
            GrabModule::set_constraint(agent.module_accessor, 2, true);
            GrabModule::set_constraint(agent.module_accessor, 3, true);
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 5.0, 0.0, 9.5, 5.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 4.5, 0.0, 8.5, 12.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 4.0, 0.0, 9.5, 5.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 3.5, 0.0, 8.5, 12.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
        if WorkModule::is_flag(agent.module_accessor, FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_IS_VALID_RAGE_DRIVE) {
            AttackModule::clear_all(agent.module_accessor);
            macros::CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 9.5, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 8.5, 12.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 2, Hash40::new("top"), 4.0, 0.0, 9.5, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            macros::CATCH(agent, 3, Hash40::new("top"), 3.5, 0.0, 8.5, 12.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(agent.module_accessor, 0, true);
            GrabModule::set_constraint(agent.module_accessor, 1, true);
            GrabModule::set_constraint(agent.module_accessor, 2, true);
            GrabModule::set_constraint(agent.module_accessor, 3, true);
        }
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 5.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 4.5, 0.0, 19.0, 9.0, Some(0.0), Some(8.5), Some(12.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 3.5, 0.0, 21.0, 8.5, Some(0.0), Some(8.5), Some(12.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
        if WorkModule::is_flag(agent.module_accessor, FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_IS_VALID_RAGE_DRIVE) {
            AttackModule::clear_all(agent.module_accessor);
            macros::CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 19.0, 9.0, Some(0.0), Some(8.5), Some(12.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 2, Hash40::new("top"), 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            macros::CATCH(agent, 3, Hash40::new("top"), 3.5, 0.0, 21.0, 8.5, Some(0.0), Some(8.5), Some(12.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(agent.module_accessor, 0, true);
            GrabModule::set_constraint(agent.module_accessor, 1, true);
            GrabModule::set_constraint(agent.module_accessor, 2, true);
            GrabModule::set_constraint(agent.module_accessor, 3, true);
        }
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 5.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 4.5, 0.0, 22.0, 9.0, Some(0.0), Some(8.5), Some(10.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 3.5, 0.0, 24.0, 8.5, Some(0.0), Some(8.5), Some(10.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
        if WorkModule::is_flag(agent.module_accessor, FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_IS_VALID_RAGE_DRIVE) {
            AttackModule::clear_all(agent.module_accessor);
            macros::CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 22.0, 7.0, Some(0.0), Some(8.5), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 2, Hash40::new("top"), 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            macros::CATCH(agent, 3, Hash40::new("top"), 3.5, 0.0, 24.0, 6.5, Some(0.0), Some(8.5), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(agent.module_accessor, 0, true);
            GrabModule::set_constraint(agent.module_accessor, 1, true);
            GrabModule::set_constraint(agent.module_accessor, 2, true);
            GrabModule::set_constraint(agent.module_accessor, 3, true);
        }
        frame(agent.lua_state_agent, 16.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            GrabModule::set_rebound(agent.module_accessor, false);
        }
        frame(agent.lua_state_agent, 38.0);
        macros::FT_MOTION_RATE(agent, 1.3);
        frame(agent.lua_state_agent, 52.0);
        FighterSpecializer_Demon::set_devil(agent.module_accessor, false, 0.0);
    }
    else {
        FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 10.0);
        if macros::is_excute(agent) {
            macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
        }
        frame(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
        frame(agent.lua_state_agent, 12.0);
        if macros::is_excute(agent) {
            GrabModule::set_rebound(agent.module_accessor, true);
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 5.0, 0.0, 9.5, 5.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 4.5, 0.0, 8.5, 12.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 4.0, 0.0, 9.5, 5.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 3.5, 0.0, 8.5, 12.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
        if WorkModule::is_flag(agent.module_accessor, FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_IS_VALID_RAGE_DRIVE) {
            AttackModule::clear_all(agent.module_accessor);
            macros::CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 9.5, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 8.5, 12.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 2, Hash40::new("top"), 4.0, 0.0, 9.5, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            macros::CATCH(agent, 3, Hash40::new("top"), 3.5, 0.0, 8.5, 12.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(agent.module_accessor, 0, true);
            GrabModule::set_constraint(agent.module_accessor, 1, true);
            GrabModule::set_constraint(agent.module_accessor, 2, true);
            GrabModule::set_constraint(agent.module_accessor, 3, true);
        }
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 5.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 4.5, 0.0, 19.0, 9.0, Some(0.0), Some(8.5), Some(12.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 3.5, 0.0, 21.0, 8.5, Some(0.0), Some(8.5), Some(12.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
        if WorkModule::is_flag(agent.module_accessor, FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_IS_VALID_RAGE_DRIVE) {
            AttackModule::clear_all(agent.module_accessor);
            macros::CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 19.0, 9.0, Some(0.0), Some(8.5), Some(12.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 2, Hash40::new("top"), 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            macros::CATCH(agent, 3, Hash40::new("top"), 3.5, 0.0, 21.0, 8.5, Some(0.0), Some(8.5), Some(12.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
            GrabModule::set_constraint(agent.module_accessor, 0, true);
            GrabModule::set_constraint(agent.module_accessor, 1, true);
            GrabModule::set_constraint(agent.module_accessor, 2, true);
            GrabModule::set_constraint(agent.module_accessor, 3, true);
        }
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            GrabModule::set_rebound(agent.module_accessor, false);
        }
        frame(agent.lua_state_agent, 38.0);
        macros::FT_MOTION_RATE(agent, 1.3);
        frame(agent.lua_state_agent, 52.0);
        FighterSpecializer_Demon::set_devil(agent.module_accessor, false, 0.0);
    }
}

pub fn install() {
    Agent::new("demon")
    .game_acmd("game_specials", ssbexo_demon_grounded_side_special_acmd, Priority::Low)
    .game_acmd("game_specialairs", ssbexo_demon_aerial_side_special_acmd, Priority::Low)
    .game_acmd("game_speciallw", ssbexo_demon_grounded_down_special_acmd, Priority::Low)
    .game_acmd("game_specialairlw", ssbexo_demon_aerial_down_special_acmd, Priority::Low)
    .game_acmd("game_attackragedrive", ssbexo_demon_rage_drive_acmd, Priority::Low)
    .install()
    ;
    Agent::new("demon_blaster")
    .game_acmd("game_flyhi", ssbexo_demon_laser_fly_hi_acmd, Priority::Low)
    .game_acmd("game_flylw", ssbexo_demon_laser_fly_lw_acmd, Priority::Low)
    .game_acmd("game_flyairn", ssbexo_demon_laser_fly_air_n_acmd, Priority::Low)
    .game_acmd("game_flyairlw", ssbexo_demon_laser_fly_air_lw_acmd, Priority::Low)
    .install()
    ;
}