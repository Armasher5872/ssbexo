use super::*;

//Grounded Side Special Start ACMD
#[acmd_script( agent = "kirby", script = "game_specialsstart", category = ACMD_GAME)]
unsafe fn ssbuexo_kirby_grounded_side_special_start_acmd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, 0);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_s"), false, 0.0);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_armfoot"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_eye1"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_facen"), false);
    }
}

//Aerial Side Special Start ACMD
#[acmd_script( agent = "kirby", script = "game_specialairsstart", category = ACMD_GAME)]
unsafe fn ssbuexo_kirby_aerial_side_special_start_acmd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, 0);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_s"), false, 0.0);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_armfoot"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_eye1"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_facen"), false);
    }
}

//Grounded Side Special Start Effect
#[acmd_script( agent = "kirby", script = "effect_specialsstart", category = ACMD_EFFECT)]
unsafe fn ssbuexo_kirby_grounded_side_special_start_effect(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("kirby_stone_s"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//Aerial Side Special Start Effect
#[acmd_script( agent = "kirby", script = "effect_specialairsstart", category = ACMD_EFFECT)]
unsafe fn ssbuexo_kirby_aerial_side_special_start_effect(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("kirby_stone_s"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//Side Special Start Sound
#[acmd_script( agent = "kirby", scripts = ["sound_specialsstart", "sound_specialairsstart"], category = ACMD_SOUND)]
unsafe fn ssbuexo_kirby_side_special_start_sound(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_kirby_landing01"));
    }
}

//Grounded Side Special Start Expression
#[acmd_script( agent = "kirby", script = "expression_specialsstart", category = ACMD_EXPRESSION)]
unsafe fn ssbuexo_kirby_grounded_side_special_start_expression(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
        macros::HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("claviclec"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("clavicler"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("claviclel"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("legc"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("toer"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("toel"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_NORMAL);
    }
}

//Aerial Side Special Start Expression
#[acmd_script( agent = "kirby", script = "expression_specialairsstart", category = ACMD_EXPRESSION)]
unsafe fn ssbuexo_kirby_aerial_side_special_start_expression(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
        macros::HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("claviclec"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("clavicler"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("claviclel"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("legc"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("toer"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("toel"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_NORMAL);
    }
}

//Side Special Hammer Effect
#[acmd_script( agent = "kirby_hammer", scripts = ["effect_specialsstart", "effect_specialairsstart", "effect_specials", "effect_specialairs"], category = ACMD_EFFECT)]
unsafe fn ssbuexo_kirby_side_special_hammer_effect(_fighter: &mut L2CAgentBase) {}

//Side Special Hold ACMD
#[acmd_script( agent = "kirby", scripts = ["game_specialshold", "game_specialsfall"], category = ACMD_GAME)]
unsafe fn ssbuexo_kirby_side_special_hold_acmd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    for _ in 0..60 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 0.3, 90, 50, 70, 0, 4.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        wait(fighter.lua_state_agent, 1.0);
    }
}

//Side Special Turn ACMD
#[acmd_script( agent = "kirby", script = "game_specialsturn", category = ACMD_GAME)]
unsafe fn ssbuexo_kirby_side_special_turn_acmd(fighter: &mut L2CAgentBase) {
    let wheel_power_up = WorkModule::get_float(fighter.module_accessor, FIGHTER_KIRBY_INSTANCE_WORK_ID_FLOAT_WHEEL_POWER_UP);
    if macros::is_excute(fighter) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_armfoot"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_eye1"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_facen"), false);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_s"), false, 0.0);
        macros::REVERSE_LR(fighter);
    }
    frame(fighter.lua_state_agent, 1.0);
    for _ in 0..60 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 0.3, 90, 50, 70, 0, 4.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        wait(fighter.lua_state_agent, 1.0);
    }
}

//Side Special Turn Sound
#[acmd_script( agent = "kirby", script = "sound_specialsturn", category = ACMD_SOUND)]
unsafe fn ssbuexo_kirby_side_special_turn_sound(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
        sound!(fighter, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
        macros::STOP_SE(fighter, Hash40::new("se_kirby_attack100"));
        macros::PLAY_SE(fighter, Hash40::new("se_kirby_attack100"));
    }
}

//Side Special Jump ACMD
#[acmd_script( agent = "kirby", script = "game_specialsjump", category = ACMD_GAME)]
unsafe fn ssbuexo_kirby_side_special_jump_acmd(fighter: &mut L2CAgentBase) {
    let wheel_power_up = WorkModule::get_float(fighter.module_accessor, FIGHTER_KIRBY_INSTANCE_WORK_ID_FLOAT_WHEEL_POWER_UP);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, 0);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_armfoot"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_eye1"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_facen"), false);
    }
}

//Side Special Jump Sound
#[acmd_script( agent = "kirby", script = "sound_specialsjump", category = ACMD_SOUND)]
unsafe fn ssbuexo_kirby_side_special_jump_sound(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
        sound!(fighter, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
        macros::STOP_SE(fighter, Hash40::new("se_kirby_attack100"));
    }
}

//Grounded Side Special Attack ACMD
#[acmd_script( agent = "kirby", script = "game_specials", category = ACMD_GAME)]
unsafe fn ssbuexo_kirby_grounded_side_special_attack_acmd(fighter: &mut L2CAgentBase) {
    let wheel_power_up = WorkModule::get_float(fighter.module_accessor, FIGHTER_KIRBY_INSTANCE_WORK_ID_FLOAT_WHEEL_POWER_UP);
    if macros::is_excute(fighter) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_armfoot"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_eye1"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_facen"), false);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_s"), false, 0.0);
        macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 10.0*wheel_power_up, 75, 60, 0, 40, 6.0, 0.0, 3.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 8.0*wheel_power_up, 75, 60, 0, 40, 6.0, 0.0, 3.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 6.0*wheel_power_up, 75, 60, 0, 40, 6.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 30.0);
	if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_armfoot"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_eye1"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_facen"), true);
    }
}

