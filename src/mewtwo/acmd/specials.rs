use super::*;

//Shadowball Charge ACMD
unsafe extern "C" fn ssbexo_mewtwo_shadowball_charge_acmd(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 95, 93, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.78, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 2.0);
    }
}

//Confusion/Future Sight ACMD
unsafe extern "C" fn ssbexo_mewtwo_side_special_acmd(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if STORED_POWER_ENABLED[entry_id] == 1 {
        frame(agent.lua_state_agent, 16.0);
        if macros::is_excute(agent) {
            if HAS_FUTURESIGHT[entry_id] == false || FUTURESIGHT_CURRENT_FRAME[entry_id] < FUTURESIGHT_FUSE_TIME {
                FUTURESIGHT_CURRENT_FRAME[entry_id] = 0;
                FUTURESIGHT_X[entry_id] = PostureModule::pos_x(agent.module_accessor) + (6.0 * PostureModule::lr(agent.module_accessor));
                FUTURESIGHT_Y[entry_id] = PostureModule::pos_y(agent.module_accessor) + 16.0;
                HAS_FUTURESIGHT[entry_id] = true;
            }
        }
    }
    else {
        frame(agent.lua_state_agent, 1.0);
        macros::FT_MOTION_RATE(agent, 0.667);
        frame(agent.lua_state_agent, 6.0);
        macros::FT_MOTION_RATE(agent, 1);
        frame(agent.lua_state_agent, 12.0);
        if macros::is_excute(agent) {
            macros::CATCH(agent, 0, Hash40::new("top"), 7.4, 0.0, 8.4, 17.0, None, None, None, *FIGHTER_STATUS_KIND_MEWTWO_THROWN, *COLLISION_SITUATION_MASK_GA);
            macros::CATCH(agent, 1, Hash40::new("top"), 10.2, 0.0, 8.4, 17.0, None, None, None, *FIGHTER_STATUS_KIND_MEWTWO_THROWN, *COLLISION_SITUATION_MASK_G);
            shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MEWTWO_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
            macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 1.0, 280, 16, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_POS, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_POS, 0.0, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW_MEWTWO, 0, 1.15, 280, 16, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_POS, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            AttackModule::set_catch_only(agent.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW_MEWTWO, true, true);
        }
        wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        }
        frame(agent.lua_state_agent, 20.0);
        for _ in 0..7 {
            wait(agent.lua_state_agent, 2.0);
            if macros::is_excute(agent) {
                macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW_MEWTWO, Hash40::new("throw"), WorkModule::get_int64(agent.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_S_WORK_INT_TARGET_OBJECT_ID), WorkModule::get_int64(agent.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_S_WORK_INT_THROWN_HIT_GROUP), WorkModule::get_int64(agent.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_S_WORK_INT_THROWN_HIT_NO));
            }
        }
        frame(agent.lua_state_agent, 36.0);
        if macros::is_excute(agent) {
            shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MEWTWO_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_S_FLAG_GRAVITY_NORMAL);
        }
        frame(agent.lua_state_agent, 40.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_S_FLAG_HIT);
        }
    }
}

//Confusion/Future Sight Effect
unsafe extern "C" fn ssbexo_mewtwo_side_special_effect(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if STORED_POWER_ENABLED[entry_id] == 1 {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), -1.5, 0, 2, 0, 0, 0, 0.4, true, *EF_FLIP_YZ);
        }
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_pk_hand"), false, false);
        }
    }
    else {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), -1.5, 0, 2, 0, 0, 0, 0.4, true, *EF_FLIP_YZ);
        }
        frame(agent.lua_state_agent, 12.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("mewtwo_nenriki"), Hash40::new("top"), 0, 9, 17, 0, 90, 0, 0.45, 0, 0, 0, 0, 0, 360, true);
        }
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_pk_hand"), false, false);
        }
    }
}

