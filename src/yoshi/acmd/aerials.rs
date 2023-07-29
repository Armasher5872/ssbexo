use super::*;

//Nair ACMD
#[acmd_script( agent = "yoshi", script = "game_attackairn", category = ACMD_GAME)]
unsafe fn ssbuexo_yoshi_nair_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_YOSHI_STATUS_SPECIAL_S_FLAG_DISP_EGG);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 361, 110, 0, 20, 4.8, 7.5, 0.0, -1.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 361, 110, 0, 20, 3.6, 7.5, 0.0, -1.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 110, 0, 20, 3.0, 7.5, 0.0, -1.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_YOSHI_STATUS_SPECIAL_S_FLAG_HIDE_EGG);
    }
    frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Nair Effect
#[acmd_script( agent = "yoshi", script = "effect_attackairn", category = ACMD_EFFECT)]
unsafe fn ssbuexo_yoshi_nair_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 2, 5, 0, 0, 0, 0, 2.4, 0, 0, 0, 0, 0, 360, true);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("tamago_kakera"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        sv_animcmd::EFFECT_FOLLOW_VARIATION(fighter.lua_state_agent);
    }
}

//Nair Sound
#[acmd_script( agent = "yoshi", script = "sound_attackairn", category = ACMD_SOUND)]
unsafe fn ssbuexo_yoshi_nair_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_yoshi_special_s02"));
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_yoshi_special_s04"));
    }
}

//Nair Expression
#[acmd_script( agent = "yoshi", script = "expression_attackairn", category = ACMD_EXPRESSION)]
unsafe fn ssbuexo_yoshi_nair_expression(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_egg") as i64);
        macros::HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("snout"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("toer"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("toel"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("tail2"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_NORMAL);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
        macros::HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("snout"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("toer"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("toel"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("tail2"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_OFF);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_awaken"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 0);
    }
}

//Nair Landing Effect
#[acmd_script( agent = "yoshi", script = "effect_landingairn", category = ACMD_EFFECT)]
unsafe fn ssbuexo_yoshi_nair_landing_effect(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("tamago_kakera"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        sv_animcmd::EFFECT_FOLLOW_VARIATION(fighter.lua_state_agent);
    }
}

//Nair Landing Expression
#[acmd_script( agent = "yoshi", script = "expression_landingairn", category = ACMD_EXPRESSION)]
unsafe fn ssbuexo_yoshi_nair_landing_expression(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_egg") as i64);
        macros::HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("snout"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("toer"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("toel"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("tail2"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_NORMAL);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
        macros::HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("snout"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("toer"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("toel"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("tail2"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_OFF);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_awaken"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 0);
    }
}

//Fair ACMD
#[acmd_script( agent = "yoshi", script = "game_attackairf", category = ACMD_GAME)]
unsafe fn ssbuexo_yoshi_fair_acmd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 15.0, 361, 90, 0, 0, 4.2, 1.5, -0.5, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("head"), 14.0, 270, 90, 0, 0, 5.0, 7.0, 1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 67.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

//Bair ACMD
#[acmd_script( agent = "yoshi", script = "game_attackairb", category = ACMD_GAME)]
unsafe fn ssbuexo_yoshi_bair_acmd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 367, 100, 0, 20, 5.5, 0.0, 5.0, -10.0, Some(0.0), Some(3.0), Some(-10.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_TAIL);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 367, 100, 0, 20, 5.5, 0.0, 5.0, -10.0, Some(0.0), Some(3.0), Some(-10.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_TAIL);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 54, 120, 0, 30, 8.0, 0.0, 5.0, -11.0, Some(0.0), Some(5.0), Some(-9.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_TAIL);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Dair ACMD
#[acmd_script( agent = "yoshi", script = "game_attackairlw", category = ACMD_GAME)]
unsafe fn ssbuexo_yoshi_dair_acmd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 367, 80, 5, 0, 5.0, 0.0, 1.0, 2.0, None, None, None, 0.6, 1.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 367, 80, 5, 0, 3.5, 0.0, 2.0, 6.5, None, None, None, 0.6, 1.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 367, 80, 5, 0, 3.5, 0.0, 2.0, -2.5, None, None, None, 0.6, 1.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 367, 80, 5, 0, 5.0, 0.0, 1.0, 2.0, None, None, None, 0.6, 1.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 367, 80, 5, 0, 3.5, 0.0, 2.0, 6.5, None, None, None, 0.6, 1.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 367, 80, 5, 0, 3.5, 0.0, 2.0, -2.5, None, None, None, 0.6, 1.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 367, 80, 5, 0, 5.0, 0.0, 1.0, 2.0, None, None, None, 0.6, 1.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 367, 80, 5, 0, 3.5, 0.0, 2.0, 6.5, None, None, None, 0.6, 1.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 367, 80, 5, 0, 3.5, 0.0, 2.0, -2.5, None, None, None, 0.6, 1.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 367, 80, 5, 0, 5.0, 0.0, 1.0, 2.0, None, None, None, 0.6, 1.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 367, 80, 5, 0, 3.5, 0.0, 2.0, 6.5, None, None, None, 0.6, 1.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 367, 80, 5, 0, 3.5, 0.0, 2.0, -2.5, None, None, None, 0.6, 1.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 38.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 100, 0, 50, 6.0, 0.0, 1.0, 2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 361, 100, 0, 50, 4.5, 0.0, 2.0, 6.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 361, 100, 0, 50, 4.5, 0.0, 2.0, -2.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 41.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_yoshi_nair_acmd,
        ssbuexo_yoshi_nair_effect,
        ssbuexo_yoshi_nair_sound,
        ssbuexo_yoshi_nair_expression,
        ssbuexo_yoshi_nair_landing_effect,
        ssbuexo_yoshi_nair_landing_expression,
        ssbuexo_yoshi_fair_acmd,
        ssbuexo_yoshi_bair_acmd,
        ssbuexo_yoshi_dair_acmd
    );
}