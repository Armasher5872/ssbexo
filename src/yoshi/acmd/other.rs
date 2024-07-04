use super::*;

//Aerial Jump Forward ACMD
unsafe extern "C" fn ssbexo_yoshi_aerial_jump_forward_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    MotionModule::set_rate(agent.module_accessor, 0.75);
    frame(agent.lua_state_agent, 26.0);
    MotionModule::set_rate(agent.module_accessor, 1.0);
}

//Aerial Jump Backward ACMD
unsafe extern "C" fn ssbexo_yoshi_aerial_jump_backward_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    MotionModule::set_rate(agent.module_accessor, 0.75);
    frame(agent.lua_state_agent, 22.0);
    MotionModule::set_rate(agent.module_accessor, 1.0);
}

//Final Smash Start ACMD
unsafe extern "C" fn ssbexo_yoshi_final_smash_start_acmd(agent: &mut L2CAgentBase) {
    let scale = PostureModule::scale(agent.module_accessor);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_CHARGE) {
        if macros::is_excute(agent) {
            macros::CHECK_VALID_FINAL_START_CAMERA(agent, 0, 7, 20, 0, 0, 0);
        }
        frame(agent.lua_state_agent, 10.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
            if macros::is_excute(agent) {
                macros::FT_SET_FINAL_FEAR_FACE(agent, 60);
                macros::REQ_FINAL_START_CAMERA(agent, Hash40::new("d04finalstart.nuanmb"), false);
            }
        }
        else {
            if macros::is_excute(agent) {
                macros::CAM_ZOOM_IN_arg5(agent, 2.0, 0.0, 3.0*scale, 0.0, 0.0);
                macros::FT_START_CUTIN(agent);
            }
        }
        frame(agent.lua_state_agent, 25.0);
        if macros::is_excute(agent) {
            macros::CAM_ZOOM_OUT(agent);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::CHECK_VALID_FINAL_START_CAMERA(agent, 0, 7, 20, 0, 0, 0);
            macros::SLOW_OPPONENT(agent, 12.0, 44.0);
        }
        frame(agent.lua_state_agent, 10.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
            if macros::is_excute(agent) {
                macros::FT_SET_FINAL_FEAR_FACE(agent, 60);
                macros::REQ_FINAL_START_CAMERA(agent, Hash40::new("d04finalstart.nuanmb"), false);
            }
        }
        else {
            if macros::is_excute(agent) {
                macros::CAM_ZOOM_IN_arg5(agent, 2.0, 0.0, 3.0*scale, 0.0, 0.0);
                macros::FT_START_CUTIN(agent);
            }
        }
        frame(agent.lua_state_agent, 25.0);
        if macros::is_excute(agent) {
            macros::CAM_ZOOM_OUT(agent);
        }
    }
}

//Final Smash Dash ACMD
unsafe extern "C" fn ssbexo_yoshi_final_smash_dash_acmd(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_CHARGE) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 361, 130, 50, 0, 8.0, 0.0, 8.0, 10.0, Some(0.0), Some(8.0), Some(16.0), 0.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        }
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 361, 130, 50, 0, 8.0, 0.0, 8.0, 10.0, Some(0.0), Some(8.0), Some(16.0), 0.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        }
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

pub fn install() {
    Agent::new("yoshi")
    .game_acmd("game_jumpaerialfront", ssbexo_yoshi_aerial_jump_forward_acmd, Priority::Low)
    .game_acmd("game_jumpaerialback", ssbexo_yoshi_aerial_jump_backward_acmd, Priority::Low)
    .game_acmd("game_finalstart", ssbexo_yoshi_final_smash_start_acmd, Priority::Low)
    .game_acmd("game_finalairstart", ssbexo_yoshi_final_smash_start_acmd, Priority::Low)
    .game_acmd("game_finaldash", ssbexo_yoshi_final_smash_dash_acmd, Priority::Low)
    .install()
    ;
}