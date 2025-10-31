use super::*;

//Punisher Forward Smash Charge ACMD
unsafe extern "C" fn ssbexo_cloud_punisher_forward_smash_charge_acmd(_agent: &mut L2CAgentBase) {}

//Punisher Forward Smash Charge Effect
unsafe extern "C" fn ssbexo_cloud_punisher_forward_smash_charge_effect(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        frame(agent.lua_state_agent, 5.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 4, 0, 10, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 5.0);
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 3, 3, 3, 0, 0, 0, true);
        }
    }
}

//Punisher Forward Smash Charge Sound
unsafe extern "C" fn ssbexo_cloud_punisher_forward_smash_charge_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
}

//Punisher Forward Smash Charge Expression
unsafe extern "C" fn ssbexo_cloud_punisher_forward_smash_charge_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        physics!(agent, *MA_MSC_CMD_PHYSICS_START_CHARGE, 0.7, 0.7, -1, 0.7, 0.7, -1, Hash40::new("invalid"));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 61.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold2"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Punisher Forward Smash ACMD
unsafe extern "C" fn ssbexo_cloud_punisher_forward_smash_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        JostleModule::set_team(agent.module_accessor, 1);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 4.0, 30, 40, 40, 0, 6.0, 3.0, 13.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_SMASH_01, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 4.0, 30, 40, 40, 0, 6.0, 3.0, 9.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_SMASH_01, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 4.0, 30, 40, 40, 0, 6.0, 3.0, 4.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_SMASH_01, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 12.0, false);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 33.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 4.0, 30, 40, 40, 0, 6.0, 3.0, 13.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_SMASH_02, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 4.0, 30, 40, 60, 0, 6.0, 3.0, 9.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_SMASH_02, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 4.0, 30, 40, 80, 0, 6.0, 3.0, 4.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_SMASH_02, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 24.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 24.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 24.0, false);
    }
    frame(agent.lua_state_agent, 36.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 54.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.3, 361, 30, 20, 0, 5.5, 0.0, 9.0, 10.0, Some(0.0), Some(9.0), Some(6.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_SMASH_03, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.3, 361, 30, 20, 0, 5.5, 0.0, 9.0, 16.0, Some(0.0), Some(9.0), Some(6.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_SMASH_03, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 55.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 56.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.3, 361, 30, 20, 0, 5.5, 0.0, 9.0, 10.0, Some(0.0), Some(9.0), Some(6.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_SMASH_03, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.3, 361, 30, 20, 0, 5.5, 0.0, 9.0, 16.0, Some(0.0), Some(9.0), Some(6.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_SMASH_03, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 57.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 58.0);
    if is_excute(agent) {
        
        ATTACK(agent, 0, 0, Hash40::new("haver"), 4.3, 361, 148, 0, 80, 6.0, 3.0, 13.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_SMASH_03, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 4.3, 361, 148, 0, 80, 6.0, 3.0, 9.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_SMASH_03, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 4.3, 361, 138, 0, 80, 6.0, 3.0, 4.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_SMASH_03, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 59.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        JostleModule::set_team(agent.module_accessor, 0);
    }
}

//Punisher Forward Smash Effect
unsafe extern "C" fn ssbexo_cloud_punisher_forward_smash_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 17, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.5);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 4, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.19, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
    frame(agent.lua_state_agent, 32.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.5);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 4, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.19, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 35.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 36.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
    frame(agent.lua_state_agent, 53.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.5);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 4, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.19, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 56.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 60.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
}

//Punisher Forward Smash Sound
unsafe extern "C" fn ssbexo_cloud_punisher_forward_smash_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_cloud_attack05"));
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_smash_s01"));
    }
    frame(agent.lua_state_agent, 33.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_smash_s02"));
    }
    frame(agent.lua_state_agent, 54.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_smash_s03"));
    }
}

//Punisher Forward Smash Expression
unsafe extern "C" fn ssbexo_cloud_punisher_forward_smash_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 13.0);
    sv_animcmd::execute(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
    frame(agent.lua_state_agent, 32.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 53.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 54.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

//Punisher Up Smash ACMD
unsafe extern "C" fn ssbexo_cloud_punisher_up_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 18.0, 90, 60, 0, 40, 6.0, 3.0, 13.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 18.0, 90, 60, 0, 40, 6.0, 3.0, 9.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 18.0, 90, 60, 0, 40, 6.0, 3.0, 4.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 33.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Punisher Up Smash Effect
unsafe extern "C" fn ssbexo_cloud_punisher_up_smash_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 17, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.2);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 4, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.19, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 32.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 44.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
}

//Punisher Up Smash Sound
unsafe extern "C" fn ssbexo_cloud_punisher_up_smash_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_smash_h01"));
         PLAY_SEQUENCE(agent, Hash40::new("seq_cloud_rnd_attack_smash_h"));
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_smash_h02"));
    }
}

