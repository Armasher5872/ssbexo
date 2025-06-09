use super::*;

//Grounded Side Special Start ACMD
unsafe extern "C" fn ssbexo_kirby_grounded_side_special_start_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, 0);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_s"), false, 0.0);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kirby_armfoot"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kirby_eye1"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kirby_facen"), false);
    }
}

//Aerial Side Special Start ACMD
unsafe extern "C" fn ssbexo_kirby_aerial_side_special_start_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, 0);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_s"), false, 0.0);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kirby_armfoot"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kirby_eye1"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kirby_facen"), false);
    }
}

//Grounded Side Special Start Effect
unsafe extern "C" fn ssbexo_kirby_grounded_side_special_start_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 8.0);
	if is_excute(agent) {
        EFFECT(agent, Hash40::new("kirby_stone_s"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//Aerial Side Special Start Effect
unsafe extern "C" fn ssbexo_kirby_aerial_side_special_start_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 10.0);
	if is_excute(agent) {
        EFFECT(agent, Hash40::new("kirby_stone_s"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//Side Special Start Sound
unsafe extern "C" fn ssbexo_kirby_side_special_start_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_kirby_landing01"));
    }
}

//Grounded Side Special Start Expression
unsafe extern "C" fn ssbexo_kirby_grounded_side_special_start_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        ItemModule::set_attach_item_visibility(agent.module_accessor, false, 0);
        HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("claviclec"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("clavicler"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("claviclel"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("handl"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("legc"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("footr"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("footl"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("toer"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("toel"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("rot"), *HIT_STATUS_NORMAL);
    }
}

//Aerial Side Special Start Expression
unsafe extern "C" fn ssbexo_kirby_aerial_side_special_start_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        ItemModule::set_attach_item_visibility(agent.module_accessor, false, 0);
        HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("claviclec"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("clavicler"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("claviclel"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("handl"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("legc"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("footr"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("footl"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("toer"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("toel"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("rot"), *HIT_STATUS_NORMAL);
    }
}

//Side Special Hammer Effect
unsafe extern "C" fn ssbexo_kirby_side_special_hammer_effect(_fighter: &mut L2CAgentBase) {}

//Side Special Hold ACMD
unsafe extern "C" fn ssbexo_kirby_side_special_hold_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    for _ in 0..60 {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("have"), 0.3, 90, 50, 70, 0, 4.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 1.0);
    }
}

//Side Special Turn ACMD
unsafe extern "C" fn ssbexo_kirby_side_special_turn_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kirby_armfoot"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kirby_eye1"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kirby_facen"), false);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_s"), false, 0.0);
        REVERSE_LR(agent);
    }
    frame(agent.lua_state_agent, 1.0);
    for _ in 0..60 {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("have"), 0.3, 90, 50, 70, 0, 4.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 1.0);
    }
}

//Side Special Turn Sound
unsafe extern "C" fn ssbexo_kirby_side_special_turn_sound(agent: &mut L2CAgentBase) {
	if is_excute(agent) {
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
        STOP_SE(agent, Hash40::new("se_kirby_attack100"));
        PLAY_SE(agent, Hash40::new("se_kirby_attack100"));
    }
}

//Side Special Jump ACMD
unsafe extern "C" fn ssbexo_kirby_side_special_jump_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, 0);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kirby_armfoot"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kirby_eye1"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kirby_facen"), false);
    }
}

//Side Special Jump Sound
unsafe extern "C" fn ssbexo_kirby_side_special_jump_sound(agent: &mut L2CAgentBase) {
	if is_excute(agent) {
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
        STOP_SE(agent, Hash40::new("se_kirby_attack100"));
    }
}

//Grounded Side Special Attack ACMD
unsafe extern "C" fn ssbexo_kirby_grounded_side_special_attack_acmd(agent: &mut L2CAgentBase) {
    let wheel_power_up = WorkModule::get_float(agent.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLOAT_WHEEL_POWER_UP);
    if is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kirby_armfoot"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kirby_eye1"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kirby_facen"), false);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_s"), false, 0.0);
        ATTACK(agent, 0, 0, Hash40::new("have"), 10.0*wheel_power_up, 75, 60, 0, 40, 6.0, 0.0, 3.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("have"), 8.0*wheel_power_up, 75, 60, 0, 40, 6.0, 0.0, 3.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("have"), 6.0*wheel_power_up, 75, 60, 0, 40, 6.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 30.0);
	if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kirby_armfoot"), true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kirby_eye1"), true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kirby_facen"), true);
    }
}

//Aerial Side Special Attack ACMD
unsafe extern "C" fn ssbexo_kirby_aerial_side_special_attack_acmd(agent: &mut L2CAgentBase) {
    let wheel_power_up = WorkModule::get_float(agent.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLOAT_WHEEL_POWER_UP);
    if is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kirby_armfoot"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kirby_eye1"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kirby_facen"), false);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_s"), false, 0.0);
        ATTACK(agent, 0, 0, Hash40::new("have"), 10.0*wheel_power_up, 75, 60, 0, 40, 6.0, 0.0, 3.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("have"), 8.0*wheel_power_up, 75, 60, 0, 40, 6.0, 0.0, 3.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("have"), 6.0*wheel_power_up, 75, 60, 0, 40, 6.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 30.0);
	if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kirby_armfoot"), true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kirby_eye1"), true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("kirby_facen"), true);
    }
}

