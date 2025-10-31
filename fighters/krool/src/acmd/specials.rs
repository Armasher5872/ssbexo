use super::*;

unsafe extern "C" fn ssbexo_krool_down_special_start_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, false, -1);
        if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, Hash40::new("special_lw_start"), false, -1.0);
        }
    }
}

unsafe extern "C" fn ssbexo_krool_down_special_start_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn ssbexo_krool_down_special_start_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_krool_special_n08"));
    }
}

unsafe extern "C" fn ssbexo_krool_down_special_start_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
}

unsafe extern "C" fn ssbexo_krool_down_special_charge_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, Hash40::new("special_lw_charge"), false, -1.0);
            ArticleModule::set_rate(agent.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, 1.25);
        }
        MotionModule::set_rate(agent.module_accessor, 1.25);
    }
}

unsafe extern "C" fn ssbexo_krool_down_special_charge_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 45.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 55.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn ssbexo_krool_down_special_charge_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_krool_special_n01"));
    }
}

unsafe extern "C" fn ssbexo_krool_down_special_charge_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn ssbexo_krool_grounded_down_special_launch_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, Hash40::new("special_lw_launch"), false, -1.0);
        }
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        let launch_angle = WorkModule::get_int64(agent.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_LW_ANGLE) as u64;
        let charge = WorkModule::get_float(agent.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_CHARGE);
        let damage = 6.0*(1.0+charge);
        let sound = if charge > 1.0 {*ATTACK_SOUND_LEVEL_L} else {*ATTACK_SOUND_LEVEL_M};
        ATTACK(agent, 0, 0, Hash40::new("top"), damage, launch_angle, 40, 0, 80, 10.0, 0.0, 3.0, -13.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), sound, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 45.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn ssbexo_krool_aerial_down_special_launch_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, Hash40::new("special_air_lw_launch"), false, -1.0);
        }
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        let launch_angle = WorkModule::get_int64(agent.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_LW_ANGLE) as u64;
        let charge = WorkModule::get_float(agent.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_CHARGE);
        let damage = 6.0*(1.0+charge);
        let sound = if charge > 1.0 {*ATTACK_SOUND_LEVEL_L} else {*ATTACK_SOUND_LEVEL_M};
        ATTACK(agent, 0, 0, Hash40::new("top"), damage, launch_angle, 40, 0, 80, 8.0, 0.0, 3.0, -16.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), sound, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 45.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn ssbexo_krool_down_special_launch_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_bomb_b"), Hash40::new("havel"), 0, 0, 0, 90, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("krool_cannon_shot"), Hash40::new("havel"), 0, 0, 0, 90, 0, 0, 1.3, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("krool_cannon_shot"), -1);
    }
}

unsafe extern "C" fn ssbexo_krool_down_special_launch_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_krool_special_n11"));
    }
}

unsafe extern "C" fn ssbexo_krool_down_special_launch_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

pub fn install() {
    Agent::new("krool")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallwstart", ssbexo_krool_down_special_start_acmd, Low)
    .effect_acmd("effect_speciallwstart", ssbexo_krool_down_special_start_effect, Low)
    .sound_acmd("sound_speciallwstart", ssbexo_krool_down_special_start_sound, Low)
    .expression_acmd("expression_speciallwstart", ssbexo_krool_down_special_start_expression, Low)
    .game_acmd("game_specialairlwstart", ssbexo_krool_down_special_start_acmd, Low)
    .effect_acmd("effect_specialairlwstart", ssbexo_krool_down_special_start_effect, Low)
    .sound_acmd("sound_specialairlwstart", ssbexo_krool_down_special_start_sound, Low)
    .expression_acmd("expression_specialairlwstart", ssbexo_krool_down_special_start_expression, Low)
    .game_acmd("game_speciallwcharge", ssbexo_krool_down_special_charge_acmd, Low)
    .effect_acmd("effect_speciallwcharge", ssbexo_krool_down_special_charge_effect, Low)
    .sound_acmd("sound_speciallwcharge", ssbexo_krool_down_special_charge_sound, Low)
    .expression_acmd("expression_speciallwcharge", ssbexo_krool_down_special_charge_expression, Low)
    .game_acmd("game_specialairlwcharge", ssbexo_krool_down_special_charge_acmd, Low)
    .effect_acmd("effect_specialairlwcharge", ssbexo_krool_down_special_charge_effect, Low)
    .sound_acmd("sound_specialairlwcharge", ssbexo_krool_down_special_charge_sound, Low)
    .expression_acmd("expression_specialairlwcharge", ssbexo_krool_down_special_charge_expression, Low)
    .game_acmd("game_speciallwlaunch", ssbexo_krool_grounded_down_special_launch_acmd, Low)
    .effect_acmd("effect_speciallwlaunch", ssbexo_krool_down_special_launch_effect, Low)
    .sound_acmd("sound_speciallwlaunch", ssbexo_krool_down_special_launch_sound, Low)
    .expression_acmd("expression_speciallwlaunch", ssbexo_krool_down_special_launch_expression, Low)
    .game_acmd("game_specialairlwlaunch", ssbexo_krool_aerial_down_special_launch_acmd, Low)
    .effect_acmd("effect_specialairlwlaunch", ssbexo_krool_down_special_launch_effect, Low)
    .sound_acmd("sound_specialairlwlaunch", ssbexo_krool_down_special_launch_sound, Low)
    .expression_acmd("expression_specialairlwlaunch", ssbexo_krool_down_special_launch_expression, Low)
    .install()
    ;
}