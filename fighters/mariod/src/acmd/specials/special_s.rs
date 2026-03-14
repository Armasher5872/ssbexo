use super::*;

//Grounded Side Special ACMD
unsafe extern "C" fn ssbexo_mariod_grounded_side_special_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, false, -1);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        SEARCH(agent, 0, 0, Hash40::new("top"), 8.0, 0.0, 6.5, 2.5, Some(0.0), Some(6.5), Some(8.0), *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        WorkModule::set_float(agent.module_accessor, 9.0, *FIGHTER_MARIOD_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIOD_REFLECTOR_KIND_DRMANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 361, 110, 0, 20, 4.5, 0.0, 13.0, 8.0, Some(0.0), Some(1.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 361, 110, 0, 20, 5.0, 0.0, 6.7, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIOD_REFLECTOR_KIND_DRMANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
    frame(agent.lua_state_agent, 45.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Grounded Side Special Effect
unsafe extern "C" fn ssbexo_mariod_grounded_side_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_down_smoke"), Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    if sv_animcmd::get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_supermant_wind_r"), Hash40::new("mariod_supermant_wind_l"), Hash40::new("top"), 1, 7.5, 5, 0, 35, 0, 1.2, true, *EF_FLIP_NONE);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_supermant_wind_r"), Hash40::new("mariod_supermant_wind_l"), Hash40::new("top"), -1, 7.5, 5, 0, -35, 0, 1.2, true, *EF_FLIP_NONE);
        }
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("mariod_supermant_flash"), Hash40::new("top"), 0, 5.5, 7.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("top"), 1.0, 5.5, 7.5, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("top"), 1.0, 8.5, 7.5, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("top"), 1.0, 2.5, 7.5, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
    }
}

//Aerial Side Special ACMD
unsafe extern "C" fn ssbexo_mariod_aerial_side_special_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, false, -1);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        SEARCH(agent, 0, 0, Hash40::new("top"), 8.0, 0.0, 6.5, 2.5, Some(0.0), Some(6.5), Some(8.0), *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        WorkModule::set_float(agent.module_accessor, 9.0, *FIGHTER_MARIOD_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIOD_REFLECTOR_KIND_DRMANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        //WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARIOD_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 361, 110, 0, 20, 4.5, 0.0, 13.0, 8.0, Some(0.0), Some(1.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 361, 110, 0, 20, 5.0, 0.0, 6.7, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        //WorkModule::off_flag(agent.module_accessor, *FIGHTER_MARIOD_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIOD_REFLECTOR_KIND_DRMANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
    frame(agent.lua_state_agent, 45.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Aerial Side Special Effect
unsafe extern "C" fn ssbexo_mariod_aerial_side_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if sv_animcmd::get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_supermant_wind_r"), Hash40::new("mariod_supermant_wind_l"), Hash40::new("top"), 1, 7.5, 5, 0, 35, 0, 1.2, true, *EF_FLIP_NONE);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_supermant_wind_r"), Hash40::new("mariod_supermant_wind_l"), Hash40::new("top"), -1, 7.5, 5, 0, -35, 0, 1.2, true, *EF_FLIP_NONE);
        }
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("mariod_supermant_flash"), Hash40::new("top"), 0, 5.5, 7.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("top"), 1.0, 5.5, 7.5, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("top"), 1.0, 8.5, 7.5, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("top"), 1.0, 2.5, 7.5, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("mariod_supermant_wind_r"), -1);
        EFFECT_DETACH_KIND(agent, Hash40::new("mariod_supermant_wind_l"), -1);
    }
}

//Side Special Sound
unsafe extern "C" fn ssbexo_mariod_side_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mariod_special_s01"));
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark2"));
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_spirits_floor_elec_spark1"));
    }
}

pub fn install() {
    Agent::new("mariod")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specials", ssbexo_mariod_grounded_side_special_acmd, Low)
    .game_acmd("game_specialairs", ssbexo_mariod_aerial_side_special_acmd, Low)
    .effect_acmd("effect_specials", ssbexo_mariod_grounded_side_special_effect, Low)
    .effect_acmd("effect_specialairs", ssbexo_mariod_aerial_side_special_effect, Low)
    .sound_acmd("sound_specials", ssbexo_mariod_side_special_sound, Low)
    .sound_acmd("sound_specialairs", ssbexo_mariod_side_special_sound, Low)
    .install()
    ;
}