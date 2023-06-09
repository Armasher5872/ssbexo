use super::*;

//Forward Smash Charge Effect
#[acmd_script( agent = "plizardon", script = "effect_attacks4charge", category = ACMD_EFFECT)]
unsafe fn ssbuexo_plizardon_forward_smash_charge_effect(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 15, 0, 8, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    for _ in 0..59 {
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 3, 4.0, 0, 0, 0, 0, 1, 2, 2, 2, 0, 0, 0, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_fire"), Hash40::new("haver"), 1.5, 0.0, 0.0, 0, 0, 0, 0.4, true);
        }
        wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 3, 4.0, 0, 0, 0, 0, 1, 2, 2, 2, 0, 0, 0, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_fire"), Hash40::new("haver"), 1.5, 0.0, 0.0, 0, 0, 0, 0.4, true);
        }
    }
}

//Forward Smash ACMD
#[acmd_script( agent = "plizardon", script = "game_attacks4", category = ACMD_GAME)]
unsafe fn ssbuexo_plizardon_forward_smash_acmd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 21.0, 361, 50, 0, 75, 10.0, 0.0, 5.0, 10.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Forward Smash Effect
#[acmd_script( agent = "plizardon", script = "effect_attacks4", category = ACMD_EFFECT)]
unsafe fn ssbuexo_plizardon_forward_smash_effect(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 3.0);
    for _ in 0..3 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_fire"), Hash40::new("haver"), 1.5, 0.0, 0.0, 0, 0, 0, 0.4, true);
        }
        wait(fighter.lua_state_agent, 2.0);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_fire"), Hash40::new("haver"), 1.5, 0.0, 0.0, 0, 0, 0, 0.4, true);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_fire"), Hash40::new("haver"), 1.5, 0.0, 0.0, 0, 0, 0, 0.4, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_fire"), Hash40::new("haver"), 1.5, 0.0, 0.0, 0, 0, 0, 0.4, true);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0.0, 5.0, 10.0, 0, 0, 0, 1.0, true);
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
    frame(fighter.lua_state_agent, 55.0);
    if macros::is_excute(fighter) {
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bomb_a"), false, true);
    }
}

//Up Smash ACMD
#[acmd_script( agent = "plizardon", script = "game_attackhi4", category = ACMD_GAME)]
unsafe fn ssbuexo_plizardon_up_smash_acmd(fighter: &mut L2CAgentBase) {
    let vector = &mut Vector2f{x: 0.0, y: 18.0};
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    MotionModule::set_rate(fighter.module_accessor, 0.6);
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 110, 100, 100, 0, 6.0, 0.0, 9.0, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 7.0);
    macros::FT_MOTION_RATE(fighter, 0.6);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("wingl4"), 5.0, 368, 100, 0, 0, 4.2, 1.5, 0.0, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 2, 0, Hash40::new("wingl2"), 5.0, 368, 100, 0, 0, 5.0, 3.5, -1.5, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 3, 0, Hash40::new("wingl4"), 5.0, 280, 100, 32, 0, 4.2, 7.0, 0.0, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        AttackModule::clear(fighter.module_accessor, 0, false);
        AttackModule::set_vec_target_pos(fighter.module_accessor, 1, Hash40::new("top"), vector, 8, false);
        AttackModule::set_vec_target_pos(fighter.module_accessor, 2, Hash40::new("top"), vector, 8, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 3, 0, Hash40::new("wingl4"), 5.0, 320, 100, 42, 0, 4.2, 7.0, 0.0, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 3, 0, Hash40::new("wingl4"), 5.0, 340, 100, 60, 0, 4.2, 7.0, 0.0, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 0, 0, Hash40::new("wingl4"), 5.0, 368, 100, 0, 0, 4.2, 7.0, 0.0, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("top"), vector, 8, false);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 13.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    macros::FT_START_ADJUST_MOTION_FRAME_arg1(fighter, 1.0);
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("wingr2"), 10.0, 83, 107, 0, 40, 5.5, 3.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("wingr4"), 10.0, 83, 107, 0, 40, 5.0, 2.7, -2.0, -2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 2, 0, Hash40::new("wingr7"), 10.0, 83, 107, 0, 40, 4.5, 6.5, 0.0, 6.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_plizardon_forward_smash_charge_effect,
        ssbuexo_plizardon_forward_smash_acmd,
        ssbuexo_plizardon_forward_smash_effect,
        ssbuexo_plizardon_up_smash_acmd
    );
}