//Side Special Attack Effect
unsafe extern "C" fn ssbexo_kirby_side_special_attack_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 7.5, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 3.0);
	if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 2, 6.5, 0, 0, 180, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.6, 0.7, 1.0);
        LAST_EFFECT_SET_RATE(agent, 0.2);
    }
    frame(agent.lua_state_agent, 30.0);
	if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_attack_speedline"), false, true);
    }
    frame(agent.lua_state_agent, 35.0);
	if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("kirby_stone_e"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
    }
}

//Grounded Side Special Attack Sound
unsafe extern "C" fn ssbexo_kirby_grounded_side_special_attack_sound(agent: &mut L2CAgentBase) {
	if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_kirby_special_h02"));
        PLAY_SE(agent, Hash40::new("se_kirby_attack100"));
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_kirby_008"));
    }
    frame(agent.lua_state_agent, 33.0);
	if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_kirby_special_h02"));
    }
    frame(agent.lua_state_agent, 35.0);
	if is_excute(agent) {
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
        STOP_SE(agent, Hash40::new("se_kirby_attack100"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_kirby_rnd_special_lw"));
    }
    frame(agent.lua_state_agent, 38.0);
	if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_kirby_special_l03"));
    }
}

//Aerial Side Special Attack Sound
unsafe extern "C" fn ssbexo_kirby_aerial_side_special_attack_sound(agent: &mut L2CAgentBase) {
	if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_kirby_special_h02"));
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_kirby_008"));
    }
    frame(agent.lua_state_agent, 33.0);
	if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_kirby_special_h02"));
    }
    frame(agent.lua_state_agent, 35.0);
	if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_kirby_rnd_special_lw"));
    }
    frame(agent.lua_state_agent, 38.0);
	if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_kirby_special_l03"));
    }
}

