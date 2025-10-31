use super::*;

//Forward Smash Charge ACMD
unsafe extern "C" fn ssbexo_toonlink_forward_smash_charge_acmd(agent: &mut L2CAgentBase) {
    if !ArticleModule::is_exist(agent.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW) {
        if is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, false, -1);
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, Hash40::new("attack_s4_hold"), false, -1.0);
        }
    }
    else {
        if is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, Hash40::new("attack_s4_hold"), false, -1.0);
        }
    }
}

//Forward Smash Charge Effect
unsafe extern "C" fn ssbexo_toonlink_forward_smash_charge_effect(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 4.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 7, 0, 4, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 5.0);
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 0, 18, 0, 0, 0, 1, 5, 2.5, 2.5, 0, 0, 0, true);
        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

//Forward Smash Charge Expression
unsafe extern "C" fn ssbexo_toonlink_forward_smash_charge_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        physics!(agent, *MA_MSC_CMD_PHYSICS_START_CHARGE, 0.8, 0.8, -1, 0.8, 0.8, -1, Hash40::new("invalid"));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(agent.lua_state_agent, 61.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold2"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Forward Smash ACMD
unsafe extern "C" fn ssbexo_toonlink_forward_smash_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, Hash40::new("attack_s4"), false, -1.0);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 21.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, Hash40::new("attack_s4"), false, -1.0);
            ArticleModule::set_frame(agent.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, 21.0);
        }
    }
    frame(agent.lua_state_agent, 29.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 24.0, 361, 50, 0, 85, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 24.0, 361, 50, 0, 85, 3.5, 0.0, 6.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 24.0, 361, 50, 0, 85, 3.5, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 24.0, 361, 50, 0, 85, 7.0, 0.0, 18.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 33.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 68.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, ArticleOperationTarget(0));
    }
}

//Forward Smash Effect
unsafe extern "C" fn ssbexo_toonlink_forward_smash_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 18, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("toonlink_kaiten_flare"), Hash40::new("haver"), 0, 18, 0, 0, 0, 0, 1, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("toonlink_sword3"), Hash40::new("toonlink_sword2"), 5, Hash40::new("haver"), 0, 0, 0, Hash40::new("haver"), 0, 18.0, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.2);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 1.5, 0, 18, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 1.5, 0, 18, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
    frame(agent.lua_state_agent, 32.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("toonlink_kaiten_flare"), false, false);
    }
}

//Forward Smash Sound
unsafe extern "C" fn ssbexo_toonlink_forward_smash_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start"));
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_toonlink_attack05"));
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_smashswing_03"));
        PLAY_SE(agent, Hash40::new("se_toonlink_swing_ll"));
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        PLAY_LANDING_SE(agent, Hash40::new("se_toonlink_attackair_l01"));
    }
}

//Forward Smash Expression
unsafe extern "C" fn ssbexo_toonlink_forward_smash_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(agent.lua_state_agent, 20.0);
    sv_animcmd::execute(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl_l"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
    frame(agent.lua_state_agent, 29.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 32.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(agent.lua_state_agent, 68.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
    frame(agent.lua_state_agent, 83.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
    }
}

//Down Smash ACMD
unsafe extern "C" fn ssbexo_toonlink_down_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 42, 85, 0, 55, 4.0, 0.0, 2.2, 16.0, Some(0.0), Some(2.9), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 42, 85, 0, 55, 4.3, 0.0, 4.2, -16.0, Some(0.0), Some(4.2), Some(-5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    FT_MOTION_RATE(agent, 0.88);
    frame(agent.lua_state_agent, 50.0);
    FT_MOTION_RATE(agent, 1.0);
}

pub fn install() {
    Agent::new("toonlink")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attacks4charge", ssbexo_toonlink_forward_smash_charge_acmd, Low)
    .effect_acmd("effect_attacks4charge", ssbexo_toonlink_forward_smash_charge_effect, Low)
    .expression_acmd("expression_attacks4charge", ssbexo_toonlink_forward_smash_charge_expression, Low)
    .game_acmd("game_attacks4", ssbexo_toonlink_forward_smash_acmd, Low)
    .effect_acmd("effect_attacks4", ssbexo_toonlink_forward_smash_effect, Low)
    .sound_acmd("sound_attacks4", ssbexo_toonlink_forward_smash_sound, Low)
    .expression_acmd("expression_attacks4", ssbexo_toonlink_forward_smash_expression, Low)
    .game_acmd("game_attacklw4", ssbexo_toonlink_down_smash_acmd, Low)
    .install()
    ;
}