//Punisher Up Smash Expression
unsafe extern "C" fn ssbexo_cloud_punisher_up_smash_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 18.0);
    sv_animcmd::execute(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

//Punisher Down Smash Charge ACMD
unsafe extern "C" fn ssbexo_cloud_punisher_down_smash_charge_acmd(_agent: &mut L2CAgentBase) {}

//Punisher Down Smash Charge Effect
unsafe extern "C" fn ssbexo_cloud_punisher_down_smash_charge_effect(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        frame(agent.lua_state_agent, 5.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 4, 0, 10, 0, 0, 0, false);
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_dead"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.5, true);
        }
        wait(agent.lua_state_agent, 5.0);
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 3, 3, 3, 0, 0, 0, true);
            EFFECT_OFF_KIND(agent, Hash40::new("sys_hit_dead"), false, true);
        }
    }
}

//Punisher Down Smash Charge Sound
unsafe extern "C" fn ssbexo_cloud_punisher_down_smash_charge_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
}

//Punisher Down Smash Charge Expression
unsafe extern "C" fn ssbexo_cloud_punisher_down_smash_charge_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        physics!(agent, *MA_MSC_CMD_PHYSICS_START_CHARGE, 0.7, 0.7, -1, 0.7, 0.7, -1, Hash40::new("invalid"));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 61.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold2"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Punisher Down Smash ACMD
unsafe extern "C" fn ssbexo_cloud_punisher_down_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 16.0, 280, 90, 0, 20, 6.0, 3.0, 13.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 16.0, 280, 90, 0, 20, 6.0, 3.0, 9.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 16.0, 280, 90, 0, 20, 6.0, 3.0, 4.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        ATTACK(agent, 3, 0, Hash40::new("top"), 4.0, 15, 30, 0, 80, 3.0, 0.0, 2.0, 25.0, Some(0.0), Some(2.0), Some(8.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Punisher Down Smash Effect
unsafe extern "C" fn ssbexo_cloud_punisher_down_smash_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 17, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_dead"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.5, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_dead"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.5, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.2);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 4, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.19, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_dead"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.5, true);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), 20, 0, 3.0, 0, 0, -180, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
}

//Punisher Down Smash Sound
unsafe extern "C" fn ssbexo_cloud_punisher_down_smash_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new_raw(0x22591898c7));
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_attackair_f01"));
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_special_h04"));
    }
}

//Punisher Down Smash Expression
unsafe extern "C" fn ssbexo_cloud_punisher_down_smash_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 12.0);
    sv_animcmd::execute(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2, true);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2);
            ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        }
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_punishattacks4charge", ssbexo_cloud_punisher_forward_smash_charge_acmd, Low)
    .effect_acmd("effect_punishattacks4charge", ssbexo_cloud_punisher_forward_smash_charge_effect, Low)
    .sound_acmd("sound_punishattacks4charge", ssbexo_cloud_punisher_forward_smash_charge_sound, Low)
    .expression_acmd("expression_punishattacks4charge", ssbexo_cloud_punisher_forward_smash_charge_expression, Low)
    .game_acmd("game_punishattacks4", ssbexo_cloud_punisher_forward_smash_acmd, Low)
    .effect_acmd("effect_punishattacks4", ssbexo_cloud_punisher_forward_smash_effect, Low)
    .sound_acmd("sound_punishattacks4", ssbexo_cloud_punisher_forward_smash_sound, Low)
    .expression_acmd("expression_punishattacks4", ssbexo_cloud_punisher_forward_smash_expression, Low)
    .game_acmd("game_punishattackhi4", ssbexo_cloud_punisher_up_smash_acmd, Low)
    .effect_acmd("effect_punishattackhi4", ssbexo_cloud_punisher_up_smash_effect, Low)
    .sound_acmd("sound_punishattackhi4", ssbexo_cloud_punisher_up_smash_sound, Low)
    .expression_acmd("expression_punishattackhi4", ssbexo_cloud_punisher_up_smash_expression, Low)
    .game_acmd("game_punishattacklw4charge", ssbexo_cloud_punisher_down_smash_charge_acmd, Low)
    .effect_acmd("effect_punishattacklw4charge", ssbexo_cloud_punisher_down_smash_charge_effect, Low)
    .sound_acmd("sound_punishattacklw4charge", ssbexo_cloud_punisher_down_smash_charge_sound, Low)
    .expression_acmd("expression_punishattacklw4charge", ssbexo_cloud_punisher_down_smash_charge_expression, Low)
    .game_acmd("game_punishattacklw4", ssbexo_cloud_punisher_down_smash_acmd, Low)
    .effect_acmd("effect_punishattacklw4", ssbexo_cloud_punisher_down_smash_effect, Low)
    .sound_acmd("sound_punishattacklw4", ssbexo_cloud_punisher_down_smash_sound, Low)
    .expression_acmd("expression_punishattacklw4", ssbexo_cloud_punisher_down_smash_expression, Low)
    .install()
    ;
}