//Side Special Attack Expression
unsafe extern "C" fn ssbexo_kirby_side_special_attack_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("claviclec"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("clavicler"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("claviclel"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("handl"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("legc"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("footr"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("footl"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("toer"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("toel"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("rot"), *HIT_STATUS_NORMAL);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        ItemModule::set_attach_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 31.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 35.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
        HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("s_stingd1"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("footr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("footl"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("rot"), *HIT_STATUS_OFF);
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_rush"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

//Grounded Up Special 1 Effect
unsafe extern "C" fn ssbexo_kirby_grounded_up_special_1_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("kirby_fcut_arc"), Hash40::new("kirby_fcut_arc"), Hash40::new("top"), 0, 12, 0, 0, 0, -149, 1, false, *EF_FLIP_YZ);
    }
}

//Aerial Up Special 1 Effect
unsafe extern "C" fn ssbexo_kirby_aerial_up_special_1_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("kirby_fcut_arc"), Hash40::new("kirby_fcut_arc"), Hash40::new("top"), 0, 12, 0, 0, 0, -149, 1, false, *EF_FLIP_YZ);
    }
}

//Grounded Up Special 2 ACMD
unsafe extern "C" fn ssbexo_kirby_grounded_up_special_2_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi2"), false, -1.0);
    }
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 5.0, 90, 100, 100, 0, 6.0, 0.0, 8.5, 0.0, Some(0.0), Some(15.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 5.0, 90, 100, 50, 0, 6.0, 0.0, 8.5, 0.0, Some(0.0), Some(15.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Aerial Up Special 2 ACMD
unsafe extern "C" fn ssbexo_kirby_aerial_up_special_2_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi2"), false, -1.0);
    }
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 5.0, 90, 100, 100, 0, 6.0, 0.0, 8.5, 0.0, Some(0.0), Some(15.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 5.0, 90, 100, 50, 0, 6.0, 0.0, 8.5, 0.0, Some(0.0), Some(15.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Up Special 2 Effect
unsafe extern "C" fn ssbexo_kirby_up_special_2_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_kirby_cutter"), Hash40::new("tex_kirby_cutter"), 12, Hash40::new("haver"), 0.0, 3.0, 0.25, Hash40::new("haver"), 0.0, 15.0, 0.5, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        EFFECT_FOLLOW(agent, Hash40::new("kirby_fcut_rise"), Hash40::new("haver"), 0, 3, 0.3, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
}

//Up Special 3 ACMD
unsafe extern "C" fn ssbexo_kirby_up_special_3_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi3"), false, -1.0);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hip"), 8.0, 30, 80, 0, 60, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 8.0, 30, 80, 0, 60, 6.0, 0.0, 8.5, 0.0, Some(0.0), Some(15.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 37.0);
    if is_excute(agent) {
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
    }
}

//Up Special 3 Effect
unsafe extern "C" fn ssbexo_kirby_up_special_3_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_kirby_cutter"), Hash40::new("tex_kirby_cutter"), 12, Hash40::new("haver"), 0.0, 3.0, 0.25, Hash40::new("haver"), 0.0, 15.0, 0.5, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        EFFECT_FOLLOW(agent, Hash40::new("kirby_fcut_rise"), Hash40::new("haver"), 0, 3, 0.3, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
}

//Up Special 3 Sound
unsafe extern "C" fn ssbexo_kirby_up_special_3_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_kirby_008"));
        PLAY_SE(agent, Hash40::new("se_kirby_special_h03"));
    }
}

//Up Special 3 Slam ACMD
unsafe extern "C" fn ssbexo_kirby_up_special_3_slam_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi3"), false, -1.0);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 2.0, 275, 100, 96, 0, 6.0, 0.0, 8.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("special_hi32"), 0.0, 1.0, false, 0.0, false, false);
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3, true);
    }
}

//Up Special 3 Slam Effect
unsafe extern "C" fn ssbexo_kirby_up_special_3_slam_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_kirby_cutter"), Hash40::new("tex_kirby_cutter"), 12, Hash40::new("haver"), 0.0, 2.5, 0.5, Hash40::new("haver"), 0.0, 12.0, 0.3, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, -4, 16, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//Up Special 3 Fall ACMD
unsafe extern "C" fn ssbexo_kirby_up_special_3_fall_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi3"), false, -1.0);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 2.0, 275, 100, 96, 0, 6.0, 0.0, 8.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("special_hi32"), 0.0, 1.0, false, 0.0, false, false);
    }
}

//Up Special 3 Fall Effect
unsafe extern "C" fn ssbexo_kirby_up_special_3_fall_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_kirby_cutter"), Hash40::new("tex_kirby_cutter"), 12, Hash40::new("haver"), 0.0, 2.5, 0.5, Hash40::new("haver"), 0.0, 12.0, 0.3, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1, *TRAIL_CULL_NONE, 1.29999995, 0.100000001);
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, -4, 16, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//Aerial Stone ACMD
unsafe extern "C" fn ssbexo_kirby_aerial_stone_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, Hash40::new("special_air_lw"), false, -1.0);
        MotionModule::set_rate(agent.module_accessor, 1.21);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_STONE_BLINK_ONOFF);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 25);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, Hash40::new("special_lw_to_air"), false, -1.0);
        AttackModule::clear(agent.module_accessor, 1, false);
        ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 70, 76, 0, 69, 6.5, 0.0, 2.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::init_attack_pos(agent.module_accessor, 0);
    }
}

