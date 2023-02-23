#![allow(unused_macros)]
use {
    crate::functions::{
        FULL_SMASH_ATTACK,
        SPECIAL_ZOOM_GFX
    },
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::*,
        },
        lua2cpp::L2CAgentBase,
        lib::lua_const::*,
        phx::Hash40,
    },
    smashline::*,
    smash_script::*,
};

//Smash Attack Charge
#[acmd_script( agent = "ganon", scripts = ["game_attacks4charge", "game_attackhi4charge", "game_attacklw4charge"], category = ACMD_GAME)]
unsafe fn ssbuexo_armstrong_smash_attack_charge(fighter: &mut L2CAgentBase) {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 59.0);
    if macros::is_excute(fighter) {
        FULL_SMASH_ATTACK[entry_id] = true;
    }
}

//Forward Smash ACMD
#[acmd_script( agent = "ganon", script = "game_attacks4", category = ACMD_GAME)]
unsafe fn ssbuexo_armstrong_forward_smash_acmd(fighter: &mut L2CAgentBase) 
{
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 9.5);
        DamageModule::set_reaction_mul(fighter.module_accessor, 0.65);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
        }
        macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 24.0, 40, 60, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 24.0, 40, 60, 0, 60, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 24.0, 40, 60, 0, 60, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
        FULL_SMASH_ATTACK[entry_id] = false;
        SPECIAL_ZOOM_GFX[entry_id] = 0;
    }
}

//Forward Smash Effect
#[acmd_script( agent = "ganon", script = "effect_attacks4", category = ACMD_EFFECT)]
unsafe fn ssbuexo_armstrong_forward_smash_effect(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("ganon_majinken_start"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("ganon_majinken_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_shield_smoke"), false, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("ganon_majinken_hold"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("ganon_majinken"), Hash40::new("top"), 0, 12.5, 22.5, 0, -10, 0, 0.8, true);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
}

//Forward Smash Sound
#[acmd_script( agent = "ganon", script = "sound_attacks4", category = ACMD_SOUND)]
unsafe fn ssbuexo_armstrong_forward_smash_sound(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
        macros::PLAY_SE(fighter, Hash40::new("vc_ganon_attack08"));
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ganon_special_n02"));
    }
}

//Up Smash ACMD
#[acmd_script( agent = "ganon", script = "game_attackhi4", category = ACMD_GAME)]
unsafe fn ssbuexo_armstrong_up_smash_acmd(fighter: &mut L2CAgentBase) 
{
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 8.5);
        DamageModule::set_reaction_mul(fighter.module_accessor, 0.65);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
        }
        macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 22.0, 80, 70, 0, 40, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 22.0, 80, 70, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 22.0, 80, 70, 0, 40, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
        FULL_SMASH_ATTACK[entry_id] = false;
        SPECIAL_ZOOM_GFX[entry_id] = 0;
    }
}

//Up Smash Effect
#[acmd_script( agent = "ganon", script = "effect_attackhi4", category = ACMD_EFFECT)]
unsafe fn ssbuexo_armstrong_up_smash_effect(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 21.0);
	if macros::is_excute(fighter) {
		macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
		macros::LAST_EFFECT_SET_RATE(fighter, 0.4);
        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x10e97de698), Hash40::new("top"), 0, 15, -3, 180, 190, -90, 1.15, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.88, 0.35, 0.13);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new_raw(0x1156ac182a), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1.5, true, 0.9);
        macros::LAST_EFFECT_SET_RATE(fighter, 2.0);
	}
}

//Down Smash ACMD
#[acmd_script( agent = "ganon", script = "game_attacklw4", category = ACMD_GAME)]
unsafe fn ssbuexo_armstrong_down_smash_acmd(fighter: &mut L2CAgentBase) 
{
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 8.0);
        DamageModule::set_reaction_mul(fighter.module_accessor, 0.65);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
        }
        macros::ATTACK(fighter, 3, 0, Hash40::new("handr"), 20.0, 300, 50, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("handr"), 20.0, 300, 50, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 19.0, 361, 50, 0, 50, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.45, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 18.0, 361, 50, 0, 50, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_level(fighter.module_accessor, 3, *FIGHTER_RYU_SAVING_LV_3 as u8);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
        AttackModule::clear_all(fighter.module_accessor);
        FULL_SMASH_ATTACK[entry_id] = false;
        SPECIAL_ZOOM_GFX[entry_id] = 0;
    }
}

//Down Smash Effect
#[acmd_script( agent = "ganon", script = "effect_attacklw4", category = ACMD_EFFECT)]
unsafe fn ssbuexo_armstrong_down_smash_effect(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 10, 3, 0, -80, -90, 1.4, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.88, 0.35, 0.13);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 10, 0, 0, -40, -90, 1.2, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.88, 0.35, 0.13);
    }
}

//Down Smash Sound
#[acmd_script( agent = "ganon", script = "sound_attacklw4", category = ACMD_SOUND)]
unsafe fn ssbuexo_armstrong_down_smash_sound(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_ganon_attack07"));
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_armstrong_smash_attack_charge,
        ssbuexo_armstrong_forward_smash_acmd,
        ssbuexo_armstrong_forward_smash_effect,
        ssbuexo_armstrong_forward_smash_sound,
        ssbuexo_armstrong_up_smash_acmd,
        ssbuexo_armstrong_up_smash_effect,
        ssbuexo_armstrong_down_smash_acmd,
        ssbuexo_armstrong_down_smash_effect,
        ssbuexo_armstrong_down_smash_sound
    );
}