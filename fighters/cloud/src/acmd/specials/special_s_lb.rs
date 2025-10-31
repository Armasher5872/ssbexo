use super::*;

//Limit Break Cross Slash ACMD
unsafe extern "C" fn ssbexo_cloud_limit_break_cross_slash_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        display_final_window(true);
    }
    frame(agent.lua_state_agent, 10.0);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 80, 0, 4.025, 0.0, 9.0, 16.0, Some(0.0), Some(6.5), Some(16.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 367, 100, 80, 0, 8.05, 0.0, 19.0, 14.0, Some(0.0), Some(9.0), Some(16.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    else {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 80, 0, 4.025, 0.0, 9.0, 10.5, Some(0.0), Some(6.5), Some(10.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 367, 100, 80, 0, 8.05, 0.0, 19.0, 16.0, Some(0.0), Some(9.0), Some(14.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 16.0);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 80, 0, 8.395, 0.0, 13.5, 17.7, Some(0.0), Some(9.0), Some(15.7), 0.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    else {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 80, 0, 8.395, 0.0, 13.5, 15.7, Some(0.0), Some(9.0), Some(17.7), 0.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 280, 100, 14, 0, 10.12, 0.0, 13.5, 16.8, Some(0.0), Some(8.5), Some(16.8), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 70, 100, 21, 0, 10.12, 0.0, 8.5, 16.8, Some(0.0), Some(8.5), Some(16.8), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(agent.lua_state_agent, 28.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 34.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 95, 100, 26, 0, 8.05, 0.0, 9.0, 11.7, Some(0.0), Some(9.0), Some(11.7), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 110, 100, 26, 0, 9.2, 0.0, 9.0, 12.7, Some(0.0), Some(9.0), Some(18.7), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(agent.lua_state_agent, 36.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 48.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 74, 80, 0, 60, 10.35, 0.0, 18.0, 12.7, Some(0.0), Some(9.0), Some(12.7), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    else {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 280, 100, 14, 0, 10.12, 0.0, 13.5, 12.7, Some(0.0), Some(8.5), Some(12.7), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 70, 100, 21, 0, 10.12, 0.0, 8.5, 12.7, Some(0.0), Some(8.5), Some(12.7), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(agent.lua_state_agent, 28.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 34.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 95, 100, 26, 0, 8.05, 0.0, 9.0, 11.7, Some(0.0), Some(9.0), Some(11.7), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 110, 100, 26, 0, 9.2, 0.0, 9.0, 18.7, Some(0.0), Some(9.0), Some(12.7), 0.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(agent.lua_state_agent, 36.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 48.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 74, 80, 0, 60, 10.35, 0.0, 18.0, 16.8, Some(0.0), Some(9.0), Some(16.8), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        display_final_window(false);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 63.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x25813802b6));
    }
}

//Limit Break Cross Slash Effect
unsafe extern "C" fn ssbexo_cloud_limit_break_cross_slash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.2);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1_yellow"), Hash40::new("tex_cloud_sword2"), 4, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        EFFECT_OFF_KIND(agent, Hash40::new("cloud_limitbreak_aura"), false, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("cloud_kyogiri_stroke1_l"), Hash40::new("top"), 0, 10, 17, 0, 100, 0, 1, 0, 0, 0, 0, 0, 0, true);
            agent.clear_lua_stack();
            lua_args!(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_KYOU_EFFECT);
            LAST_EFFECT_SET_WORK_INT(agent.lua_state_agent);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("cloud_kyogiri_stroke1_r"), Hash40::new("top"), 0, 10, 17, 0, -100, 0, 1, 0, 0, 0, 0, 0, 0, true);
            agent.clear_lua_stack();
            lua_args!(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_KYOU_EFFECT);
            LAST_EFFECT_SET_WORK_INT(agent.lua_state_agent);
        }
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        EffectModule::end_kind(agent.module_accessor, Hash40::new("cloud_kyogiri_stroke1_r"), 5);
        EffectModule::end_kind(agent.module_accessor, Hash40::new("cloud_kyogiri_stroke1_l"), 5);
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, true, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.4);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1_yellow"), Hash40::new("tex_cloud_sword2"), 3, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("cloud_kyogiri_stroke2_l"), Hash40::new("top"), 0, 10, 17, 0, 100, 0, 1, 0, 0, 0, 0, 0, 0, true);
            agent.clear_lua_stack();
            lua_args!(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_KYOU_EFFECT);
            LAST_EFFECT_SET_WORK_INT(agent.lua_state_agent);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("cloud_kyogiri_stroke2_r"), Hash40::new("top"), 0, 10, 17, 0, -100, 0, 1, 0, 0, 0, 0, 0, 0, true);
            agent.clear_lua_stack();
            lua_args!(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_KYOU_EFFECT);
            LAST_EFFECT_SET_WORK_INT(agent.lua_state_agent);
        }
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        AFTER_IMAGE_OFF(agent, 2);
        EffectModule::end_kind(agent.module_accessor, Hash40::new("cloud_kyogiri_stroke2_r"), 5);
        EffectModule::end_kind(agent.module_accessor, Hash40::new("cloud_kyogiri_stroke2_l"), 5);
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, true, true);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.5);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1_yellow"), Hash40::new("tex_cloud_sword2"), 4, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("cloud_kyogiri_stroke3_l"), Hash40::new("top"), 0, 10, 17, 0, 100, 0, 1, 0, 0, 0, 0, 0, 0, true);
            agent.clear_lua_stack();
            lua_args!(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_KYOU_EFFECT);
            LAST_EFFECT_SET_WORK_INT(agent.lua_state_agent);
            EFFECT(agent, Hash40::new("cloud_kyogiri_stroke4_l"), Hash40::new("top"), 0, 10, 17, 0, 100, 0, 1, 0, 0, 0, 0, 0, 0, true);
            agent.clear_lua_stack();
            lua_args!(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_KYOU_EFFECT2);
            LAST_EFFECT_SET_WORK_INT(agent.lua_state_agent);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("cloud_kyogiri_stroke3_r"), Hash40::new("top"), 0, 10, 17, 0, -100, 0, 1, 0, 0, 0, 0, 0, 0, true);
            agent.clear_lua_stack();
            lua_args!(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_KYOU_EFFECT);
            LAST_EFFECT_SET_WORK_INT(agent.lua_state_agent);
            EFFECT(agent, Hash40::new("cloud_kyogiri_stroke4_r"), Hash40::new("top"), 0, 10, 17, 0, -100, 0, 1, 0, 0, 0, 0, 0, 0, true);
            agent.clear_lua_stack();
            lua_args!(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_KYOU_EFFECT2);
            LAST_EFFECT_SET_WORK_INT(agent.lua_state_agent);
        }
    }
    frame(agent.lua_state_agent, 34.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 38.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, true, true);
        EffectModule::end_kind(agent.module_accessor, Hash40::new("cloud_kyogiri_stroke3_r"), 5);
        EffectModule::end_kind(agent.module_accessor, Hash40::new("cloud_kyogiri_stroke3_l"), 5);
        EffectModule::end_kind(agent.module_accessor, Hash40::new("cloud_kyogiri_stroke4_r"), 5);
        EffectModule::end_kind(agent.module_accessor, Hash40::new("cloud_kyogiri_stroke4_l"), 5);
    }
    frame(agent.lua_state_agent, 45.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.35);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1_yellow"), Hash40::new("tex_cloud_sword2"), 7, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 47.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 48.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("cloud_kyogiri_vanish"), Hash40::new("top"), 0, 10, 17, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 51.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, true, true);
    }
}

