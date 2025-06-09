use super::*;

//Back Dash ACMD
unsafe extern "C" fn ssbexo_donkey_turn_dash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("footl"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("footr"), *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

//Final Smash Start ACMD
unsafe extern "C" fn ssbexo_donkey_final_smash_start_acmd(agent: &mut L2CAgentBase) {
    let scale = PostureModule::scale(agent.module_accessor);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_CHARGE) {
        if is_excute(agent) {
            CHECK_VALID_FINAL_START_CAMERA(agent, 0, 7, 20, 0, 0, 0);
        }
        frame(agent.lua_state_agent, 5.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
            if is_excute(agent) {
                FT_SET_FINAL_FEAR_FACE(agent, 20);
                REQ_FINAL_START_CAMERA(agent, Hash40::new("d04finalstart.nuanmb"), false);
                FT_START_CUTIN(agent);
            }
        }
        else {
            if is_excute(agent) {
                CAM_ZOOM_IN_arg5(agent, 2.0, 0.0, 5.0*scale, 0.0, 0.0);
                FT_START_CUTIN(agent);
            }
        }
        frame(agent.lua_state_agent, 25.0);
        if is_excute(agent) {
            CAM_ZOOM_OUT(agent);
        }
        frame(agent.lua_state_agent, 29.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 361, 0, 0, 0, 13.0, 0.0, 10.0, 8.0, Some(0.0), Some(10.0), Some(28.0), 5.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
            AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        }
        frame(agent.lua_state_agent, 34.0);
        if is_excute(agent) {
            CAM_ZOOM_OUT(agent);
        }
        frame(agent.lua_state_agent, 35.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 37.0);
        if is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_DONKEY_STATUS_FINAL_FLAG_TO_ATTACK);
        }
    }
    else {
        if is_excute(agent) {
            WHOLE_HIT(agent, *HIT_STATUS_XLU);
            CHECK_VALID_FINAL_START_CAMERA(agent, 0, 7, 20, 0, 0, 0);
            SLOW_OPPONENT(agent, 7.0, 30.0);
        }
        frame(agent.lua_state_agent, 5.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
            if is_excute(agent) {
                FT_SET_FINAL_FEAR_FACE(agent, 20);
                REQ_FINAL_START_CAMERA(agent, Hash40::new("d04finalstart.nuanmb"), false);
                FT_START_CUTIN(agent);
            }
        }
        else {
            if is_excute(agent) {
                CAM_ZOOM_IN_arg5(agent, 2.0, 0.0, 5.0*scale, 0.0, 0.0);
                FT_START_CUTIN(agent);
            }
        }
        frame(agent.lua_state_agent, 15.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
            if is_excute(agent) {
                SlowModule::set_whole(agent.module_accessor, 30, 0);
            }
        }
        frame(agent.lua_state_agent, 16.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
            if is_excute(agent) {
                SlowModule::clear_whole(agent.module_accessor);
            }
        }
        frame(agent.lua_state_agent, 24.0);
        if is_excute(agent) {
            SlowModule::clear_whole(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 25.0);
        if is_excute(agent) {
            CAM_ZOOM_OUT(agent);
        }
        frame(agent.lua_state_agent, 29.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 361, 0, 0, 0, 13.0, 0.0, 10.0, 8.0, Some(0.0), Some(10.0), Some(28.0), 5.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
            AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        }
        frame(agent.lua_state_agent, 34.0);
        if is_excute(agent) {
            CAM_ZOOM_OUT(agent);
        }
        frame(agent.lua_state_agent, 35.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 37.0);
        if is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_DONKEY_STATUS_FINAL_FLAG_TO_ATTACK);
        }
    }
}

pub fn install() {
    Agent::new("donkey")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_turndash", ssbexo_donkey_turn_dash_acmd, Low)
    .game_acmd("game_finalstart", ssbexo_donkey_final_smash_start_acmd, Low)
    .game_acmd("game_finalairstart", ssbexo_donkey_final_smash_start_acmd, Low)
    .install()
    ;
}