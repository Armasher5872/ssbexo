use super::*;

//Forward Smash ACMD
#[acmd_script(agent = "samusd", scripts = ["game_attacks4", "game_attacks4hi", "game_attacks4lw"], category = ACMD_GAME)]
unsafe fn ssbuexo_samusd_forward_smash_acmd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    MotionModule::set_rate(fighter.module_accessor, 0.333333);
    frame(fighter.lua_state_agent, 8.0);
    MotionModule::set_rate(fighter.module_accessor, 1.0);
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 0.25);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.1, 0, 5, 5, 0, 4.5, 0.0, 10.5, 46.0, Some(0.0), Some(10.5), Some(10.0), 0.3, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_ENERGY);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 416, 16, 0.5, false);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::ATTACK(fighter, 0, 1, Hash40::new("armr"), 6.0, 361, 90, 0, 40, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 1, Hash40::new("armr"), 6.0, 361, 90, 0, 40, 3.3, -4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 1, Hash40::new("top"), 8.0, 361, 90, 0, 40, 4.5, 0.0, 10.5, 46.0, Some(0.0), Some(10.5), Some(10.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_ENERGY);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Forward Smash Effect
#[acmd_script( agent = "samusd", scripts = ["effect_attacks4", "effect_attacks4hi", "effect_attacks4lw"], category = ACMD_EFFECT)]
unsafe fn ssbuexo_samusd_forward_smash_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("armr"), 4.289, -0.272, -0.135, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 7, 0, 0, 0, 0, 0, 1.3, true, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        sv_animcmd::EFFECT_FOLLOW_arg11(fighter.lua_state_agent);
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, 3, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(fighter, Hash40::new("sys_machstamp"), Hash40::new("top"), 0.0, 10.0, 20.0, 90, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.8);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_machstamp"), false, true);
    }
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_gbeam_flash_01"), false, true);
    }
}

//Up Smash ACMD
#[acmd_script( agent = "samusd", script = "game_attackhi4", category = ACMD_GAME)]
unsafe fn ssbuexo_samusd_up_smash_acmd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 80, 162, 0, 50, 8.0, 0.0, 22.0, 18.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 416, 16, 0.5, false);
        MotionModule::set_rate(fighter.module_accessor, 0.5);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 80, 162, 0, 50, 8.0, 0.0, 25.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        AttackModule::set_poison_param(fighter.module_accessor, 1, 416, 16, 0.5, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    MotionModule::set_rate(fighter.module_accessor, 0.5);
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 80, 162, 0, 50, 8.0, 0.0, 22.0, -18.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        AttackModule::set_poison_param(fighter.module_accessor, 2, 416, 16, 0.5, false);
    }
    frame(fighter.lua_state_agent, 28.0);
    MotionModule::set_rate(fighter.module_accessor, 0.5);
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Up Smash Effect
#[acmd_script( agent = "samusd", script = "effect_attackhi4", category = ACMD_EFFECT)]
unsafe fn ssbuexo_samusd_up_smash_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("armr"), 4.289, -0.272, -0.135, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("samusd_atk_bomb"), Hash40::new("top"), 0.0, 22.0, 18.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(fighter, Hash40::new("samusd_entry"), Hash40::new("top"), 0.0, 22.0, 18.0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_v"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("samusd_atk_bomb"), Hash40::new("armr"), 0.0, 25.0, 0.0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(fighter, Hash40::new("samusd_entry"), Hash40::new("top"), 0.0, 25.0, 0.0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("samusd_atk_bomb"), Hash40::new("armr"), -18.0, 22.0, 0.0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(fighter, Hash40::new("samusd_entry"), Hash40::new("top"), 0.0, 22.0, -18.0, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, true);
    }
}

//Up Smash Sound
#[acmd_script( agent = "samusd", script = "sound_attackhi4", category = ACMD_SOUND)]
unsafe fn ssbuexo_samusd_up_smash_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samusd_swing_l"));
        macros::PLAY_SE(fighter, Hash40::new("se_samusd_smash_h01"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samusd_smash_h01"));
    }
    wait(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samusd_smash_h01"));
    }
}