//Aerial Side Special Attack ACMD
#[acmd_script( agent = "kirby", script = "game_specialairs", category = ACMD_GAME)]
unsafe fn ssbuexo_kirby_aerial_side_special_attack_acmd(fighter: &mut L2CAgentBase) {
    let wheel_power_up = WorkModule::get_float(fighter.module_accessor, FIGHTER_KIRBY_INSTANCE_WORK_ID_FLOAT_WHEEL_POWER_UP);
    if macros::is_excute(fighter) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_armfoot"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_eye1"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_facen"), false);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_s"), false, 0.0);
        macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 10.0*wheel_power_up, 75, 60, 0, 40, 6.0, 0.0, 3.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 8.0*wheel_power_up, 75, 60, 0, 40, 6.0, 0.0, 3.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 6.0*wheel_power_up, 75, 60, 0, 40, 6.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 30.0);
	if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_armfoot"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_eye1"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_facen"), true);
    }
}

//Side Special Attack Effect
#[acmd_script( agent = "kirby", scripts = ["effect_specials", "effect_specialairs"], category = ACMD_EFFECT)]
unsafe fn ssbuexo_kirby_side_special_attack_effect(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 7.5, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 2, 6.5, 0, 0, 180, 0, 1.0, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.6, 0.7, 1.0);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.2);
    }
    frame(fighter.lua_state_agent, 30.0);
	if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_speedline"), false, true);
    }
    frame(fighter.lua_state_agent, 35.0);
	if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("kirby_stone_e"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
    }
}

//Grounded Side Special Attack Sound
#[acmd_script( agent = "kirby", script = "sound_specials", category = ACMD_SOUND)]
unsafe fn ssbuexo_kirby_grounded_side_special_attack_sound(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_kirby_special_h02"));
        macros::PLAY_SE(fighter, Hash40::new("se_kirby_attack100"));
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_kirby_008"));
    }
    frame(fighter.lua_state_agent, 33.0);
	if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_kirby_special_h02"));
    }
    frame(fighter.lua_state_agent, 35.0);
	if macros::is_excute(fighter) {
        sound!(fighter, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
        macros::STOP_SE(fighter, Hash40::new("se_kirby_attack100"));
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_kirby_rnd_special_lw"));
    }
    frame(fighter.lua_state_agent, 38.0);
	if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_kirby_special_l03"));
    }
}

//Aerial Side Special Attack Sound
#[acmd_script( agent = "kirby", script = "sound_specialairs", category = ACMD_SOUND)]
unsafe fn ssbuexo_kirby_aerial_side_special_attack_sound(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_kirby_special_h02"));
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_kirby_008"));
    }
    frame(fighter.lua_state_agent, 33.0);
	if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_kirby_special_h02"));
    }
    frame(fighter.lua_state_agent, 35.0);
	if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_kirby_rnd_special_lw"));
    }
    frame(fighter.lua_state_agent, 38.0);
	if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_kirby_special_l03"));
    }
}

