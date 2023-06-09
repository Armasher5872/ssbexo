use super::*;

//Nair ACMD
#[acmd_script( agent = "mewtwo", script = "game_attackairn", category = ACMD_GAME)]
unsafe fn ssbuexo_mewtwo_nair_acmd(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 6.0);
    for _ in 0..6 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("waist"), 1.6, 367, 60, 80, 0, 11.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, -10.0, false);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        wait(fighter.lua_state_agent, 2.0);
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 110, 0, 40, 13.5, 0.0, 7.0, -2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Nair Effect
#[acmd_script( agent = "mewtwo", script = "effect_attackairn", category = ACMD_EFFECT)]
unsafe fn ssbuexo_mewtwo_nair_effect(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_pk_attack_f"), Hash40::new("top"), 0, 9, -0.5, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.25, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_aura"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.25, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.25, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.25, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1.25, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_aura"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1.25, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1.25, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_aura"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1.25, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("top"), 0, 9, -0.5, 0, 0, 0, 2, true);
    }
    for _ in 0..10 {
        if macros::is_excute(fighter) {
            macros::BURN_COLOR(fighter, 0.7, 0.2, 1, 0.7);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::BURN_COLOR_FRAME(fighter, 1, 0.7, 0.2, 1, 0);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::BURN_COLOR_NORMAL(fighter);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::FLASH(fighter, 1, 0.7, 1, 0.5);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::FLASH_FRM(fighter, 1, 1, 0.7, 1, 0);
            macros::COL_NORMAL(fighter);
        }
    }
    frame(fighter.lua_state_agent, 47.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_deathscythe_trace"), false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_deathscythe_aura"), false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_shadowball_hold"), false, false);
    }
}

//Fair ACMD
#[acmd_script( agent = "mewtwo", script = "game_attackairf", category = ACMD_GAME)]
unsafe fn ssbuexo_mewtwo_fair_acmd(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 47, 100, 0, 40, 5.5, 0.0, 8.2, 13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 47, 100, 0, 40, 4.5, 0.0, 8.2, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Fair Effect
#[acmd_script( agent = "mewtwo", script = "effect_attackairf", category = ACMD_EFFECT)]
unsafe fn ssbuexo_mewtwo_fair_effect(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.8, true);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("haver"), 1.5, 0, 2.5, 0, 0, 0, 0.55, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_attack_g"), Hash40::new("mewtwo_pk_attack_g"), Hash40::new("top"), 0, 7, 5, 0, 0, 40, 1.3, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_pk_hand"), false, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_deathscythe_trace"), false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_deathscythe_aura"), false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_shadowball_hold"), false, false);
    }
}

//Bair ACMD
#[acmd_script( agent = "mewtwo", script = "game_attackairb", category = ACMD_GAME)]
unsafe fn ssbuexo_mewtwo_bair_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 30, 70, 0, 45, 7.0, 0.0, 10.0, -12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Bair Effect
#[acmd_script( agent = "mewtwo", script = "effect_attackairb", category = ACMD_EFFECT)]
unsafe fn ssbuexo_mewtwo_bair_effect(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_dead_dark"), Hash40::new("sys_dead_dark"), Hash40::new("top"), 0, 8, -12.0, 0, -90, 0, 0.15, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_deathscythe_trace"), false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_deathscythe_aura"), false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_shadowball_hold"), false, false);
    }
}

//Uair ACMD
#[acmd_script( agent = "mewtwo", script = "game_attackairhi", category = ACMD_GAME)]
unsafe fn ssbuexo_mewtwo_uair_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("s_tail3"), 10.0, 72, 98, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 1, 0, Hash40::new("s_tail5"), 9.0, 65, 92, 0, 40, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 2, 0, Hash40::new("s_tail7"), 8.0, 55, 92, 0, 40, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
    }
    wait(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Uair Effect
#[acmd_script( agent = "mewtwo", script = "effect_attackairhi", category = ACMD_EFFECT)]
unsafe fn ssbuexo_mewtwo_uair_effect(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_2"), 5, Hash40::new("s_tail6"), 0.0, 0.0, 0.0, Hash40::new("s_tail7"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail7"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 5, Hash40::new("s_tail5"), 0.0, 0.0, 0.0, Hash40::new("s_tail6"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail6"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 4, Hash40::new("s_tail4"), 0.0, 0.0, 0.0, Hash40::new("s_tail5"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail5"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 4, Hash40::new("s_tail3"), 0.0, 0.0, 0.0, Hash40::new("s_tail4"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail4"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail2"), 0.0, 0.0, 0.0, Hash40::new("s_tail3"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail3"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_mewtwo_tail_purple"), Hash40::new("tex_mewtwo_tail_1"), 3, Hash40::new("s_tail1"), 0.0, 0.0, 0.0, Hash40::new("s_tail2"), 5.0, 0.0, 0.0, true, Hash40::new(""), Hash40::new("s_tail2"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail7"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail5"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail3"), 0, 0, 0, 0, 0, 0, 0.6, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail1"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail7"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail5"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail3"), 0, 0, 0, 0, 0, 0, 0.6, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("s_tail1"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_deathscythe_trace"), false, false);
        macros::AFTER_IMAGE_OFF(fighter, 4);
    }
}

//Dair ACMD
#[acmd_script( agent = "mewtwo", script = "game_attackairlw", category = ACMD_GAME)]
unsafe fn ssbuexo_mewtwo_dair_acmd(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 60, 0, 30, 3.0, 0.0, 0.0, -0.5, Some(0.0), Some(-2.5), Some(-0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 300, 40, 0, 30, 6.3, 0.0, -6.5, -0.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 16.0, 270, 40, 0, 30, 6.3, 0.0, -12.0, -5.0, Some(0.0), Some(-12.0), Some(5.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 60, 0, 30, 3.0, 0.0, 0.0, -0.5, Some(0.0), Some(-2.5), Some(-0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 361, 60, 0, 30, 6.3, 0.0, -6.5, -0.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 16.0, 361, 60, 0, 30, 6.3, 0.0, -12.0, -5.0, Some(0.0), Some(-12.0), Some(5.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 55.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Dair Effect
#[acmd_script( agent = "mewtwo", script = "effect_attackairlw", category = ACMD_EFFECT)]
unsafe fn ssbuexo_mewtwo_dair_effect(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_trace"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_deathscythe_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_shadowball_hold"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_attack_b"), Hash40::new("mewtwo_pk_attack_b"), Hash40::new("top"), 0, -2, -0.5, 0, -90, 0, 1, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_deathscythe_trace"), false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_deathscythe_aura"), false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_shadowball_hold"), false, false);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_mewtwo_nair_acmd,
        ssbuexo_mewtwo_nair_effect,
        ssbuexo_mewtwo_fair_acmd,
        ssbuexo_mewtwo_fair_effect,
        ssbuexo_mewtwo_bair_acmd,
        ssbuexo_mewtwo_bair_effect,
        ssbuexo_mewtwo_uair_acmd,
        ssbuexo_mewtwo_uair_effect,
        ssbuexo_mewtwo_dair_acmd,
        ssbuexo_mewtwo_dair_effect
    );
}