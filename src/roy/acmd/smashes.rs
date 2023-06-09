use super::*;

//Forward Smash ACMD
#[acmd_script( agent = "roy", script = "game_attacks4", category = ACMD_GAME)]
unsafe fn ssbuexo_roy_forward_smash_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 12.0);
    MotionModule::set_rate(fighter.module_accessor, 2.0);
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 361, 75, 0, 60, 3.3, 0.0, 7.7, 9.1, Some(0.0), Some(7.7), Some(7.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 14.0, 361, 75, 0, 60, 4.0, 2.0, 1.0, 1.5, Some(9.5), Some(1.0), Some(1.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 12.0, 361, 75, 0, 60, 4.0, 2.0, 1.0, 7.5, Some(14.0), Some(1.0), Some(14.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("sword1"), 12.0, 361, 75, 0, 60, 4.0, 2.0, 1.0, 7.5, Some(2.0), Some(1.0), Some(19.5), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 4, 0, Hash40::new("sword1"), 12.0, 361, 70, 0, 50, 3.5, 2.0, 1.0, 7.5, Some(2.5), Some(1.0), Some(24.5), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        }
        else {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 361, 75, 0, 60, 3.3, 0.0, 7.7, 9.1, Some(0.0), Some(7.7), Some(7.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 14.0, 361, 75, 0, 60, 4.0, 2.0, 1.0, 1.5, Some(14.0), Some(1.0), Some(1.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 12.0, 361, 70, 0, 50, 3.5, 2.0, 1.0, 7.5, Some(9.5), Some(1.0), Some(7.5), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        }   
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    }
}

//Forward Smash Effect
#[acmd_script( agent = "roy", script = "effect_attacks4", category = ACMD_EFFECT)]
unsafe fn ssbuexo_roy_forward_smash_effect(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 4, 16, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
            macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
            smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
            fighter.pop_lua_stack(1);
            macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 7, 0, 0, 0, 1, true);
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 14.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
            smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
            fighter.pop_lua_stack(1);
            macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 14, 0, 0, 0, 1, true);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 15, Hash40::new("sword1"), 0.0, 0.0, 12.0, Hash40::new("sword1"), -0.0, -0.0, 28.0, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword1"), Hash40::new("tex_roy_sword2"), 5, Hash40::new("sword1"), 0.0, 0.0, -0.8, Hash40::new("sword1"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        else {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword1"), Hash40::new("tex_roy_sword2"), 5, Hash40::new("sword1"), 0.0, 0.0, -0.8, Hash40::new("sword1"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 6);
    }
}

//Up Smash ACMD
#[acmd_script( agent = "roy", script = "game_attackhi4", category = ACMD_GAME)]
unsafe fn ssbuexo_roy_up_smash_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 118, 100, 120, 0, 3.0, 0.0, 9.0, 4.0, Some(0.0), Some(9.0), Some(-3.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 127, 100, 150, 0, 3.0, 0.0, 9.0, 7.8, Some(0.0), Some(9.0), Some(-6.8), 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false); 
    }
    frame(fighter.lua_state_agent, 13.0);
    for _ in 0..4 {
        if macros::is_excute(fighter) {
            if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 2.0, 98, 100, 85, 0, 5.4, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 2.0, 260, 100, 55, 0, 4.9, 0.0, 0.0, 5.8, Some(0.0), Some(0.0), Some(18.8), 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
                AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false); 
            }
            else {
                macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 2.0, 98, 100, 85, 0, 5.4, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 2.0, 260, 100, 55, 0, 4.9, 0.0, 0.0, 5.8, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
                AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false); 
            }
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        wait(fighter.lua_state_agent, 1.0);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 10.0, 90, 70, 0, 70, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 10.0, 90, 70, 0, 70, 6.9, 0.0, 0.0, 6.2, Some(0.0), Some(0.0), Some(19.2), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 10.0, 90, 70, 0, 70, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 10.0, 90, 70, 0, 70, 6.9, 0.0, 0.0, 6.2, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        }
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    }
}

