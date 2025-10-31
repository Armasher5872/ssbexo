use super::*;

//Grounded Side Special ACMD
unsafe extern "C" fn ssbexo_cloud_grounded_side_special_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.333);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 12.0, 88, 76, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 12.0, 88, 76, 0, 40, 5.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 12.0, 88, 76, 0, 40, 5.0, 0.0, 14.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Aerial Side Special ACMD
unsafe extern "C" fn ssbexo_cloud_aerial_side_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 12.0, 70, 77, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 12.0, 70, 77, 0, 30, 5.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 12.0, 70, 77, 0, 30, 5.0, 0.0, 14.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Grounded Side Special Effect
unsafe extern "C" fn ssbexo_cloud_grounded_side_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.2);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1_blue"), Hash40::new("tex_cloud_sword2"), 16, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, true, true);
    }
}

//Aerial Side Special Effect
unsafe extern "C" fn ssbexo_cloud_aerial_side_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.2);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1_blue"), Hash40::new("tex_cloud_sword2"), 16, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, true, true);
    }
}

//Grounded Side Special Sound
unsafe extern "C" fn ssbexo_cloud_grounded_side_special_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_special_s01"));
    }
}

//Aerial Side Special Sound
unsafe extern "C" fn ssbexo_cloud_aerial_side_special_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_special_s01"));
    }
}

//Grounded Side Special Expression
unsafe extern "C" fn ssbexo_cloud_grounded_side_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 45.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

//Aerial Side Special Expression
unsafe extern "C" fn ssbexo_cloud_aerial_side_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 55.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

//Grounded Punisher Side Special ACMD
unsafe extern "C" fn ssbexo_cloud_grounded_punisher_side_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 12.0, 140, 100, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 3.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 12.0, 140, 100, 0, 30, 5.0, 0.0, 6.0, 0.0, None, None, None, 3.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 12.0, 140, 100, 0, 30, 5.0, 0.0, 14.5, 0.0, None, None, None, 3.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 12.0, 160, 100, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 3.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 12.0, 160, 100, 0, 30, 5.0, 0.0, 6.0, 0.0, None, None, None, 3.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 12.0, 160, 100, 0, 30, 5.0, 0.0, 14.5, 0.0, None, None, None, 3.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 12.0, 190, 100, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 3.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 12.0, 190, 100, 0, 30, 5.0, 0.0, 6.0, 0.0, None, None, None, 3.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 12.0, 190, 100, 0, 30, 5.0, 0.0, 14.5, 0.0, None, None, None, 3.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 12.0, 210, 100, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 3.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 12.0, 210, 100, 0, 30, 5.0, 0.0, 6.0, 0.0, None, None, None, 3.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 12.0, 210, 100, 0, 30, 5.0, 0.0, 14.5, 0.0, None, None, None, 3.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 12.0, 38, 150, 0, 28, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 12.0, 38, 150, 0, 28, 5.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 12.0, 38, 150, 0, 28, 5.0, 0.0, 14.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Aerial Punisher Side Special ACMD
unsafe extern "C" fn ssbexo_cloud_aerial_punisher_side_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 3.0, 90, 90, 120, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 3.0, 90, 90, 120, 0, 5.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 3.0, 90, 90, 120, 0, 5.0, 0.0, 14.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 3.0, 75, 90, 90, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 3.0, 75, 90, 90, 0, 5.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 3.0, 75, 90, 90, 0, 5.0, 0.0, 14.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 3.0, 90, 90, 80, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 3.0, 90, 90, 80, 0, 5.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 3.0, 90, 90, 80, 0, 5.0, 0.0, 14.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 27.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 10.0, 90, 50, 0, 30, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 10.0, 90, 50, 0, 30, 7.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 10.0, 90, 50, 0, 30, 7.0, 0.0, 14.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 32.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Grounded Punisher Side Special Effect
unsafe extern "C" fn ssbexo_cloud_grounded_punisher_side_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.2);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1_blue"), Hash40::new("tex_cloud_sword2"), 16, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1_blue"), Hash40::new("tex_cloud_sword2"), 16, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 1.5, 0, -18, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 1.5, 0, -18, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, true, true);
    }
}

//Aerial Punisher Side Special Effect
unsafe extern "C" fn ssbexo_cloud_aerial_punisher_side_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.2);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1_blue"), Hash40::new("tex_cloud_sword2"), 16, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.2);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1_blue"), Hash40::new("tex_cloud_sword2"), 16, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.2);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1_blue"), Hash40::new("tex_cloud_sword2"), 16, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 62.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, true, true);
    }
}

//Grounded Punisher Side Special Sound
unsafe extern "C" fn ssbexo_cloud_grounded_punisher_side_special_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_special_s01"));
    }
}

//Aerial Punisher Side Special Sound
unsafe extern "C" fn ssbexo_cloud_aerial_punisher_side_special_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_special_s01"));
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_smash_s02"));
    }
    frame(agent.lua_state_agent, 27.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_smash_s03"));
    }
}

//Grounded Punisher Side Special Expression
unsafe extern "C" fn ssbexo_cloud_grounded_punisher_side_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Aerial Punisher Side Special Expression
unsafe extern "C" fn ssbexo_cloud_aerial_punisher_side_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 27.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 45.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specials", ssbexo_cloud_grounded_side_special_acmd, Low)
    .game_acmd("game_specialairs", ssbexo_cloud_aerial_side_special_acmd, Low)
    .effect_acmd("effect_specials", ssbexo_cloud_grounded_side_special_effect, Low)
    .effect_acmd("effect_specialairs", ssbexo_cloud_aerial_side_special_effect, Low)
    .sound_acmd("sound_specials", ssbexo_cloud_grounded_side_special_sound, Low)
    .sound_acmd("sound_specialairs", ssbexo_cloud_aerial_side_special_sound, Low)
    .expression_acmd("expression_specials", ssbexo_cloud_grounded_side_special_expression, Low)
    .expression_acmd("expression_specialairs", ssbexo_cloud_aerial_side_special_expression, Low)
    .game_acmd("game_punishspecials", ssbexo_cloud_grounded_punisher_side_special_acmd, Low)
    .game_acmd("game_punishspecialairs", ssbexo_cloud_aerial_punisher_side_special_acmd, Low)
    .effect_acmd("effect_punishspecials", ssbexo_cloud_grounded_punisher_side_special_effect, Low)
    .effect_acmd("effect_punishspecialairs", ssbexo_cloud_aerial_punisher_side_special_effect, Low)
    .sound_acmd("sound_punishspecials", ssbexo_cloud_grounded_punisher_side_special_sound, Low)
    .sound_acmd("sound_punishspecialairs", ssbexo_cloud_aerial_punisher_side_special_sound, Low)
    .expression_acmd("expression_punishspecials", ssbexo_cloud_grounded_punisher_side_special_expression, Low)
    .expression_acmd("expression_punishspecialairs", ssbexo_cloud_aerial_punisher_side_special_expression, Low)
    .install()
    ;
}