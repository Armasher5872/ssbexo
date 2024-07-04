use super::*;

//Credit to C# for the Wall Climb

unsafe extern "C" fn ssbexo_link_attachwall_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_link_step_left_s_ft"));
        macros::STOP_SE(agent, Hash40::new("se_link_step_right_s_ft"));
    }
}

unsafe extern "C" fn ssbexo_link_attachwall_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
}

unsafe extern "C" fn ssbexo_link_attachwallclimb_sound(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_link_step_right_s_ft"));
        }
        wait(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_link_step_left_s_ft"));
        }
        wait(agent.lua_state_agent, 5.0);
    }
}

unsafe extern "C" fn ssbexo_link_attachwallclimb_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(agent.lua_state_agent, 1.0);
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(agent.lua_state_agent, 1.0);
    }
}

//Final Smash ACMD
unsafe extern "C" fn ssbexo_link_final_smash_acmd(agent: &mut L2CAgentBase) {
    let scale = PostureModule::scale(agent.module_accessor);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_CHARGE) {
        if macros::is_excute(agent) {
            macros::CHECK_VALID_FINAL_START_CAMERA(agent, 0, 7, 20, 0, 0, 0);
        }
        frame(agent.lua_state_agent, 10.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
            if macros::is_excute(agent) {
                macros::FT_SET_FINAL_FEAR_FACE(agent, 60);
                macros::REQ_FINAL_START_CAMERA(agent, Hash40::new("d04final.nuanmb"), false);
                macros::FT_START_CUTIN(agent);
            }
        }
        else {
            if macros::is_excute(agent) {
                macros::CAM_ZOOM_IN_arg5(agent, 1.8, 0.0, 3.0*scale, 0.0, 0.0);
                macros::FT_START_CUTIN(agent);
                camera!(agent, *MA_MSC_CMD_CAMERA_CAM_RECT, 0, -5, -2, 0);
            }
        }
        frame(agent.lua_state_agent, 30.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
            if macros::is_excute(agent) {
                macros::CAM_ZOOM_OUT(agent);
            }
        }
        frame(agent.lua_state_agent, 60.0);
        if macros::is_excute(agent) {
            ArticleModule::shoot(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_ANCIENT_BOWARROW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
        frame(agent.lua_state_agent, 70.0);
        macros::FT_MOTION_RATE(agent, 0.85);
    }
    else {
        if macros::is_excute(agent) {
            macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
            macros::CHECK_VALID_FINAL_START_CAMERA(agent, 0, 7, 20, 0, 0, 0);
            macros::SLOW_OPPONENT(agent, 30.0, 100.0);
        }
        frame(agent.lua_state_agent, 10.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
            if macros::is_excute(agent) {
                macros::FT_SET_FINAL_FEAR_FACE(agent, 60);
                macros::REQ_FINAL_START_CAMERA(agent, Hash40::new("d04final.nuanmb"), false);
                macros::FT_START_CUTIN(agent);
            }
        }
        else {
            if macros::is_excute(agent) {
                macros::CAM_ZOOM_IN_arg5(agent, 1.8, 0.0, 3.0*scale, 0.0, 0.0);
                macros::FT_START_CUTIN(agent);
                camera!(agent, *MA_MSC_CMD_CAMERA_CAM_RECT, 0, -5, -2, 0);
            }
        }
        frame(agent.lua_state_agent, 30.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
            if macros::is_excute(agent) {
                macros::CAM_ZOOM_OUT(agent);
            }
        }
        frame(agent.lua_state_agent, 60.0);
        if macros::is_excute(agent) {
            ArticleModule::shoot(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_ANCIENT_BOWARROW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
        frame(agent.lua_state_agent, 70.0);
        macros::FT_MOTION_RATE(agent, 0.85);
    }
}

//Ancient Arrow Stick ACMD
unsafe extern "C" fn ssbexo_link_ancient_arrow_stick_acmd(agent: &mut L2CAgentBase) {
    let scale = PostureModule::scale(agent.module_accessor);
    let object_id = WorkModule::get_int64(agent.module_accessor, *WEAPON_LINK_ANCIENTBOWARROW_INSTANCE_WORK_ID_INT_HIT_OBJECT_ID) as u32;
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_CHARGE) {
        if macros::is_excute(agent) {
            macros::CAM_ZOOM_OUT_FINAL(agent);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 35.0, 60, 46, 0, 102, 25.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
            AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
        }
        if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            if macros::is_excute(agent) {
                macros::CAM_ZOOM_IN_FINAL_arg13(agent, 3.0, 0.0, 2.0*scale, 1, 0, -1, -10, 10, true, object_id, 0, 10, 0);
            }
        }
        else {
            if macros::is_excute(agent) {
                macros::CAM_ZOOM_IN_FINAL_arg13(agent, 3.0, 0.0, 2.0, -1, 0, 1, -10, 10, true, object_id, 0, -10, 0);
            }
        }
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            macros::CAM_ZOOM_OUT_FINAL(agent);
        }
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::CAM_ZOOM_OUT_FINAL(agent);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 35.0, 60, 46, 0, 102, 25.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
            AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
        }
        if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            if macros::is_excute(agent) {
                macros::CAM_ZOOM_IN_FINAL_arg13(agent, 3.0, 0.0, 2.0*scale, 1, 0, -1, -10, 10, true, object_id, 0, 10, 0);
            }
        }
        else {
            if macros::is_excute(agent) {
                macros::CAM_ZOOM_IN_FINAL_arg13(agent, 3.0, 0.0, 2.0, -1, 0, 1, -10, 10, true, object_id, 0, -10, 0);
            }
        }
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            macros::CAM_ZOOM_OUT_FINAL(agent);
        }
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
        }
    }
}

pub fn install() {
    Agent::new("link")
    .sound_acmd("sound_attachwall", ssbexo_link_attachwall_sound, Priority::Low)
    .expression_acmd("expression_attachwall", ssbexo_link_attachwall_expression, Priority::Low)
    .sound_acmd("sound_attachwallclimb", ssbexo_link_attachwallclimb_sound, Priority::Low)
    .expression_acmd("expression_attachwallclimb", ssbexo_link_attachwallclimb_expression, Priority::Low)
    .game_acmd("game_final", ssbexo_link_final_smash_acmd, Priority::Low)
    .game_acmd("game_finalair", ssbexo_link_final_smash_acmd, Priority::Low)
    .install()
    ;
    Agent::new("link_ancientbowarrow")    
    .game_acmd("game_stick", ssbexo_link_ancient_arrow_stick_acmd, Priority::Low)
    .install()
    ;
}