//Limit Break Cross Slash Sound
unsafe extern "C" fn ssbexo_cloud_limit_break_cross_slash_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_special_lbstart"));
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_cloud_rnd_special_s01_lb"));
        PLAY_SE(agent, Hash40::new("se_cloud_special_s07"));
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_special_s08"));
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_special_s09"));
    }
    wait(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_special_s10"));
    }
    wait(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_cloud_rnd_special_s04_lb"));
        PLAY_SE(agent, Hash40::new("se_cloud_special_s06"));
    }
}

//Limit Break Cross Slash Expression
unsafe extern "C" fn ssbexo_cloud_limit_break_cross_slash_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 4);
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("top"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 5);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 5);
    }
    frame(agent.lua_state_agent, 37.0);
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("top"), AttackDirectionAxis(*ATTACK_DIRECTION_X_MINUS), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 46.0);
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 49.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashll"), 0);
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialslb", ssbexo_cloud_limit_break_cross_slash_acmd, Low)
    .game_acmd("game_specialairslb", ssbexo_cloud_limit_break_cross_slash_acmd, Low)
    .effect_acmd("effect_specialslb", ssbexo_cloud_limit_break_cross_slash_effect, Low)
    .effect_acmd("effect_specialairslb", ssbexo_cloud_limit_break_cross_slash_effect, Low)
    .sound_acmd("sound_specialslb", ssbexo_cloud_limit_break_cross_slash_sound, Low)
    .sound_acmd("sound_specialairslb", ssbexo_cloud_limit_break_cross_slash_sound, Low)
    .expression_acmd("expression_specialslb", ssbexo_cloud_limit_break_cross_slash_expression, Low)
    .expression_acmd("expression_specialairslb", ssbexo_cloud_limit_break_cross_slash_expression, Low)
    .install()
    ;
}