//Side Special Attack Expression
#[acmd_script( agent = "kirby", scripts = ["expression_specials", "expression_specialairs"], category = ACMD_EXPRESSION)]
unsafe fn ssbuexo_kirby_side_special_attack_expression(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("claviclec"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("clavicler"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("claviclel"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("legc"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("toer"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("toel"), *HIT_STATUS_OFF);
        macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_NORMAL);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
        macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("s_stingd1"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_OFF);
        ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
    }
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_rush"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

//Grounded Up Special 1 Effect
#[acmd_script( agent = "kirby", script = "effect_specialhi", category = ACMD_EFFECT)]
unsafe fn ssbuexo_kirby_grounded_up_special_1_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("kirby_fcut_arc"), Hash40::new("kirby_fcut_arc"), Hash40::new("top"), 0, 12, 0, 0, 0, -149, 1, false, *EF_FLIP_YZ);
    }
}

//Aerial Up Special 1 Effect
#[acmd_script( agent = "kirby", script = "effect_specialairhi", category = ACMD_EFFECT)]
unsafe fn ssbuexo_kirby_aerial_up_special_1_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("kirby_fcut_arc"), Hash40::new("kirby_fcut_arc"), Hash40::new("top"), 0, 12, 0, 0, 0, -149, 1, false, *EF_FLIP_YZ);
    }
}

//Grounded Up Special 2 ACMD
#[acmd_script( agent = "kirby", script = "game_specialhi2", category = ACMD_GAME)]
unsafe fn ssbuexo_kirby_grounded_up_special_2_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi2"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 5.0, 90, 100, 100, 0, 6.0, 0.0, 8.5, 0.0, Some(0.0), Some(15.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 5.0, 90, 100, 50, 0, 6.0, 0.0, 8.5, 0.0, Some(0.0), Some(15.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Aerial Up Special 2 ACMD
#[acmd_script( agent = "kirby", script = "game_specialairhi2", category = ACMD_GAME)]
unsafe fn ssbuexo_kirby_aerial_up_special_2_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi2"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 5.0, 90, 100, 100, 0, 6.0, 0.0, 8.5, 0.0, Some(0.0), Some(15.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 5.0, 90, 100, 50, 0, 6.0, 0.0, 8.5, 0.0, Some(0.0), Some(15.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Up Special 2 Effect
#[acmd_script( agent = "kirby", scripts = ["effect_specialhi2", "effect_specialairhi2"], category = ACMD_EFFECT)]
unsafe fn ssbuexo_kirby_up_special_2_effect(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_kirby_cutter"), Hash40::new("tex_kirby_cutter"), 12, Hash40::new("haver"), 0.0, 3.0, 0.25, Hash40::new("haver"), 0.0, 15.0, 0.5, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("kirby_fcut_rise"), Hash40::new("haver"), 0, 3, 0.3, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 3);
    }
}

//Up Special 3 ACMD
#[acmd_script( agent = "kirby", script = "game_specialhi3", category = ACMD_GAME)]
unsafe fn ssbuexo_kirby_up_special_3_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi3"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 8.0, 30, 80, 0, 60, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 8.0, 30, 80, 0, 60, 6.0, 0.0, 8.5, 0.0, Some(0.0), Some(15.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
    }
}

//Up Special 3 Effect
#[acmd_script( agent = "kirby", script = "effect_specialhi3", category = ACMD_EFFECT)]
unsafe fn ssbuexo_kirby_up_special_3_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_kirby_cutter"), Hash40::new("tex_kirby_cutter"), 12, Hash40::new("haver"), 0.0, 3.0, 0.25, Hash40::new("haver"), 0.0, 15.0, 0.5, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("kirby_fcut_rise"), Hash40::new("haver"), 0, 3, 0.3, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 3);
    }
}

//Up Special 3 Sound
#[acmd_script( agent = "kirby", script = "sound_specialhi3", category = ACMD_SOUND)]
unsafe fn ssbuexo_kirby_up_special_3_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_kirby_008"));
        macros::PLAY_SE(fighter, Hash40::new("se_kirby_special_h03"));
    }
}

//Up Special 3 Slam ACMD
#[acmd_script( agent = "kirby", script = "game_specialhi31", category = ACMD_GAME)]
unsafe fn ssbuexo_kirby_up_special_3_slam_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi3"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 2.0, 275, 100, 96, 0, 6.0, 0.0, 8.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi32"), 0.0, 1.0, false, 0.0, false, false);
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3, true);
    }
}