//Grounded Disable ACMD
unsafe extern "C" fn ssbexo_mewtwo_grounded_down_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 361, 140, 0, 0, 3.0, 0.0, 14.0, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 361, 180, 0, 20, 3.0, 0.0, -1.7, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_lr_check_front_lr(agent.module_accessor, 0);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 361, 140, 0, 0, 3.0, 0.0, 14.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 361, 180, 0, 20, 3.0, 0.0, -1.7, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_lr_check_front_lr(agent.module_accessor, 0);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 361, 140, 0, 0, 3.0, 0.0, 14.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 361, 180, 0, 20, 3.0, 0.0, -1.7, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_lr_check_front_lr(agent.module_accessor, 0);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 361, 140, 0, 0, 3.0, 0.0, 14.0, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 361, 180, 0, 20, 3.0, 0.0, -1.7, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_lr_check_front_lr(agent.module_accessor, 0);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 361, 140, 0, 0, 3.0, 0.0, 14.0, 14.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 361, 180, 0, 20, 3.0, 0.0, -1.7, 14.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_lr_check_front_lr(agent.module_accessor, 0);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 361, 140, 0, 0, 3.0, 0.0, 14.0, 17.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 361, 180, 0, 20, 3.0, 0.0, -1.7, 17.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_lr_check_front_lr(agent.module_accessor, 0);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 361, 140, 0, 0, 3.0, 0.0, 14.0, 20.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 361, 180, 0, 20, 3.0, 0.0, -1.7, 20.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_lr_check_front_lr(agent.module_accessor, 0);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 361, 140, 0, 0, 3.0, 0.0, 14.0, 23.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 361, 180, 0, 20, 3.0, 0.0, -1.7, 23.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_lr_check_front_lr(agent.module_accessor, 0);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Grounded Disable Effect
unsafe extern "C" fn ssbexo_mewtwo_grounded_down_special_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FLASH(agent, 1, 1, 1, 0.55);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 0.35, 0.5, 1, 0.4);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH_FRM(agent, 5, 0, 0, 0.35, 0);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 1, 1, 1, 0.55);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 0.35, 0.5, 1, 0.4);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH_FRM(agent, 5, 0, 0, 0.35, 0);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 1, 1, 1, 0.55);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 0.35, 0.5, 1, 0.4);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH_FRM(agent, 5, 0, 0, 0.35, 0);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 10.0);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) == 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_kanasibari_eye"), Hash40::new("head"), 0.7, 1.9, 1.4, 0, 0, 0, 0.5, true);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_kanasibari_eye"), Hash40::new("head"), 0.7, 1.9, -1.4, 0, 0, 0, 0.5, true);
        }
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_kanasibari"), false, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_kanasibari"), Hash40::new("top"), 0.7, 13.9, 1.4, 0, 0, 0, 1.0, true);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_kanasibari"), false, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_kanasibari"), Hash40::new("top"), 0.7, 13.9, 4.4, 0, 0, 0, 1.0, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_kanasibari"), false, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_kanasibari"), Hash40::new("top"), 0.7, 13.9, 7.4, 0, 0, 0, 1.0, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_kanasibari"), false, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_kanasibari"), Hash40::new("top"), 0.7, 13.9, 10.4, 0, 0, 0, 1.0, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_kanasibari"), false, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_kanasibari"), Hash40::new("top"), 0.7, 13.9, 13.4, 0, 0, 0, 1.0, true);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_kanasibari"), false, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_kanasibari"), Hash40::new("top"), 0.7, 13.9, 16.4, 0, 0, 0, 1.0, true);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_kanasibari"), false, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_kanasibari"), Hash40::new("top"), 0.7, 13.9, 19.4, 0, 0, 0, 1.0, true);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_kanasibari"), false, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_kanasibari"), Hash40::new("top"), 0.7, 13.9, 22.4, 0, 0, 0, 1.0, true);
    }
}

//Aerial Disable Projectile ACMD
unsafe extern "C" fn ssbexo_mewtwo_aerial_disable_projectile_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 361, 280, 0, 20, 3.0, 0.0, -1.7, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_lr_check_front_lr(agent.module_accessor, 0);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
    }
}

pub fn install() {
    Agent::new("mewtwo")
    .game_acmd("game_specials", ssbexo_mewtwo_side_special_acmd)
    .effect_acmd("effect_specials", ssbexo_mewtwo_side_special_effect)
    .game_acmd("game_speciallw", ssbexo_mewtwo_grounded_down_special_acmd)
    .effect_acmd("effect_speciallw", ssbexo_mewtwo_grounded_down_special_effect)
    .install()
    ;
    Agent::new("mewtwo_shadowball")
    .game_acmd("game_charge", ssbexo_mewtwo_shadowball_charge_acmd)
    .game_acmd("game_chargemax", ssbexo_mewtwo_shadowball_charge_acmd)
    .install()
    ;
    Agent::new("mewtwo_bindball")
    .game_acmd("game_shoot", ssbexo_mewtwo_aerial_disable_projectile_acmd)
    .install()
    ;
}