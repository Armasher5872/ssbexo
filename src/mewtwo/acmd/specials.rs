#![allow(unused_macros)]
use {
    crate::functions::{
        FUTURESIGHT_CURRENT_FRAME,
        FUTURESIGHT_FUSE_TIME,
        FUTURESIGHT_X,
        FUTURESIGHT_Y,
        HAS_FUTURESIGHT,
        STORED_POWER_ENABLED
    },
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::*,
        },
        lib::lua_const::*,
        lua2cpp::L2CAgentBase,
        phx::Hash40,
    },
    smashline::*,
    smash_script::*,
};

//Shadowball Charge ACMD
#[acmd_script( agent = "mewtwo_shadowball", scripts = ["game_charge", "game_chargemax"], category = ACMD_GAME)]
unsafe fn ssbuexo_mewtwo_shadowball_charge_acmd(fighter: &mut L2CAgentBase) 
{
    for _ in 0..i32::MAX {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 95, 93, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.78, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        wait(fighter.lua_state_agent, 2.0);
    }
}

//Confusion/Future Sight ACMD
#[acmd_script( agent = "mewtwo", script = "game_specials", category = ACMD_GAME)]
unsafe fn ssbuexo_mewtwo_side_special_acmd(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if STORED_POWER_ENABLED[entry_id] == 1 {
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            if HAS_FUTURESIGHT[entry_id] == false || FUTURESIGHT_CURRENT_FRAME[entry_id] < FUTURESIGHT_FUSE_TIME {
                FUTURESIGHT_CURRENT_FRAME[entry_id] = 0;
                FUTURESIGHT_X[entry_id] = PostureModule::pos_x(fighter.module_accessor) + (6.0 * PostureModule::lr(fighter.module_accessor));
                FUTURESIGHT_Y[entry_id] = PostureModule::pos_y(fighter.module_accessor) + 16.0;
                HAS_FUTURESIGHT[entry_id] = true;
            }
        }
    }
    else {
        frame(fighter.lua_state_agent, 1.0);
        macros::FT_MOTION_RATE(fighter, 0.667);
        frame(fighter.lua_state_agent, 6.0);
        macros::FT_MOTION_RATE(fighter, 1);
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            macros::CATCH(fighter, 0, Hash40::new("top"), 7.4, 0.0, 8.4, 17.0, None, None, None, *FIGHTER_STATUS_KIND_MEWTWO_THROWN, *COLLISION_SITUATION_MASK_GA);
            macros::CATCH(fighter, 1, Hash40::new("top"), 10.2, 0.0, 8.4, 17.0, None, None, None, *FIGHTER_STATUS_KIND_MEWTWO_THROWN, *COLLISION_SITUATION_MASK_G);
            shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MEWTWO_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 1.0, 280, 16, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_POS, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_POS, 0.0, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW_MEWTWO, 0, 1.15, 280, 16, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_POS, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            AttackModule::set_catch_only(fighter.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW_MEWTWO, true, true);
        }
        wait(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        }
        frame(fighter.lua_state_agent, 20.0);
        for _ in 0..7 {
            wait(fighter.lua_state_agent, 2.0);
            if macros::is_excute(fighter) {
                macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW_MEWTWO, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_S_WORK_INT_TARGET_OBJECT_ID), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_S_WORK_INT_THROWN_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_S_WORK_INT_THROWN_HIT_NO));
            }
        }
        frame(fighter.lua_state_agent, 36.0);
        if macros::is_excute(fighter) {
            shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MEWTWO_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_S_FLAG_GRAVITY_NORMAL);
        }
        frame(fighter.lua_state_agent, 40.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_S_FLAG_HIT);
        }
    }
}

//Confusion/Future Sight Effect
#[acmd_script( agent = "mewtwo", script = "effect_specials", category = ACMD_EFFECT)]
unsafe fn ssbuexo_mewtwo_side_special_effect(fighter: &mut L2CAgentBase) 
{
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if STORED_POWER_ENABLED[entry_id] == 1 {
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), -1.5, 0, 2, 0, 0, 0, 0.4, true, *EF_FLIP_YZ);
        }
        frame(fighter.lua_state_agent, 14.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 20.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_pk_hand"), false, false);
        }
    }
    else {
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), -1.5, 0, 2, 0, 0, 0, 0.4, true, *EF_FLIP_YZ);
        }
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("mewtwo_nenriki"), Hash40::new("top"), 0, 9, 17, 0, 90, 0, 0.45, 0, 0, 0, 0, 0, 360, true);
        }
        frame(fighter.lua_state_agent, 14.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 20.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_pk_hand"), false, false);
        }
    }
}

