use super::*;

//Forward Tilt ACMD
#[acmd_script( agent = "yoshi", scripts = ["game_attacks3", "game_attacks3hi", "game_attacks3lw"], category = ACMD_GAME)]
unsafe fn ssbuexo_yoshi_forward_tilt_acmd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("tail1"), 8.0, 70, 75, 0, 60, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 1, 0, Hash40::new("tail1"), 8.0, 95, 75, 0, 60, 3.5, 4.3, -1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 2, 0, Hash40::new("tail2"), 8.0, 95, 75, 0, 60, 3.0, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_TAIL);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Forward Tilt F Effect
#[acmd_script( agent = "yoshi", script = "effect_attacks3", category = ACMD_EFFECT)]
unsafe fn ssbuexo_yoshi_forward_tilt_f_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 1, 9, 6, -3.255, -12.797, 37.054, 0.85, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("yoshi_tamago_line"), Hash40::new("tail3"), 0, 0, 0, 0, 0, 0, 1, false);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("yoshi_tamago_line"), false, true);
    }
}

//Forward Tilt Hi Effect
#[acmd_script( agent = "yoshi", script = "effect_attacks3hi", category = ACMD_EFFECT)]
unsafe fn ssbuexo_yoshi_forward_tilt_hi_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 1, 11, 6, -7.79, -24, 49.865, 0.85, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.7);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("yoshi_tamago_line"), Hash40::new("tail3"), 0, 0, 0, 0, 0, 0, 1, false);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("yoshi_tamago_line"), false, true);
    }
}

//Forward Tilt Lw Effect
#[acmd_script( agent = "yoshi", script = "effect_attacks3lw", category = ACMD_EFFECT)]
unsafe fn ssbuexo_yoshi_forward_tilt_lw_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 1, 5.5, 6, 1.53, -13.258, 23.277, 0.85, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("yoshi_tamago_line"), Hash40::new("tail3"), 0, 0, 0, 0, 0, 0, 1, false);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("yoshi_tamago_line"), false, true);
    }
}

//Up Tilt ACMD
#[acmd_script( agent = "yoshi", script = "game_attackhi3", category = ACMD_GAME)]
unsafe fn ssbuexo_yoshi_up_tilt_acmd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.9);
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("tail1"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("tail2"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("tail3"), *HIT_STATUS_XLU);
        macros::FT_MOTION_RATE(fighter, 1.0);
        macros::ATTACK(fighter, 0, 0, Hash40::new("tail2"), 7.0, 100, 45, 0, 72, 4.0, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 1, 0, Hash40::new("tail2"), 7.0, 100, 45, 0, 72, 4.5, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 2, 0, Hash40::new("tail1"), 7.0, 100, 45, 0, 72, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_TAIL);
    }
    wait(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("tail1"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("tail2"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("tail3"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Up Tilt Effect
#[acmd_script( agent = "yoshi", script = "effect_attackhi3", category = ACMD_EFFECT)]
unsafe fn ssbuexo_yoshi_up_tilt_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 1, 13, -1, 0, 52, 88, 1.15, true, *EF_FLIP_YZ);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("yoshi_tamago_line"), Hash40::new("tail3"), 0, 0, 0, 0, 0, 0, 1, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("yoshi_tamago_line"), false, true);
    }
}

//Down Tilt ACMD
#[acmd_script( agent = "yoshi", script = "game_attacklw3", category = ACMD_GAME)]
unsafe fn ssbuexo_yoshi_down_tilt_acmd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.1);
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("tail1"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("tail2"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("tail3"), *HIT_STATUS_XLU);
        macros::ATTACK(fighter, 0, 0, Hash40::new("tail1"), 5.0, 28, 30, 0, 67, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 1, 0, Hash40::new("tail2"), 4.5, 28, 30, 0, 67, 3.5, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 2, 0, Hash40::new("tail3"), 4.0, 361, 30, 0, 67, 3.0, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_TAIL);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("tail1"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("tail2"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("tail3"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(fighter.module_accessor);
    }
    macros::FT_MOTION_RATE(fighter, 0.9);
}

//Down Tilt Effect
#[acmd_script( agent = "yoshi", script = "effect_attacklw3", category = ACMD_EFFECT)]
unsafe fn ssbuexo_yoshi_down_tilt_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 4, 4, 0, 35, 180, 1.2, true, *EF_FLIP_YZ);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("yoshi_tamago_line"), Hash40::new("tail3"), 0, 0, 0, 0, 0, 0, 0.5, false);
        macros::LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, -2.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("yoshi_tamago_line"), false, true);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_yoshi_forward_tilt_acmd,
        ssbuexo_yoshi_forward_tilt_f_effect,
        ssbuexo_yoshi_forward_tilt_hi_effect,
        ssbuexo_yoshi_forward_tilt_lw_effect,
        ssbuexo_yoshi_up_tilt_acmd,
        ssbuexo_yoshi_up_tilt_effect,
        ssbuexo_yoshi_down_tilt_acmd,
        ssbuexo_yoshi_down_tilt_effect
    );
}