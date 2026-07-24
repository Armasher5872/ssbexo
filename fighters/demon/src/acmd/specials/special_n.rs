use super::*;

//Grounded Neutral Special Start ACMD
unsafe extern "C" fn ssbexo_demon_grounded_neutral_special_start_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, true, 10.0);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 11.0);
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, true, 6.0);
    }
    frame(lua_state, 17.0);
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, true, 5.0);
    }
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 19.0);
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, true, 4.0);
    }
    frame(lua_state, 23.0);
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, true, 3.0);
    }
}

//Aerial Neutral Special Start ACMD
unsafe extern "C" fn ssbexo_demon_aerial_neutral_special_start_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, true, 10.0);
    }
    frame(lua_state, 4.0);
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, true, 6.0);
    }
    frame(lua_state, 6.0);
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, true, 5.0);
    }
    frame(lua_state, 9.0);
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, true, 4.0);
    }
    frame(lua_state, 12.0);
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, true, 3.0);
    }
}

//Grounded Neutral Special Shoot ACMD
unsafe extern "C" fn ssbexo_demon_grounded_neutral_special_shoot_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let hold_frame = WorkModule::get_int(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_INT_SPECIAL_N_HOLD_FRAME);
    if hold_frame >= 14 {
        if is_excute(agent) {
            if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
                FighterSpecializer_Demon::set_devil(boma, true, 2.0);
            }
            FT_MOTION_RATE(agent, 20.0/11.0);
        }
    }
    else {
        if is_excute(agent) {
            if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
                FighterSpecializer_Demon::set_devil(boma, true, 2.0);
            }
        }
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        ArticleModule::generate_article(boma, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, false, -1);
        ArticleModule::shoot_exist(boma, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_SCALING);
        WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_SCALING);
        WorkModule::off_flag(boma, *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(lua_state, 60.0);
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, true, 3.0);
    }
    frame(lua_state, 64.0);
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, true, 5.0);
    }
    frame(lua_state, 65.0);
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, true, 7.0);
    }
    frame(lua_state, 66.0);
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, true, 8.0);
    }
    frame(lua_state, 67.0);
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, false, 0.0);
    }
}

//Aerial Neutral Special Shoot ACMD
unsafe extern "C" fn ssbexo_demon_aerial_neutral_special_shoot_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let hold_frame = WorkModule::get_int(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_INT_SPECIAL_N_HOLD_FRAME);
    if hold_frame >= 14 {
        if is_excute(agent) {
            if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
                FighterSpecializer_Demon::set_devil(boma, true, 2.0);
            }
            FT_MOTION_RATE(agent, 20.0/14.0);
        }
    }
    else {
        if is_excute(agent) {
            if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
                FighterSpecializer_Demon::set_devil(boma, true, 2.0);
            }
        }
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        ArticleModule::generate_article(boma, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, false, -1);
        ArticleModule::shoot_exist(boma, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(lua_state, 43.0);
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, true, 3.0);
    }
    frame(lua_state, 46.0);
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, true, 4.0);
    }
    frame(lua_state, 50.0);
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, true, 5.0);
    }
    frame(lua_state, 54.0);
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, true, 6.0);
    }
    frame(lua_state, 56.0);
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, true, 7.0);
    }
    frame(lua_state, 58.0);
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, true, 8.0);
    }
    frame(lua_state, 60.0);
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
        FighterSpecializer_Demon::set_devil(boma, false, 0.0);
    }
}

//Laser Fly ACMD
unsafe extern "C" fn ssbexo_demon_laser_fly_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let owner_boma = get_owner_boma(agent);
    let hold_frame = WorkModule::get_int(owner_boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_INT_SPECIAL_N_HOLD_FRAME);
    if hold_frame >= 14 {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 60, 35, 0, 80, 1.0, 0.0, 0.0, 0.0, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 50, 35, 0, 80, 7.0, 0.0, -3.0, 6.0, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.1);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.1);
            AttackModule::set_add_reaction_frame_revised(boma, 1, 0.1, false);
        }
        frame(lua_state, 1.0);
        if is_excute(agent) {
            AttackModule::clear(boma, 1, false);
            ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 60, 35, 0, 80, 2.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-10.0), 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            AttackModule::set_add_reaction_frame_revised(boma, 0, 10.0, false);
        }
    }
    else {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 60, 35, 0, 80, 1.0, 0.0, 0.0, 0.0, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 50, 35, 0, 80, 7.0, 0.0, -3.0, 6.0, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.1);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.1);
            AttackModule::set_add_reaction_frame_revised(boma, 1, 0.1, false);
        }
        frame(lua_state, 1.0);
        if is_excute(agent) {
            AttackModule::clear(boma, 1, false);
            ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 60, 35, 0, 80, 2.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-10.0), 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            AttackModule::set_add_reaction_frame_revised(boma, 0, 10.0, false);
        }
    }
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialnstart", ssbexo_demon_grounded_neutral_special_start_acmd, Low)
    .acmd("game_specialairnstart", ssbexo_demon_aerial_neutral_special_start_acmd, Low)
    .acmd("game_specialn", ssbexo_demon_grounded_neutral_special_shoot_acmd, Low)
    .acmd("game_specialnhi", ssbexo_demon_grounded_neutral_special_shoot_acmd, Low)
    .acmd("game_specialnlw", ssbexo_demon_grounded_neutral_special_shoot_acmd, Low)
    .acmd("game_specialairn", ssbexo_demon_aerial_neutral_special_shoot_acmd, Low)
    .acmd("game_specialairnhi", ssbexo_demon_aerial_neutral_special_shoot_acmd, Low)
    .acmd("game_specialairnlw", ssbexo_demon_aerial_neutral_special_shoot_acmd, Low)
    .install()
    ;
    Agent::new("demon_blaster")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_flyn", ssbexo_demon_laser_fly_acmd, Low)
    .acmd("game_flyhi", ssbexo_demon_laser_fly_acmd, Low)
    .acmd("game_flylw", ssbexo_demon_laser_fly_acmd, Low)
    .acmd("game_flyairn", ssbexo_demon_laser_fly_acmd, Low)
    .acmd("game_flyairhi", ssbexo_demon_laser_fly_acmd, Low)
    .acmd("game_flyairlw", ssbexo_demon_laser_fly_acmd, Low)
    .install()
    ;
}