//Down Smash ACMD
#[acmd_script(agent = "samusd", script = "game_attacklw4", category = ACMD_GAME)]
unsafe fn ssbuexo_samusd_down_smash_acmd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 90, 140, 80, 0, 4.0, 0.0, 2.0, -12.0, Some(0.0), Some(2.0), Some(12.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 416, 16, 0.5, false);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.9, 90, 60, 0, 90, 6.0, 0.0, 6.0, -12.0, Some(0.0), Some(6.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 0.1, 90, 60, 0, 90, 6.1, 0.0, 6.0, -12.0, Some(0.0), Some(6.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_ENERGY);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 416, 16, 0.5, false);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.9, 90, 60, 0, 90, 10.0, 0.0, 10.0, -12.0, Some(0.0), Some(10.0), Some(12.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 0.1, 90, 60, 0, 90, 10.1, 0.0, 10.0, -12.0, Some(0.0), Some(10.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_ENERGY);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 416, 16, 0.5, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.9, 90, 60, 0, 90, 12.0, 0.0, 12.0, -12.0, Some(0.0), Some(12.0), Some(12.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 0.1, 90, 60, 0, 90, 12.1, 0.0, 12.0, -12.0, Some(0.0), Some(12.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_ENERGY);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 416, 16, 0.5, false);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.9, 90, 55, 0, 85, 12.0, 0.0, 12.0, -12.0, Some(0.0), Some(12.0), Some(12.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 0.1, 90, 55, 0, 90, 12.1, 0.0, 12.0, -12.0, Some(0.0), Some(12.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_ENERGY);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 416, 16, 0.5, false);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.9, 90, 50, 0, 80, 12.0, 0.0, 12.0, -12.0, Some(0.0), Some(12.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 0.1, 90, 50, 0, 90, 12.1, 0.0, 12.0, -12.0, Some(0.0), Some(12.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_ENERGY);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 416, 16, 0.5, false);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Down Smash Effect
#[acmd_script(agent = "samusd", script = "effect_attacklw4", category = ACMD_EFFECT)]
unsafe fn ssbuexo_samusd_down_smash_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), -2.0, 0.0, 0.0, 0, 0, 0, 2.5, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new_raw(0x09aee445d1), 2.0, 0.0, 0.5, 0, 0, 0, 2.0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), 0.0, 0.0, -0.5, 0, 0, 0, 1.70000005, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), 0.0, 0.0, 0.0, 0, 0, 0, 2.0999999, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0.0, 0.0, 0.0, 0, 0, 0, 1.89999998, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0.0, 0.0, 0.0, 0, 0, 0, 2.0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new_raw(0x0954eb78b2), 2.0, 0.0, -0.5, 0, 0, 0, 2.0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), 0.0, 0.0, 0.0, 0, 0, 0, 1.70000005, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), 0.0, 0.0, 0.0, 0, 0, 0, 2.0999999, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0.0, 0.0, 0.0, 0, 0, 0, 1.89999998, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0.0, 0.0, 0.0, 0, 0, 0, 1.89999998, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_throw_hi"), Hash40::new("top"), 0, 0, -11, 0, 0, 0, 1.0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.75);
        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x092a3b5b68), Hash40::new("top"), 0, 0, -11, 0, 0, 0, 0.65, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, -11, 0, 0, 0, 0.75, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_throw_hi"), Hash40::new("top"), 0, 0, 17, 0, 0, 0, 1.0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.75);
        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x092a3b5b68), Hash40::new("top"), 0, 0, 17, 0, 0, 0, 0.65, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 17, 0, 0, 0, 0.75, true);
    }
}

//Down Smash Sound
#[acmd_script(agent = "samusd", script = "sound_attacklw4", category = ACMD_SOUND)]
unsafe fn ssbuexo_samusd_down_smash_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samusd_swing_l"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samusd_special_n04"));
    }
}

//Down Smash Expression
#[acmd_script(agent = "samusd", script = "expression_attacklw4", category = ACMD_EXPRESSION)]
unsafe fn ssbuexo_samusd_down_smash_expression(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3, true);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, 0x50000000);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 20);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_samusd_forward_smash_acmd,
        ssbuexo_samusd_forward_smash_effect,
        ssbuexo_samusd_up_smash_acmd,
        ssbuexo_samusd_up_smash_effect,
        ssbuexo_samusd_up_smash_sound,
        ssbuexo_samusd_down_smash_acmd,
        ssbuexo_samusd_down_smash_effect,
        ssbuexo_samusd_down_smash_sound,
        ssbuexo_samusd_down_smash_expression
    );
}