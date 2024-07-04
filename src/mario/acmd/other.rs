use super::*;

//Final Smash ACMD
unsafe extern "C" fn ssbexo_mario_final_smash_acmd(agent: &mut L2CAgentBase) {
    let scale = PostureModule::scale(agent.module_accessor);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_CHARGE) {
        if macros::is_excute(agent) {
            macros::CHECK_VALID_FINAL_START_CAMERA(agent, 0, 1, 20, 0, 0, 0);
        }
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_HUGE_FLAME, false, -1);
        }
        frame(agent.lua_state_agent, 10.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {      
            if macros::is_excute(agent) {
                macros::FT_SET_FINAL_FEAR_FACE(agent, 60);
                macros::REQ_FINAL_START_CAMERA_arg3(agent, Hash40::new("d04final.nuanmb"), false, false);
                macros::FT_START_CUTIN(agent);
            }  
        }
        else {
            if macros::is_excute(agent) {
                macros::CAM_ZOOM_IN_arg5(agent, 2.05, 0.0, 3.0*scale, 0.0, 0.0);
                macros::FT_START_CUTIN(agent);
            }
        }
        if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            if macros::is_excute(agent) {
                camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
            }
        }
        else {
            if macros::is_excute(agent) {
                camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
            }
        }
        frame(agent.lua_state_agent, 25.0);
        if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
            macros::CAM_ZOOM_OUT(agent);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
            macros::CHECK_VALID_FINAL_START_CAMERA(agent, 0, 1, 20, 0, 0, 0);
            macros::SLOW_OPPONENT(agent, 20.0, 75.0);
        }
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_HUGE_FLAME, false, -1);
        }
        frame(agent.lua_state_agent, 10.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {      
            if macros::is_excute(agent) {
                macros::FT_SET_FINAL_FEAR_FACE(agent, 60);
                macros::REQ_FINAL_START_CAMERA_arg3(agent, Hash40::new("d04final.nuanmb"), false, false);
                macros::FT_START_CUTIN(agent);
            }  
        }
        else {
            if macros::is_excute(agent) {
                macros::CAM_ZOOM_IN_arg5(agent, 2.05, 0.0, 3.0*scale, 0.0, 0.0);
                macros::FT_START_CUTIN(agent);
            }
        }
        if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            if macros::is_excute(agent) {
                camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
            }
        }
        else {
            if macros::is_excute(agent) {
                camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
            }
        }
        frame(agent.lua_state_agent, 25.0);
        if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
            macros::CAM_ZOOM_OUT(agent);
        }
    }
}

//Finale Fireball Regular ACMD
unsafe extern "C" fn ssbexo_mario_finale_fireball_regular_acmd(agent: &mut L2CAgentBase) {
    let owner_boma = get_owner_boma(agent);
    if WorkModule::is_flag(owner_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_CHARGE) {
        frame(agent.lua_state_agent, 60.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("fire1"), 2.5, 50, 40, 0, 25, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 1, 0, Hash40::new("fire2"), 2.5, 48, 40, 0, 25, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 100.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("fire1"), 3.0, 23, 90, 0, 20, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 1, 0, Hash40::new("fire2"), 3.0, 20, 90, 0, 20, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 2, 0, Hash40::new("fire1"), 3.0, 50, 100, 0, 15, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 3, 0, Hash40::new("fire2"), 3.0, 48, 100, 0, 15, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 150.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("fire1"), 3.5, 23, 120, 0, 10, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 1, 0, Hash40::new("fire2"), 3.5, 20, 120, 0, 10, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 2, 0, Hash40::new("fire1"), 3.5, 50, 130, 0, 5, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 3, 0, Hash40::new("fire2"), 3.5, 48, 130, 0, 5, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 220.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    else {
        frame(agent.lua_state_agent, 60.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("fire1"), 2.5, 50, 40, 0, 25, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 1, 0, Hash40::new("fire2"), 2.5, 48, 40, 0, 25, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 100.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("fire1"), 3.0, 23, 90, 0, 20, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 1, 0, Hash40::new("fire2"), 3.0, 20, 90, 0, 20, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 2, 0, Hash40::new("fire1"), 3.0, 50, 100, 0, 15, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 3, 0, Hash40::new("fire2"), 3.0, 48, 100, 0, 15, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 150.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("fire1"), 3.5, 23, 120, 0, 10, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 1, 0, Hash40::new("fire2"), 3.5, 20, 120, 0, 10, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 2, 0, Hash40::new("fire1"), 3.5, 50, 130, 0, 5, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 3, 0, Hash40::new("fire2"), 3.5, 48, 130, 0, 5, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 220.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

pub fn install() {
    Agent::new("mario")
    .game_acmd("game_final", ssbexo_mario_final_smash_acmd, Priority::Low)
    .game_acmd("game_finalair", ssbexo_mario_final_smash_acmd, Priority::Low)
    .install()
    ;
    Agent::new("mario_hugeflame")
    .game_acmd("game_regular", ssbexo_mario_finale_fireball_regular_acmd, Priority::Low)
    .install()
    ;
}