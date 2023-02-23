#![allow(unused_macros)]
use {
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

//Forward Tilt ACMD
#[acmd_script( agent = "mewtwo", scripts = ["game_attacks3", "game_attacks3hi", "game_attacks3lw"], category = ACMD_GAME)]
unsafe fn ssbuexo_mewtwo_forward_tilt_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("s_tail2"), 12.0, 361, 75, 0, 70, 5.0, 1.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 1, 0, Hash40::new("s_tail4"), 11.0, 361, 75, 0, 70, 4.6, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 2, 0, Hash40::new("s_tail6"), 10.0, 361, 75, 0, 70, 4.2, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Forward Tilt Effect
#[acmd_script( agent = "mewtwo", script = "effect_attacks3", category = ACMD_EFFECT)]
unsafe fn ssbuexo_mewtwo_forward_tilt_effect(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail7"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_aura"), Hash40::new("s_tail7"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail5"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_aura"), Hash40::new("s_tail5"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail3"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_aura"), Hash40::new("s_tail3"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail1"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_aura"), Hash40::new("s_tail1"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new_raw(0x17e344269c), Hash40::new_raw(0x10444db313), 10, Hash40::new("s_tail1"), 0.0, 0.0, 0.0, Hash40::new("s_tail7"), 1.5, 1.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 3, 7, 10, 0, -85, -15, 1.1, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_deathscythe_trace"), false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_deathscythe_aura"), false, false);
        macros::AFTER_IMAGE_OFF(fighter, 4);
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
}

//Forward Tilt Hi Effect
#[acmd_script( agent = "mewtwo", script = "effect_attacks3hi", category = ACMD_EFFECT)]
unsafe fn ssbuexo_mewtwo_forward_tilt_hi_effect(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail7"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_aura"), Hash40::new("s_tail7"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail5"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_aura"), Hash40::new("s_tail5"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail3"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_aura"), Hash40::new("s_tail3"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail1"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_aura"), Hash40::new("s_tail1"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new_raw(0x17e344269c), Hash40::new_raw(0x10444db313), 10, Hash40::new("s_tail1"), 0.0, 0.0, 0.0, Hash40::new("s_tail7"), 1.5, 1.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 3, 10, 10, 0, -85, 35, 1.1, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_deathscythe_trace"), false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_deathscythe_aura"), false, false);
        macros::AFTER_IMAGE_OFF(fighter, 4);
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
}

//Forward Tilt Lw Effect
#[acmd_script( agent = "mewtwo", script = "effect_attacks3lw", category = ACMD_EFFECT)]
unsafe fn ssbuexo_mewtwo_forward_tilt_lw_effect(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail7"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_aura"), Hash40::new("s_tail7"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail5"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_aura"), Hash40::new("s_tail5"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail3"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_aura"), Hash40::new("s_tail3"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail1"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_aura"), Hash40::new("s_tail1"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new_raw(0x17e344269c), Hash40::new_raw(0x10444db313), 10, Hash40::new("s_tail1"), 0.0, 0.0, 0.0, Hash40::new("s_tail7"), 1.5, 1.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 3, 3.5, 10, 0, -85, -13, 1.1, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_deathscythe_trace"), false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_deathscythe_aura"), false, false);
        macros::AFTER_IMAGE_OFF(fighter, 4);
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
}

//Up Tilt ACMD
#[acmd_script( agent = "mewtwo", script = "game_attackhi3", category = ACMD_GAME)]
unsafe fn ssbuexo_mewtwo_up_tilt_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 95, 45, 0, 35, 6.0, 0.0, 2.0, -12.0, Some(0.0), Some(2.0), Some(12.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 95, 45, 0, 35, 6.0, 0.0, 11.7, -6.0, Some(0.0), Some(11.7), Some(7.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 95, 45, 0, 35, 6.0, 0.0, 14.7, -3.0, Some(0.0), Some(14.7), Some(3.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Up Tilt Effect
#[acmd_script( agent = "mewtwo", script = "effect_attackhi3", category = ACMD_EFFECT)]
unsafe fn ssbuexo_mewtwo_up_tilt_effect(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_appeal_aura"), Hash40::new("top"), 0, -1.5, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_dead"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 0.8, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 1.0);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 1.8, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 20, 0, 15, 0, 0, 0, false);
    }
    wait(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_shield_smoke"), false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_hit_death"), false, false);
    }
}

//Down Tilt ACMD
#[acmd_script( agent = "mewtwo", script = "game_attacklw3", category = ACMD_GAME)]
unsafe fn ssbuexo_mewtwo_down_tilt_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 80, 82, 0, 60, 4.3, 0.0, 3.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 70, 84, 0, 65, 3.8, 0.0, 1.5, 13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 2, 0, Hash40::new("s_tail7"), 4.0, 60, 70, 0, 60, 3.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        AttackModule::set_attack_height_all(fighter.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("s_tail3"), 5.0, 80, 82, 0, 60, 4.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 1, 0, Hash40::new("s_tail5"), 4.5, 70, 84, 0, 65, 3.8, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 2, 0, Hash40::new("s_tail7"), 4.0, 60, 70, 0, 60, 3.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        AttackModule::set_attack_height_all(fighter.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Down Tilt Effect
#[acmd_script( agent = "mewtwo", script = "effect_attacklw3", category = ACMD_EFFECT)]
unsafe fn ssbuexo_mewtwo_down_tilt_effect(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail7"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_aura"), Hash40::new("s_tail7"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail5"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_aura"), Hash40::new("s_tail5"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail3"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_aura"), Hash40::new("s_tail3"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail1"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_aura"), Hash40::new("s_tail1"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new_raw(0x17e344269c), Hash40::new_raw(0x10444db313), 10, Hash40::new("s_tail1"), 0.0, 0.0, 0.0, Hash40::new("s_tail7"), 1.5, 1.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), -5.5, 5, 4.5, 0, -70, 190, 1.05, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_deathscythe_trace"), false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_deathscythe_aura"), false, false);
        macros::AFTER_IMAGE_OFF(fighter, 4);
        macros::LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_mewtwo_forward_tilt_acmd,
        ssbuexo_mewtwo_forward_tilt_effect,
        ssbuexo_mewtwo_forward_tilt_hi_effect,
        ssbuexo_mewtwo_forward_tilt_lw_effect,
        ssbuexo_mewtwo_up_tilt_acmd,
        ssbuexo_mewtwo_up_tilt_effect,
        ssbuexo_mewtwo_down_tilt_acmd,
        ssbuexo_mewtwo_down_tilt_effect
    );
}