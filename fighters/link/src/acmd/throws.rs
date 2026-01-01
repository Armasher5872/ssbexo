//Credit to AParticularUser for Back Throw's animation
use super::*;

//Standing Grab ACMD
unsafe extern "C" fn ssbexo_link_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.3, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(7.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}


//Dash Grab ACMD
unsafe extern "C" fn ssbexo_link_dash_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 2.6, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(9.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

//Pivot Grab ACMD
unsafe extern "C" fn ssbexo_link_pivot_grab_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.3, 0.0, 7.0, -4.0, Some(0.0), Some(7.0), Some(-13.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

//Back Throw ACMD
unsafe extern "C" fn ssbexo_link_back_throw_acmd(agent: &mut L2CAgentBase) {
    let capture_id = LinkModule::get_node_object_id(agent.module_accessor, *LINK_NO_CAPTURE) as u32;
    let capture_boma = sv_battle_object::module_accessor(capture_id);
    let node = WorkModule::get_param_int64(capture_boma, hash40("edge_flare_node"), 0);
    if is_excute(agent) {
        MotionModule::add_motion_2nd(capture_boma, Hash40::new("turn_run_brake"), 0.0, 1.0, false, 0.0);
        MotionModule::set_weight(capture_boma, 0.0, false);
        LinkModule::set_model_constraint_pos_ort(capture_boma, *LINK_NO_CAPTURE, Hash40::new("trans"), Hash40::new("throw"), (*CONSTRAINT_FLAG_NO_FLIP | *CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_OFFSET_ROT | *CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32, true);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 2.5, 130, 140, 0, 78, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        MotionModule::set_weight(capture_boma, 1.0, false);
        LinkModule::set_model_constraint_pos_ort(capture_boma, *LINK_NO_CAPTURE, Hash40::new_raw(node), Hash40::new("throw"), (*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_OFFSET_ROT | *CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32, true);
        ATTACK(agent, 0, 0, Hash40::new("sword2"), 7.0, 20, 100, 0, 55, 4.8, 7.8, 0.0, 0.0, None, None, None, 0.8, 1.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword2"), 7.0, 20, 100, 0, 55, 4.8, 2.0, 0.0, 0.0, None, None, None, 0.8, 1.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        CHECK_FINISH_CAMERA(agent, -18, 12);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
    }
}

//Back Throw Effect
unsafe extern "C" fn ssbexo_link_back_throw_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1.5, 0, -1.5, 0, 180, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_link_sword1"), Hash40::new("tex_link_sword2"), 6, Hash40::new("sword1"), 1, 0, 0, Hash40::new("sword1"), 14.6, 0.2, -0.2, true, Hash40::new("link_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 12, -20.5, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 5);
    }
    frame(agent.lua_state_agent, 42.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.3);
    }
}

//Back Throw Sound
unsafe extern "C" fn ssbexo_link_back_throw_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_escape"));
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_swing_l"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_link_rnd_attack"));
    }
}

//Back Throw Expression
unsafe extern "C" fn ssbexo_link_back_throw_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        agent.clear_lua_stack();
        lua_args!(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, *CAMERA_QUAKE_KIND_NONE);
        sv_animcmd::FT_ATTACK_ABS_CAMERA_QUAKE(agent.lua_state_agent);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
    frame(agent.lua_state_agent, 44.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
}

//Up Throw ACMD
unsafe extern "C" fn ssbexo_link_up_throw_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 2.0, 90, 230, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword2"), 5.0, 20, 100, 0, 50, 3.8, 6.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword2"), 5.0, 20, 100, 0, 50, 3.8, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        CHECK_FINISH_CAMERA(agent, 6, 28);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.8);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 12.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Up Throw Effect
unsafe extern "C" fn ssbexo_link_up_throw_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_link_sword1"), Hash40::new("tex_link_sword2"), 5, Hash40::new("sword1"), 1, 0, 0, Hash40::new("sword1"), 14.6, 0.2, -0.2, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
        EFFECT_FOLLOW(agent, Hash40::new("link_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 30, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("link_sword"), false, false);
    }
    frame(agent.lua_state_agent, 46.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

//Up Throw Sound
unsafe extern "C" fn ssbexo_link_up_throw_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_swing_l"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_link_rnd_attack"));
    }
}

//Up Throw Expression
unsafe extern "C" fn ssbexo_link_up_throw_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        agent.clear_lua_stack();
        lua_args!(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, *CAMERA_QUAKE_KIND_NONE);
        sv_animcmd::FT_ATTACK_ABS_CAMERA_QUAKE(agent.lua_state_agent);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
    frame(agent.lua_state_agent, 46.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_catch", ssbexo_link_grab_acmd, Low)
    .game_acmd("game_catchdash", ssbexo_link_dash_grab_acmd, Low)
    .game_acmd("game_catchturn", ssbexo_link_pivot_grab_acmd, Low)
    .game_acmd("game_throwb", ssbexo_link_back_throw_acmd, Low)
    .effect_acmd("effect_throwb", ssbexo_link_back_throw_effect, Low)
    .sound_acmd("sound_throwb", ssbexo_link_back_throw_sound, Low)
    .expression_acmd("expression_throwb", ssbexo_link_back_throw_expression, Low)
    .game_acmd("game_throwhi", ssbexo_link_up_throw_acmd, Low)
    .effect_acmd("effect_throwhi", ssbexo_link_up_throw_effect, Low)
    .sound_acmd("sound_throwhi", ssbexo_link_up_throw_sound, Low)
    .expression_acmd("expression_throwhi", ssbexo_link_up_throw_expression, Low)
    .install()
    ;
}