//Up Special 3 Slam Effect
#[acmd_script( agent = "kirby", script = "effect_specialhi31", category = ACMD_EFFECT)]
unsafe fn ssbuexo_kirby_up_special_3_slam_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_kirby_cutter"), Hash40::new("tex_kirby_cutter"), 12, Hash40::new("haver"), 0.0, 2.5, 0.5, Hash40::new("haver"), 0.0, 12.0, 0.3, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, -4, 16, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//Up Special 3 Fall ACMD
#[acmd_script( agent = "kirby", script = "game_specialhi32", category = ACMD_GAME)]
unsafe fn ssbuexo_kirby_up_special_3_fall_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi3"), false, -1.0);
        macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 2.0, 275, 100, 96, 0, 6.0, 0.0, 8.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi32"), 0.0, 1.0, false, 0.0, false, false);
    }
}

//Up Special 3 Fall Effect
#[acmd_script( agent = "kirby", script = "effect_specialhi32", category = ACMD_EFFECT)]
unsafe fn ssbuexo_kirby_up_special_3_fall_effect(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_kirby_cutter"), Hash40::new("tex_kirby_cutter"), 12, Hash40::new("haver"), 0.0, 2.5, 0.5, Hash40::new("haver"), 0.0, 12.0, 0.3, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, -4, 16, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//Aerial Stone ACMD
#[acmd_script( agent = "kirby", script = "game_specialairlw", category = ACMD_GAME)]
unsafe fn ssbuexo_kirby_aerial_stone_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, Hash40::new("special_air_lw"), false, -1.0);
        MotionModule::set_rate(fighter.module_accessor, 1.21);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_STONE_BLINK_ONOFF);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 25);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, Hash40::new("special_lw_to_air"), false, -1.0);
        AttackModule::clear(fighter.module_accessor, 1, false);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 70, 76, 0, 69, 6.5, 0.0, 2.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::init_attack_pos(fighter.module_accessor, 0);
    }
}

//Aerial Stone Landing ACMD
#[acmd_script( agent = "kirby", script = "game_speciallwtoground", category = ACMD_GAME)]
unsafe fn ssbuexo_kirby_aerial_stone_landing_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, Hash40::new("special_lw_to_ground"), false, -1.0);
        macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 14.0, 25, 30, 0, 86, 6.0, 0.0, 4.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 14.0, 25, 30, 0, 86, 6.0, 0.0, 4.0, -6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::init_attack_pos(fighter.module_accessor, 0);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_kirby_grounded_side_special_start_acmd,
        ssbuexo_kirby_aerial_side_special_start_acmd,
        ssbuexo_kirby_grounded_side_special_start_effect,
        ssbuexo_kirby_aerial_side_special_start_effect,
        ssbuexo_kirby_side_special_start_sound,
        ssbuexo_kirby_grounded_side_special_start_expression,
        ssbuexo_kirby_aerial_side_special_start_expression,
        ssbuexo_kirby_side_special_hammer_effect,
        ssbuexo_kirby_side_special_hold_acmd,
        ssbuexo_kirby_side_special_turn_acmd,
        ssbuexo_kirby_side_special_turn_sound,
        ssbuexo_kirby_side_special_jump_acmd,
        ssbuexo_kirby_side_special_jump_sound,
        ssbuexo_kirby_grounded_side_special_attack_acmd,
        ssbuexo_kirby_aerial_side_special_attack_acmd,
        ssbuexo_kirby_side_special_attack_effect,
        ssbuexo_kirby_grounded_side_special_attack_sound,
        ssbuexo_kirby_aerial_side_special_attack_sound,
        ssbuexo_kirby_side_special_attack_expression,
        ssbuexo_kirby_grounded_up_special_1_effect,
        ssbuexo_kirby_aerial_up_special_1_effect,
        ssbuexo_kirby_grounded_up_special_2_acmd,
        ssbuexo_kirby_aerial_up_special_2_acmd,
        ssbuexo_kirby_up_special_2_effect,
        ssbuexo_kirby_up_special_3_acmd,
        ssbuexo_kirby_up_special_3_effect,
        ssbuexo_kirby_up_special_3_sound,
        ssbuexo_kirby_up_special_3_slam_acmd,
        ssbuexo_kirby_up_special_3_slam_effect,
        ssbuexo_kirby_up_special_3_fall_acmd,
        ssbuexo_kirby_up_special_3_fall_effect,
        ssbuexo_kirby_aerial_stone_acmd,
        ssbuexo_kirby_aerial_stone_landing_acmd
    );
}