//Grounded Disable ACMD
#[acmd_script( agent = "mewtwo", script = "game_speciallw", category = ACMD_GAME)]
unsafe fn ssbuexo_mewtwo_grounded_disable_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 140, 0, 0, 3.0, 0.0, 14.0, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 361, 180, 0, 20, 3.0, 0.0, -1.7, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_lr_check_front_lr(fighter.module_accessor, 0);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 140, 0, 0, 3.0, 0.0, 14.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 361, 180, 0, 20, 3.0, 0.0, -1.7, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_lr_check_front_lr(fighter.module_accessor, 0);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 140, 0, 0, 3.0, 0.0, 14.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 361, 180, 0, 20, 3.0, 0.0, -1.7, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_lr_check_front_lr(fighter.module_accessor, 0);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 140, 0, 0, 3.0, 0.0, 14.0, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 361, 180, 0, 20, 3.0, 0.0, -1.7, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_lr_check_front_lr(fighter.module_accessor, 0);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 140, 0, 0, 3.0, 0.0, 14.0, 14.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 361, 180, 0, 20, 3.0, 0.0, -1.7, 14.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_lr_check_front_lr(fighter.module_accessor, 0);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 140, 0, 0, 3.0, 0.0, 14.0, 17.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 361, 180, 0, 20, 3.0, 0.0, -1.7, 17.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_lr_check_front_lr(fighter.module_accessor, 0);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 140, 0, 0, 3.0, 0.0, 14.0, 20.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 361, 180, 0, 20, 3.0, 0.0, -1.7, 20.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_lr_check_front_lr(fighter.module_accessor, 0);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 140, 0, 0, 3.0, 0.0, 14.0, 23.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 361, 180, 0, 20, 3.0, 0.0, -1.7, 23.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_lr_check_front_lr(fighter.module_accessor, 0);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Grounded Disable Effect
#[acmd_script( agent = "mewtwo", script = "effect_speciallw", category = ACMD_EFFECT)]
unsafe fn ssbuexo_mewtwo_grounded_disable_effect(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        macros::FLASH(fighter, 1, 1, 1, 0.55);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FLASH(fighter, 0.35, 0.5, 1, 0.4);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FLASH_FRM(fighter, 5, 0, 0, 0.35, 0);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FLASH(fighter, 1, 1, 1, 0.55);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FLASH(fighter, 0.35, 0.5, 1, 0.4);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FLASH_FRM(fighter, 5, 0, 0, 0.35, 0);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FLASH(fighter, 1, 1, 1, 0.55);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FLASH(fighter, 0.35, 0.5, 1, 0.4);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FLASH_FRM(fighter, 5, 0, 0, 0.35, 0);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 10.0);
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) == 0.0 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_kanasibari_eye"), Hash40::new("head"), 0.7, 1.9, 1.4, 0, 0, 0, 0.5, true);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_kanasibari_eye"), Hash40::new("head"), 0.7, 1.9, -1.4, 0, 0, 0, 0.5, true);
        }
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_kanasibari"), false, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_kanasibari"), Hash40::new("top"), 0.7, 13.9, 1.4, 0, 0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_kanasibari"), false, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_kanasibari"), Hash40::new("top"), 0.7, 13.9, 4.4, 0, 0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_kanasibari"), false, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_kanasibari"), Hash40::new("top"), 0.7, 13.9, 7.4, 0, 0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_kanasibari"), false, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_kanasibari"), Hash40::new("top"), 0.7, 13.9, 10.4, 0, 0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_kanasibari"), false, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_kanasibari"), Hash40::new("top"), 0.7, 13.9, 13.4, 0, 0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_kanasibari"), false, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_kanasibari"), Hash40::new("top"), 0.7, 13.9, 16.4, 0, 0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_kanasibari"), false, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_kanasibari"), Hash40::new("top"), 0.7, 13.9, 19.4, 0, 0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_kanasibari"), false, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_kanasibari"), Hash40::new("top"), 0.7, 13.9, 22.4, 0, 0, 0, 1.0, true);
    }
}

//Aerial Disable Projectile ACMD
#[acmd_script( agent = "mewtwo_bindball", script = "game_shoot", category = ACMD_GAME)]
unsafe fn ssbuexo_mewtwo_aerial_disable_projectile_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 280, 0, 20, 3.0, 0.0, -1.7, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_lr_check_front_lr(fighter.module_accessor, 0);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_mewtwo_shadowball_charge_acmd,
        ssbuexo_mewtwo_side_special_acmd,
        ssbuexo_mewtwo_side_special_effect,
        ssbuexo_mewtwo_grounded_disable_acmd,
        ssbuexo_mewtwo_grounded_disable_effect,
        ssbuexo_mewtwo_aerial_disable_projectile_acmd
    );
}