//Aerial Stone Landing ACMD
unsafe extern "C" fn ssbexo_kirby_aerial_stone_landing_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, Hash40::new("special_lw_to_ground"), false, -1.0);
        ATTACK(agent, 0, 1, Hash40::new("top"), 14.0, 25, 30, 0, 86, 6.0, 0.0, 4.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 1, Hash40::new("top"), 14.0, 25, 30, 0, 86, 6.0, 0.0, 4.0, -6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::init_attack_pos(agent.module_accessor, 0);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("kirby")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialsstart", ssbexo_kirby_grounded_side_special_start_acmd, Low)
    .game_acmd("game_specialairsstart", ssbexo_kirby_aerial_side_special_start_acmd, Low)
    .effect_acmd("effect_specialsstart", ssbexo_kirby_grounded_side_special_start_effect, Low)
    .effect_acmd("effect_specialairsstart", ssbexo_kirby_aerial_side_special_start_effect, Low)
    .sound_acmd("sound_specialsstart", ssbexo_kirby_side_special_start_sound, Low)
    .sound_acmd("sound_specialairsstart", ssbexo_kirby_side_special_start_sound, Low)
    .expression_acmd("expression_specialsstart", ssbexo_kirby_grounded_side_special_start_expression, Low)
    .expression_acmd("expression_specialairsstart", ssbexo_kirby_aerial_side_special_start_expression, Low)
    .game_acmd("game_specialshold", ssbexo_kirby_side_special_hold_acmd, Low)
    .game_acmd("game_specialsfall", ssbexo_kirby_side_special_hold_acmd, Low)
    .game_acmd("game_specialsturn", ssbexo_kirby_side_special_turn_acmd, Low)
    .sound_acmd("sound_specialsturn", ssbexo_kirby_side_special_turn_sound, Low)
    .game_acmd("game_specialsjump", ssbexo_kirby_side_special_jump_acmd, Low)
    .sound_acmd("sound_specialsjump", ssbexo_kirby_side_special_jump_sound, Low)
    .game_acmd("game_specials", ssbexo_kirby_grounded_side_special_attack_acmd, Low)
    .game_acmd("game_specialairs", ssbexo_kirby_aerial_side_special_attack_acmd, Low)
    .effect_acmd("effect_specials", ssbexo_kirby_side_special_attack_effect, Low)
    .effect_acmd("effect_specialairs", ssbexo_kirby_side_special_attack_effect, Low)
    .sound_acmd("sound_specials", ssbexo_kirby_grounded_side_special_attack_sound, Low)
    .sound_acmd("sound_specialairs", ssbexo_kirby_aerial_side_special_attack_sound, Low)
    .expression_acmd("expression_specials", ssbexo_kirby_side_special_attack_expression, Low)
    .expression_acmd("expression_specialairs", ssbexo_kirby_side_special_attack_expression, Low)
    .effect_acmd("effect_specialhi", ssbexo_kirby_grounded_up_special_1_effect, Low)
    .effect_acmd("effect_specialairhi", ssbexo_kirby_aerial_up_special_1_effect, Low)
    .game_acmd("game_specialhi2", ssbexo_kirby_grounded_up_special_2_acmd, Low)
    .game_acmd("game_specialairhi2", ssbexo_kirby_aerial_up_special_2_acmd, Low)
    .effect_acmd("effect_specialhi2", ssbexo_kirby_up_special_2_effect, Low)
    .effect_acmd("effect_specialairhi2", ssbexo_kirby_up_special_2_effect, Low)
    .game_acmd("game_specialhi3", ssbexo_kirby_up_special_3_acmd, Low)
    .effect_acmd("effect_specialhi3", ssbexo_kirby_up_special_3_effect, Low)
    .sound_acmd("sound_specialhi3", ssbexo_kirby_up_special_3_sound, Low)
    .game_acmd("game_specialhi31", ssbexo_kirby_up_special_3_slam_acmd, Low)
    .effect_acmd("effect_specialhi31", ssbexo_kirby_up_special_3_slam_effect, Low)
    .game_acmd("game_specialhi32", ssbexo_kirby_up_special_3_fall_acmd, Low)
    .effect_acmd("effect_specialhi32", ssbexo_kirby_up_special_3_fall_effect, Low)
    .game_acmd("game_specialairlw", ssbexo_kirby_aerial_stone_acmd, Low)
    .game_acmd("game_speciallwtoground", ssbexo_kirby_aerial_stone_landing_acmd, Low)
    .install()
    ;
    Agent::new("kirby_hammer")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .effect_acmd("effect_specialsstart", ssbexo_kirby_side_special_hammer_effect, Low)
    .effect_acmd("effect_specialairsstart", ssbexo_kirby_side_special_hammer_effect, Low)
    .effect_acmd("effect_specials", ssbexo_kirby_side_special_hammer_effect, Low)
    .effect_acmd("effect_specialairs", ssbexo_kirby_side_special_hammer_effect, Low)
    .install()
    ;
}