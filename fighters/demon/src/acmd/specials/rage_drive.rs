use super::*;

//Rage Drive ACMD
unsafe extern "C" fn ssbexo_demon_rage_drive_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let to_heavens_door = WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TO_HEAVENS_DOOR);
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ATTACK_RAGE_CAPTURE) {
        if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
            FighterSpecializer_Demon::set_devil(boma, true, 10.0);
        }
        FT_MOTION_RATE(agent, 0.7);
        if is_excute(agent) {
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
        }
        frame(lua_state, 3.0);
        if is_excute(agent) {
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
        frame(lua_state, 10.0);
        FT_MOTION_RATE(agent, 1.0);
        frame(lua_state, 11.0);
        if is_excute(agent) {
            GrabModule::set_rebound(boma, true);
        }
        frame(lua_state, 12.0);
        if to_heavens_door {
            if is_excute(agent) {
                CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 8.0, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
                CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 6.0, 8.5, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
                CATCH(agent, 2, Hash40::new("top"), 4.0, 0.0, 8.0, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
                CATCH(agent, 3, Hash40::new("top"), 3.5, 0.0, 6.0, 8.5, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
                GrabModule::set_constraint(boma, 0, true);
                GrabModule::set_constraint(boma, 1, true);
                GrabModule::set_constraint(boma, 2, true);
                GrabModule::set_constraint(boma, 3, true);
            }
        }
        else {
            if is_excute(agent) {
                AttackModule::set_damage_shake_scale(boma, 1.5);
                ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 60, 65, 0, 80, 5.0, 0.0, 8.0, 5.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
                ATTACK(agent, 1, 0, Hash40::new("top"), 18.0, 60, 65, 0, 80, 4.5, 0.0, 6.0, 8.5, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
                ATTACK(agent, 2, 0, Hash40::new("top"), 18.0, 60, 65, 0, 80, 4.0, 0.0, 8.0, 5.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
                ATTACK(agent, 3, 0, Hash40::new("top"), 18.0, 60, 65, 0, 80, 3.5, 0.0, 6.0, 8.5, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
                ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.5);
                ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 0.5);
                ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 0.5);
                ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 0.5);
                AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
                AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
                AttackModule::set_attack_camera_quake_forced(boma, 2, *CAMERA_QUAKE_KIND_L, false);
                AttackModule::set_attack_camera_quake_forced(boma, 3, *CAMERA_QUAKE_KIND_L, false);
            }
        }
        frame(lua_state, 13.0);
        if to_heavens_door {
            if is_excute(agent) {
                CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 9.5, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
                CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 8.5, 12.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
                CATCH(agent, 2, Hash40::new("top"), 4.0, 0.0, 9.5, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
                CATCH(agent, 3, Hash40::new("top"), 3.5, 0.0, 8.5, 12.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
                GrabModule::set_constraint(boma, 0, true);
                GrabModule::set_constraint(boma, 1, true);
                GrabModule::set_constraint(boma, 2, true);
                GrabModule::set_constraint(boma, 3, true);
            }
        }
        else {
            if is_excute(agent) {
                damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
                AttackModule::set_damage_shake_scale(boma, 1.0);
                ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 70, 60, 0, 80, 5.0, 0.0, 9.5, 5.0, None, None, None, 0.17, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
                ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 70, 60, 0, 80, 4.5, 0.0, 8.5, 12.0, None, None, None, 0.17, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
                ATTACK(agent, 2, 0, Hash40::new("top"), 16.0, 70, 60, 0, 80, 4.0, 0.0, 9.5, 5.0, None, None, None, 0.17, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
                ATTACK(agent, 3, 0, Hash40::new("top"), 16.0, 70, 60, 0, 80, 3.5, 0.0, 8.5, 12.0, None, None, None, 0.17, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
                ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.5);
                ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 0.5);
                ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 0.5);
                ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 0.5);
                AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
                AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
                AttackModule::set_attack_camera_quake_forced(boma, 2, *CAMERA_QUAKE_KIND_L, false);
                AttackModule::set_attack_camera_quake_forced(boma, 3, *CAMERA_QUAKE_KIND_L, false);
            }
        }
        frame(lua_state, 14.0);
        if to_heavens_door {
            if is_excute(agent) {
                CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
                CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 19.0, 9.0, Some(0.0), Some(8.5), Some(12.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
                CATCH(agent, 2, Hash40::new("top"), 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
                CATCH(agent, 3, Hash40::new("top"), 3.5, 0.0, 21.0, 8.5, Some(0.0), Some(8.5), Some(12.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
                GrabModule::set_constraint(boma, 0, true);
                GrabModule::set_constraint(boma, 1, true);
                GrabModule::set_constraint(boma, 2, true);
                GrabModule::set_constraint(boma, 3, true);
            }
        }
        else {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 70, 50, 0, 60, 5.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), 0.17, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
                ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 70, 50, 0, 60, 4.5, 0.0, 19.0, 9.0, Some(0.0), Some(8.5), Some(12.0), 0.17, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
                ATTACK(agent, 2, 0, Hash40::new("top"), 14.0, 70, 50, 0, 60, 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), 0.17, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
                ATTACK(agent, 3, 0, Hash40::new("top"), 14.0, 70, 50, 0, 60, 3.5, 0.0, 21.0, 8.5, Some(0.0), Some(8.5), Some(12.0), 0.17, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
                ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.5);
                ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 0.5);
                ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 0.5);
                ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 0.5);
                AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
                AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
                AttackModule::set_attack_camera_quake_forced(boma, 2, *CAMERA_QUAKE_KIND_L, false);
                AttackModule::set_attack_camera_quake_forced(boma, 3, *CAMERA_QUAKE_KIND_L, false);
            }
        }
        frame(lua_state, 15.0);
        if to_heavens_door {
            if is_excute(agent) {
                CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
                CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 22.0, 7.0, Some(0.0), Some(8.5), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
                CATCH(agent, 2, Hash40::new("top"), 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
                CATCH(agent, 3, Hash40::new("top"), 3.5, 0.0, 24.0, 6.5, Some(0.0), Some(8.5), Some(10.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
                GrabModule::set_constraint(boma, 0, true);
                GrabModule::set_constraint(boma, 1, true);
                GrabModule::set_constraint(boma, 2, true);
                GrabModule::set_constraint(boma, 3, true);
            }
        }
        else {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 70, 50, 0, 60, 5.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
                ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 70, 50, 0, 60, 4.5, 0.0, 22.0, 9.0, Some(0.0), Some(8.5), Some(10.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
                ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 70, 50, 0, 60, 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
                ATTACK(agent, 3, 0, Hash40::new("top"), 12.0, 70, 50, 0, 60, 3.5, 0.0, 24.0, 8.5, Some(0.0), Some(8.5), Some(10.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
                AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_M, false);
                AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_M, false);
                AttackModule::set_attack_camera_quake_forced(boma, 2, *CAMERA_QUAKE_KIND_M, false);
                AttackModule::set_attack_camera_quake_forced(boma, 3, *CAMERA_QUAKE_KIND_M, false);
            }
        }
        frame(lua_state, 16.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
            grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            GrabModule::set_rebound(boma, false);
            WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TO_HEAVENS_DOOR);
        }
        frame(lua_state, 38.0);
        FT_MOTION_RATE(agent, 1.3);
        frame(lua_state, 52.0);
        if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
            FighterSpecializer_Demon::set_devil(boma, false, 0.0);
        }
    }
    else {
        if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
            FighterSpecializer_Demon::set_devil(boma, true, 10.0);
        }
        if is_excute(agent) {
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 70, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
        }
        frame(lua_state, 4.0);
        if is_excute(agent) {
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
        frame(lua_state, 12.0);
        if is_excute(agent) {
            GrabModule::set_rebound(boma, true);
        }
        frame(lua_state, 13.0);
        if to_heavens_door {
            if is_excute(agent) {
                CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 9.5, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
                CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 8.5, 12.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
                CATCH(agent, 2, Hash40::new("top"), 4.0, 0.0, 9.5, 5.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
                CATCH(agent, 3, Hash40::new("top"), 3.5, 0.0, 8.5, 12.0, None, None, None, *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
                GrabModule::set_constraint(boma, 0, true);
                GrabModule::set_constraint(boma, 1, true);
                GrabModule::set_constraint(boma, 2, true);
                GrabModule::set_constraint(boma, 3, true);
            }
        }
        else {
            if is_excute(agent) {
                damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
                AttackModule::set_damage_shake_scale(boma, 1.5);
                ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 60, 65, 0, 80, 5.0, 0.0, 9.5, 5.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
                ATTACK(agent, 1, 0, Hash40::new("top"), 18.0, 60, 65, 0, 80, 4.5, 0.0, 8.5, 12.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
                ATTACK(agent, 2, 0, Hash40::new("top"), 18.0, 60, 65, 0, 80, 4.0, 0.0, 9.5, 5.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
                ATTACK(agent, 3, 0, Hash40::new("top"), 18.0, 60, 65, 0, 80, 3.5, 0.0, 8.5, 12.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
                ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.5);
                ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 0.5);
                ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 0.5);
                ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 0.5);
                AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
                AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
                AttackModule::set_attack_camera_quake_forced(boma, 2, *CAMERA_QUAKE_KIND_L, false);
                AttackModule::set_attack_camera_quake_forced(boma, 3, *CAMERA_QUAKE_KIND_L, false);
            }
        }
        frame(lua_state, 14.0);
        if to_heavens_door {
            if is_excute(agent) {
                CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
                CATCH(agent, 1, Hash40::new("top"), 4.5, 0.0, 19.0, 9.0, Some(0.0), Some(8.5), Some(12.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_G);
                CATCH(agent, 2, Hash40::new("top"), 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
                CATCH(agent, 3, Hash40::new("top"), 3.5, 0.0, 21.0, 8.5, Some(0.0), Some(8.5), Some(12.0), *FIGHTER_STATUS_KIND_DEMON_DIVED, *COLLISION_SITUATION_MASK_A);
                GrabModule::set_constraint(boma, 0, true);
                GrabModule::set_constraint(boma, 1, true);
                GrabModule::set_constraint(boma, 2, true);
                GrabModule::set_constraint(boma, 3, true);
            }
        }
        else {
            if is_excute(agent) {
                AttackModule::set_damage_shake_scale(boma, 1.0);
                ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 70, 60, 0, 80, 5.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), 0.17, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
                ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 70, 60, 0, 80, 4.5, 0.0, 19.0, 9.0, Some(0.0), Some(8.5), Some(12.0), 0.17, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
                ATTACK(agent, 2, 0, Hash40::new("top"), 16.0, 70, 60, 0, 80, 4.0, 0.0, 14.0, 5.0, Some(0.0), Some(9.5), Some(5.0), 0.17, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
                ATTACK(agent, 3, 0, Hash40::new("top"), 16.0, 70, 60, 0, 80, 3.5, 0.0, 21.0, 8.5, Some(0.0), Some(8.5), Some(12.0), 0.17, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
                ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.5);
                ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 0.5);
                ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 0.5);
                ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 0.5);
                AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
                AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
                AttackModule::set_attack_camera_quake_forced(boma, 2, *CAMERA_QUAKE_KIND_L, false);
                AttackModule::set_attack_camera_quake_forced(boma, 3, *CAMERA_QUAKE_KIND_L, false);
            }
        }
        frame(lua_state, 15.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
            grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            GrabModule::set_rebound(boma, false);
            WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TO_HEAVENS_DOOR);
        }
        frame(lua_state, 38.0);
        FT_MOTION_RATE(agent, 1.3);
        frame(lua_state, 52.0);
        if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
            FighterSpecializer_Demon::set_devil(boma, false, 0.0);
        }
    }
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_attackragedrive", ssbexo_demon_rage_drive_acmd, Low)
    .acmd("game_attackairragedrive", ssbexo_demon_rage_drive_acmd, Low)
    .install()
    ;
}