//Up Smash Effect
#[acmd_script( agent = "roy", script = "effect_attackhi4", category = ACMD_EFFECT)]
unsafe fn ssbuexo_roy_up_smash_effect(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 4, 16, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
            macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
            smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
            fighter.pop_lua_stack(1);
            macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 7, 0, 0, 0, 1, true);
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 14.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
            smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
            fighter.pop_lua_stack(1);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_club_tornado"), Hash40::new("top"), 0, 6, 0, 0.0, 0.0, 0.0, 1.0, true);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.88, 0.35, 0.13);
        }
        else {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("roy_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("roy_smash_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 0.9, true);
        }
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("roy_smash_bomb"), Hash40::new("sword1"), 0, 0, 7, 90, 0, 0, 0.48, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.25);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("roy_sword"), false, false);
    }
}

//Down Smash ACMD
#[acmd_script( agent = "roy", script = "game_attacklw4", category = ACMD_GAME)]
unsafe fn ssbuexo_roy_down_smash_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 14.0, 30, 60, 0, 40, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 12.5, 30, 60, 0, 40, 3.5, 0.0, 0.0, 3.5, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 11.2, 30, 60, 0, 40, 3.5, 0.0, 0.0, 7.2, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("sword1"), 11.2, 30, 60, 0, 40, 3.5, 0.0, 0.0, 11.2, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 4, 0, Hash40::new("sword1"), 11.2, 30, 60, 0, 40, 3.5, 0.0, 0.0, 15.2, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 5, 0, Hash40::new("sword1"), 11.2, 30, 60, 0, 40, 3.5, 0.0, 0.0, 19.2, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 14.0, 30, 60, 0, 40, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 12.5, 30, 60, 0, 40, 3.5, 0.0, 0.0, 3.5, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ROY_HIT, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 11.2, 30, 60, 0, 40, 3.5, 0.0, 0.0, 7.2, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }   
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    }
}

//Down Smash Effect
#[acmd_script( agent = "roy", script = "effect_attacklw4", category = ACMD_EFFECT)]
unsafe fn ssbuexo_roy_down_smash_effect(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 4, 16, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
            macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
            smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
            fighter.pop_lua_stack(1);
            macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 7, 0, 0, 0, 1, true);
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 14.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
            smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
            fighter.pop_lua_stack(1);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 15, Hash40::new("sword1"), 0.0, 0.0, 14.0, Hash40::new("sword1"), -0.0, -0.0, 28.0, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_club_tornado"), Hash40::new("top"), 0, 6, 0, 0.0, 0.0, 0.0, 1.0, true);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.88, 0.35, 0.13);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("roy_sword_light"), Hash40::new("sword1"), 0, 0, 10.55, 0, 0, 0, 1, true);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.4);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("roy_sword_light"), Hash40::new("sword1"), 0, 0, 7.0, 0, 0, 0, 1, true);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.4);
        }
        else {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword1"), Hash40::new("tex_roy_sword2"), 10, Hash40::new("sword1"), 0.0, 0.0, -0.8, Hash40::new("sword1"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("roy_sword_light"), Hash40::new("sword1"), 0, 0, 10.55, 0, 0, 0, 1, true);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.4);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("roy_sword_light"), Hash40::new("sword1"), 0, 0, 7.0, 0, 0, 0, 1, true);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.4);
        }
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 4);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("roy_sword_light"), false, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("roy_sword"), false, false);
    }
}

//Down Smash Sound
#[acmd_script( agent = "roy", script = "sound_attacklw4", category = ACMD_SOUND)]
unsafe fn ssbuexo_roy_down_smash_sound(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_roy_rnd_attack_smash_l"));
        macros::PLAY_SE(fighter, Hash40::new("se_roy_attackl_l01"));
    }
}

//Down Smash Expression
#[acmd_script( agent = "roy", script = "expression_attacklw4", category = ACMD_EXPRESSION)]
unsafe fn ssbuexo_roy_down_smash_expression(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashll"), 0);
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 1);
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 2);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 16);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_roy_forward_smash_acmd,
        ssbuexo_roy_forward_smash_effect,
        ssbuexo_roy_up_smash_acmd,
        ssbuexo_roy_up_smash_effect,
        ssbuexo_roy_down_smash_acmd,
        ssbuexo_roy_down_smash_effect,
        ssbuexo_roy_down_smash_sound,
        ssbuexo_roy_down_smash_expression
    );
}