#![allow(unused_macros)]
use {
    crate::functions::variables::*,
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{
            lua_bind::*,
            sv_animcmd::*,
            *
        },
        lib::lua_const::*,
    },
    smash_script::*,
    smashline::*,
};

//Forward Smash ACMD
#[acmd_script( agent = "ness", script = "game_attacks4", category = ACMD_GAME)]
unsafe fn ssbuexo_ness_forward_smash_acmd(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, Hash40::new("bat").hash as i64, Hash40::new("bat_visible").hash as i64);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_NESS_STATUS_ATTACK_S4_FLAG_REFLECT_START);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        if OFFENSE_UP_ACTIVE[entry_id] == true {
            if FULL_SMASH_ATTACK[entry_id] == true {
                macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 17.0, 361, 67, 0, 70, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NESS_BAT, *ATTACK_REGION_BAT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 19.0, 361, 67, 0, 70, 2.6, 0.0, 3.85, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NESS_BAT, *ATTACK_REGION_BAT);
                macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 25.0, 361, 67, 0, 70, 3.0, 0.0, 7.7, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_BAT);
            }
            else {
                macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 17.0, 361, 67, 0, 70, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NESS_BAT, *ATTACK_REGION_BAT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 19.0, 361, 67, 0, 70, 2.6, 0.0, 3.85, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NESS_BAT, *ATTACK_REGION_BAT);
                macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 21.0, 361, 67, 0, 70, 3.0, 0.0, 7.7, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_BAT);
            }
        }
        else if FULL_SMASH_ATTACK[entry_id] == true
        && OFFENSE_UP_ACTIVE[entry_id] != true {
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 15.0, 361, 67, 0, 70, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NESS_BAT, *ATTACK_REGION_BAT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 17.0, 361, 67, 0, 70, 2.6, 0.0, 3.85, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NESS_BAT, *ATTACK_REGION_BAT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 25.0, 361, 67, 0, 70, 3.0, 0.0, 7.7, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_BAT);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 15.0, 361, 67, 0, 70, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NESS_BAT, *ATTACK_REGION_BAT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 17.0, 361, 67, 0, 70, 2.6, 0.0, 3.85, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NESS_BAT, *ATTACK_REGION_BAT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 19.0, 361, 67, 0, 70, 3.0, 0.0, 7.7, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NESS_BAT, *ATTACK_REGION_BAT);
        }
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        FULL_SMASH_ATTACK[entry_id] = false;
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_NESS_STATUS_ATTACK_S4_FLAG_REFLECT_END);
    }
    frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, Hash40::new("bat").hash as i64, Hash40::new("bat_invisible").hash as i64);
    }
}

//Forward Smash Effect
#[acmd_script( agent = "ness", script = "effect_attacks4", category = ACMD_EFFECT)]
unsafe fn ssbuexo_ness_forward_smash_effect(fighter: &mut L2CAgentBase) 
{
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0.0, 8, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        if FULL_SMASH_ATTACK[entry_id] == true {
            macros::EFFECT(fighter, Hash40::new("ness_psi_atk"), Hash40::new("haver"), 0, 0, 0, 0, -90, 0, 0.6, 0, 0, 0, 0, 0, 360, true);
            macros::EFFECT(fighter, Hash40::new("ness_psi_atk"), Hash40::new("haver"), 0, 0, 6, 0, -90, 0, 0.6, 0, 0, 0, 0, 0, 360, true);
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("ness_smash_arc"), Hash40::new("ness_smash_arc"), Hash40::new("top"), 2, 4, 2, 0, 0, 13, 1, true, *EF_FLIP_YZ);
        }
        else {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("ness_smash_arc"), Hash40::new("ness_smash_arc"), Hash40::new("top"), 2, 4, 2, 0, 0, 13, 1, true, *EF_FLIP_YZ);
        }
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        if OFFENSE_UP_ACTIVE[entry_id] == true {
            macros::EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -2, 6, 12, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 360, true, 0.6);
            macros::EFFECT(fighter, Hash40::new("ness_psi_atk"), Hash40::new("top"), 0, 6, 12, 0, -90, 0, 1.0, 0, 0, 0, 0, 0, 360, true);
        }
        else {
            macros::EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -2, 6, 12, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 360, true, 0.6);
        }
    }
}

//Up Smash Charge ACMD
#[acmd_script( agent = "ness_yoyohead", script = "game_attackhi4charge", category = ACMD_GAME)]
unsafe fn ssbuexo_ness_up_smash_charge_acmd(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        if OFFENSE_UP_ACTIVE[entry_id] == true {
            macros::ATTACK(fighter, 0, 0, Hash40::new("attach"), 1.0, 45, 100, 12, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
        }
        else {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
}

//Up Smash ACMD
#[acmd_script( agent = "ness_yoyohead", script = "game_attackhi4", category = ACMD_GAME)]
unsafe fn ssbuexo_ness_up_smash_acmd(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 1.0);
    MotionModule::set_rate(fighter.module_accessor, 1.351);
    frame(fighter.lua_state_agent, 11.0);
    MotionModule::set_rate(fighter.module_accessor, 1.0);
    if macros::is_excute(fighter) {
        if OFFENSE_UP_ACTIVE[entry_id] == true {
            macros::ATTACK(fighter, 0, 0, Hash40::new("attach"), 13.0, 80, 79, 0, 70, 4.7, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("attach"), 13.0, 80, 79, 0, 70, 4.7, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
    frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Down Smash Charge ACMD
#[acmd_script( agent = "ness_yoyohead", script = "game_attacklw4charge", category = ACMD_GAME)]
unsafe fn ssbuexo_ness_down_smash_charge_acmd(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        if OFFENSE_UP_ACTIVE[entry_id] == true {
            macros::ATTACK(fighter, 0, 0, Hash40::new("attach"), 1.0, 45, 100, 12, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
        }
        else {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
}

//Down Smash ACMD
#[acmd_script( agent = "ness_yoyohead", script = "game_attacklw4", category = ACMD_GAME)]
unsafe fn ssbuexo_ness_down_smash_acmd(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        if OFFENSE_UP_ACTIVE[entry_id] == true {
            macros::ATTACK(fighter, 0, 0, Hash40::new("attach"), 1.0, 367, 100, 12, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("attach"), 1.0, 367, 100, 12, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        if OFFENSE_UP_ACTIVE[entry_id] == true {
            macros::ATTACK(fighter, 0, 0, Hash40::new("attach"), 10.0, 28, 80, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("attach"), 10.0, 28, 80, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        if OFFENSE_UP_ACTIVE[entry_id] == true {
            macros::ATTACK(fighter, 0, 0, Hash40::new("attach"), 1.0, 35, 100, 20, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("attach"), 1.0, 35, 100, 20, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        if OFFENSE_UP_ACTIVE[entry_id] == true {
            macros::ATTACK(fighter, 0, 0, Hash40::new("attach"), 1.0, 90, 100, 30, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
            AttackModule::clear_all(fighter.module_accessor);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("attach"), 1.0, 90, 100, 30, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        if OFFENSE_UP_ACTIVE[entry_id] == true {
            macros::ATTACK(fighter, 0, 0, Hash40::new("attach"), 10.0, 33, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("attach"), 10.0, 33, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_ness_forward_smash_acmd,
        ssbuexo_ness_forward_smash_effect,
        ssbuexo_ness_up_smash_charge_acmd,
        ssbuexo_ness_up_smash_acmd,
        ssbuexo_ness_down_smash_charge_acmd,
        ssbuexo_ness_down_smash_acmd
    );
}