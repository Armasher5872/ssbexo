use super::*;

//Final Smash Start ACMD
unsafe extern "C" fn ssbexo_samus_final_smash_start_acmd(agent: &mut L2CAgentBase) {
    let scale = PostureModule::scale(agent.module_accessor);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_CHARGE) {
        if is_excute(agent) {
            CHECK_VALID_FINAL_START_CAMERA(agent, 0, 7, 29, 0, 0, 0);
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_LASER, false, -1);
        }
        frame(agent.lua_state_agent, 10.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
            if is_excute(agent) {
                FT_SET_FINAL_FEAR_FACE(agent, 60);
                REQ_FINAL_START_CAMERA_arg3(agent, Hash40::new("d04finalstart.nuanmb"), false, false);
            }
            else {
                if is_excute(agent) {
                    CAM_ZOOM_IN_arg5(agent, 1.7, 0.0, 3.0*scale, 0.0, 0.0);
                    FT_START_CUTIN(agent);
                }
            }
        }
        frame(agent.lua_state_agent, 25.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
            if is_excute(agent) {
                CAM_ZOOM_OUT(agent);
            }
        }
        frame(agent.lua_state_agent, 48.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 18, 100, 12, 0, 10.0, 0.0, 13.0, 17.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 18, 100, 30, 0, 20.0, 0.0, 13.0, -15.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 10, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.0, 366, 100, 30, 0, 30.0, 0.0, 13.0, 17.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 10, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 69.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    else {
        if is_excute(agent) {
            CHECK_VALID_FINAL_START_CAMERA(agent, 0, 7, 29, 0, 0, 0);
            SLOW_OPPONENT(agent, 5.0, 50.0);
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_LASER, false, -1);
        }
        frame(agent.lua_state_agent, 10.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
            if is_excute(agent) {
                FT_SET_FINAL_FEAR_FACE(agent, 60);
                REQ_FINAL_START_CAMERA_arg3(agent, Hash40::new("d04finalstart.nuanmb"), false, false);
            }
            else {
                if is_excute(agent) {
                    CAM_ZOOM_IN_arg5(agent, 1.7, 0.0, 3.0*scale, 0.0, 0.0);
                    FT_START_CUTIN(agent);
                }
            }
        }
        frame(agent.lua_state_agent, 25.0);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
            if is_excute(agent) {
                CAM_ZOOM_OUT(agent);
            }
        }
        frame(agent.lua_state_agent, 48.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 18, 100, 12, 0, 10.0, 0.0, 13.0, 17.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 18, 100, 30, 0, 20.0, 0.0, 13.0, -15.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 10, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.0, 366, 100, 30, 0, 30.0, 0.0, 13.0, 17.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 10, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 69.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

//Final Smash ACMD
unsafe extern "C" fn ssbexo_samus_final_smash_acmd(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_CHARGE) {
        if !IS_EXIST_ARTICLE(agent, *FIGHTER_SAMUS_GENERATE_ARTICLE_LASER2) {
            if is_excute(agent) {
                ArticleModule::shoot(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_LASER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
                WorkModule::set_float(agent.module_accessor, 14.0, *FIGHTER_SAMUS_STATUS_FINAL_WORK_FLOAT_OY);
                ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_LASER2, false, -1);
                WorkModule::set_float(agent.module_accessor, -14.0, *FIGHTER_SAMUS_STATUS_FINAL_WORK_FLOAT_OY);
                ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_LASER2, false, -1);
                ArticleModule::shoot(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_LASER2, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
            }
        }
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 20, 100, 22, 0, 13.0, 0.0, 15.0, 20.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 8.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 20, 100, 22, 0, 15.0, 0.0, 15.0, 20.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 12.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    else {
        if !IS_EXIST_ARTICLE(agent, *FIGHTER_SAMUS_GENERATE_ARTICLE_LASER2) {
            if is_excute(agent) {
                ArticleModule::shoot(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_LASER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
                WorkModule::set_float(agent.module_accessor, 14.0, *FIGHTER_SAMUS_STATUS_FINAL_WORK_FLOAT_OY);
                ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_LASER2, false, -1);
                WorkModule::set_float(agent.module_accessor, -14.0, *FIGHTER_SAMUS_STATUS_FINAL_WORK_FLOAT_OY);
                ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_LASER2, false, -1);
                ArticleModule::shoot(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_LASER2, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
            }
        }
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 20, 100, 22, 0, 13.0, 0.0, 15.0, 20.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 8.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 20, 100, 22, 0, 15.0, 0.0, 15.0, 20.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 12.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

//Laser Shoot ACMD
unsafe extern "C" fn ssbexo_samus_laser_shoot_acmd(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_CHARGE) {
        wait(agent.lua_state_agent, 12.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 20, 100, 16, 0, 5.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 10, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
            ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 0, 100, 5, 0, 4.0, 0.0, -1.0, -10.0, Some(0.0), Some(-1.8), Some(-15.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 10, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        wait(agent.lua_state_agent, 128.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            ATTACK(agent, 0, 0, Hash40::new("top"), 5.5, 361, 183, 0, 45, 11.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
            AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
            AttackModule::set_final_finish_cut_in(agent.module_accessor, 0, true, true, -1.0, false);
        }
        wait(agent.lua_state_agent, 8.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    else {
        wait(agent.lua_state_agent, 12.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 20, 100, 16, 0, 5.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 10, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
            ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 0, 100, 5, 0, 4.0, 0.0, -1.0, -10.0, Some(0.0), Some(-1.8), Some(-15.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 10, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        wait(agent.lua_state_agent, 128.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            ATTACK(agent, 0, 0, Hash40::new("top"), 5.5, 361, 183, 0, 45, 11.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
            AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
            AttackModule::set_final_finish_cut_in(agent.module_accessor, 0, true, true, -1.0, false);
        }
        wait(agent.lua_state_agent, 8.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

//Laser 2 Lower ACMD
unsafe extern "C" fn ssbexo_samus_laser_2_lower_acmd(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_CHARGE) {
        wait(agent.lua_state_agent, 17.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 45, 100, 24, 0, 6.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        }
        wait(agent.lua_state_agent, 123.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.5, 45, 183, 0, 95, 12.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
            AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
            AttackModule::set_final_finish_cut_in(agent.module_accessor, 0, true, true, -1.0, false);
        }
        wait(agent.lua_state_agent, 8.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    else {
        wait(agent.lua_state_agent, 17.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 45, 100, 24, 0, 6.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        }
        wait(agent.lua_state_agent, 123.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.5, 45, 183, 0, 95, 12.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
            AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
            AttackModule::set_final_finish_cut_in(agent.module_accessor, 0, true, true, -1.0, false);
        }
        wait(agent.lua_state_agent, 8.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

//Laser 2 Upper ACMD
unsafe extern "C" fn ssbexo_samus_laser_2_upper_acmd(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_CHARGE) {
        wait(agent.lua_state_agent, 17.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 325, 100, 20, 0, 6.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        }
        wait(agent.lua_state_agent, 123.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.5, 45, 183, 0, 95, 12.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
            AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
            AttackModule::set_final_finish_cut_in(agent.module_accessor, 0, true, true, -1.0, false);
        }
        wait(agent.lua_state_agent, 8.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    else {
        wait(agent.lua_state_agent, 17.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 325, 100, 20, 0, 6.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        }
        wait(agent.lua_state_agent, 123.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.5, 45, 183, 0, 95, 12.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
            AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
            AttackModule::set_final_finish_cut_in(agent.module_accessor, 0, true, true, -1.0, false);
        }
        wait(agent.lua_state_agent, 8.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

pub fn install() {
    Agent::new("samus")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_finalstart", ssbexo_samus_final_smash_start_acmd, Low)
    .game_acmd("game_finalairstart", ssbexo_samus_final_smash_start_acmd, Low)
    .game_acmd("game_final", ssbexo_samus_final_smash_acmd, Low)
    .game_acmd("game_finalair", ssbexo_samus_final_smash_acmd, Low)
    .install()
    ;
    Agent::new("samus_laser")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_shoot", ssbexo_samus_laser_shoot_acmd, Low)
    .install()
    ;
    Agent::new("samus_laser2")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_lower", ssbexo_samus_laser_2_lower_acmd, Low)
    .game_acmd("game_upper", ssbexo_samus_laser_2_upper_acmd, Low)
    .install()
    ;
}