use super::*;

//Forward Smash ACMD
unsafe extern "C" fn ssbexo_demon_forward_smash_acmd(agent: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 10.0);
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, -1, *FIGHTER_DEMON_STATUS_ATTACK_S4_WORK_INT_CRITICAL_HIT_NO);
    }
    FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10);
    }
    frame(agent.lua_state_agent, 10.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 7.0);
    frame(agent.lua_state_agent, 11.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 5.0);
    frame(agent.lua_state_agent, 13.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 4.0);
    frame(agent.lua_state_agent, 14.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 3.0);
    frame(agent.lua_state_agent, 21.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("handl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.0, 1.0, 1.0, 1.0, false, 1.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        WorkModule::set_int(agent.module_accessor, 1, *FIGHTER_DEMON_STATUS_ATTACK_S4_WORK_INT_CRITICAL_HIT_NO);
        ATTACK(agent, 0, 0, Hash40::new("top"), 17.0, 361, 63, 0, 50, 3.0, 0.0, 8.5, 4.0, Some(0.0), Some(13.5), Some(4.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("handl"), 20.0, 361, 73, 0, 50, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.1);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.75);
    }
    wait(agent.lua_state_agent, 2.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_S4_FLAG_HIT) {
        if is_excute(agent) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
    }
    frame(agent.lua_state_agent, 51.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 4.0);
    frame(agent.lua_state_agent, 53.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 5.0);
    frame(agent.lua_state_agent, 57.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 6.0);
    frame(agent.lua_state_agent, 58.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 7.0);
    frame(agent.lua_state_agent, 59.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 8.0);
    frame(agent.lua_state_agent, 61.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, false, 0.0);
}

//Up Smash ACMD
unsafe extern "C" fn ssbexo_demon_up_smash_acmd(agent: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 10.0);
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE(agent, 0.7);
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 9.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 7.0);
    frame(agent.lua_state_agent, 11.0);
    FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 12.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 6.0);
    frame(agent.lua_state_agent, 16.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        ATTACK(agent, 0, 0, Hash40::new("handr"), 16.0, 90, 62, 0, 65, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 90, 62, 0, 65, 4.75, 0.0, 9.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 16.0, 90, 62, 0, 65, 3.5, 0.0, 11.0, 3.5, Some(0.0), Some(8.0), Some(2.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 0, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 2, false);
        ATTACK(agent, 0, 0, Hash40::new("handr"), 16.0, 90, 62, 0, 65, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("handr"), 16.0, 90, 62, 0, 65, 4.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 0, *CAMERA_QUAKE_KIND_M, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("handr"), 12.0, 90, 62, 0, 65, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("handr"), 12.0, 90, 62, 0, 65, 4.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 0, *CAMERA_QUAKE_KIND_S, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    FT_MOTION_RATE(agent, 0.8);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 51.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 7.0);
    frame(agent.lua_state_agent, 53.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 8.0);
    frame(agent.lua_state_agent, 54.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 9.0);
    frame(agent.lua_state_agent, 56.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, false, 0.0);
}

//Down Smash ACMD
unsafe extern "C" fn ssbexo_demon_down_smash_acmd(agent: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 10.0);
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 12.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 8.0);
    frame(agent.lua_state_agent, 13.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 7.0);
    frame(agent.lua_state_agent, 14.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 6.0);
    frame(agent.lua_state_agent, 15.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 5.0);
    frame(agent.lua_state_agent, 16.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 3.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        ATTACK(agent, 0, 0, Hash40::new("top"), 17.0, 277, 10, 0, 100, 4.0, 0.0, 8.2, 11.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 277, 10, 0, 100, 3.0, 0.0, 8.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 17.0, 277, 80, 0, 20, 4.0, 0.0, 8.2, 11.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("top"), 15.0, 277, 80, 0, 20, 3.0, 0.0, 8.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 5.0, false);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 1.2);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 2, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 3, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 17.0, 277, 10, 0, 100, 4.0, 0.0, 6.3, 11.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 277, 10, 0, 100, 3.0, 0.0, 6.3, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 17.0, 277, 80, 0, 20, 4.0, 0.0, 6.3, 11.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("top"), 15.0, 277, 80, 0, 20, 3.0, 0.0, 6.3, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 5.0, false);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 1.2);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 2, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 3, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 17.0, 277, 10, 0, 100, 4.0, 0.0, 2.5, 11.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 277, 10, 0, 100, 3.0, 0.0, 3.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 13.0, 277, 80, 0, 20, 4.0, -5.0, 2.5, 11.5, Some(5.0), Some(2.5), Some(11.5), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("top"), 11.0, 277, 80, 0, 20, 3.0, 0.0, 3.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 5.0, false);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 1.2);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 2, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(agent.module_accessor, 3, *CAMERA_QUAKE_KIND_L, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 46.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 5.0);
    frame(agent.lua_state_agent, 47.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 6.0);
    frame(agent.lua_state_agent, 48.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 7.0);
    frame(agent.lua_state_agent, 50.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 8.0);
    frame(agent.lua_state_agent, 52.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, true, 9.0);
    frame(agent.lua_state_agent, 53.0);
    FighterSpecializer_Demon::set_devil(agent.module_accessor, false, 0.0);
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attacks4", ssbexo_demon_forward_smash_acmd, Low)
    .game_acmd("game_attackhi4", ssbexo_demon_up_smash_acmd, Low)
    .game_acmd("game_attacklw4", ssbexo_demon_down_smash_acmd, Low)
    .install()
    ;
}