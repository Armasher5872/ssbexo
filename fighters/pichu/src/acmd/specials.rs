use super::*;

//Side Special ACMD
unsafe extern "C" fn ssbexo_pichu_side_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 0.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 7.0, 0.0, 5.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    for _ in 0..8 {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_PICHU_INSTANCE_WORK_ID_FLAG_IS_VALID_NUZZLE) {
            CATCH(agent, 0, Hash40::new("top"), 7.0, 0.0, 5.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    frame(agent.lua_state_agent, 27.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(agent.module_accessor, false);
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Side Special Effect
unsafe extern "C" fn ssbexo_pichu_side_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3, 0.5, 0, 0, -90, 0.6, true, 0.7);
        LAST_EFFECT_SET_RATE(agent, 1.4);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 3, 0.5, 0, 90, -90, 0.8, false, 0.5);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 32.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

//Side Special Sound
unsafe extern "C" fn ssbexo_pichu_side_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_pichu_rnd_attack"));
        PLAY_STATUS(agent, Hash40::new("se_pichu_attackair_n01"));
    }
}

//Side Special Expression
unsafe extern "C" fn ssbexo_pichu_side_special_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_nohits"), 0);
    }
}

//Side Special Throw ACMD
unsafe extern "C" fn ssbexo_pichu_side_special_throw_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 0, 0, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, -10, 4);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.0);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: -10.0, y: 4.0, z: 0.0});
        FT_CATCH_STOP(agent, 5, 1);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("throw"), 6.0, 140, 60, 0, 60, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        AttackModule::set_paralyze_frame(agent.module_accessor, 0, 30, false);
        FT_ADD_DAMAGE(agent, 3.5);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Side Special Throw Effect
unsafe extern "C" fn ssbexo_pichu_side_special_throw_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true, 1.0);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_kaminari_hit"), Hash40::new("top"), 0, 6, 0, 0, 90, 0, 0.5, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pichu_kaminari_hit"), false, true);
    }
}

//Side Special Throw Sound
unsafe extern "C" fn ssbexo_pichu_side_special_throw_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    wait(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_02"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_pichu_rnd_attack"));
    }
}

//Side Special Throw Expression
unsafe extern "C" fn ssbexo_pichu_side_special_throw_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

//Aerial Up Special End ACMD
unsafe extern "C" fn ssbexo_pichu_aerial_up_special_end_acmd(agent: &mut L2CAgentBase) {
    let quick_attack_count = WorkModule::get_int(agent.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_INT_QUICK_ATTACK_COUNT);
    if quick_attack_count == 2 {
        if is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_PICHU_INSTANCE_WORK_ID_FLAG_AGILITY_CAN_CANCEL);
        }
    }
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        JostleModule::set_status(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        WorkModule::inc_int(agent.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_INT_QUICK_ATTACK_PHASE);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_SPECIALUPDUMMY, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_PICHU_INSTANCE_WORK_ID_FLAG_AGILITY_CAN_CANCEL);
    }
}

pub fn install() {
    Agent::new("pichu")
    .game_acmd("game_specials", ssbexo_pichu_side_special_acmd, Low)
    .game_acmd("game_specialairs", ssbexo_pichu_side_special_acmd, Low)
    .effect_acmd("effect_specials", ssbexo_pichu_side_special_effect, Low)
    .effect_acmd("effect_specialairs", ssbexo_pichu_side_special_effect, Low)
    .sound_acmd("sound_specials", ssbexo_pichu_side_special_sound, Low)
    .sound_acmd("sound_specialairs", ssbexo_pichu_side_special_sound, Low)
    .expression_acmd("expression_specials", ssbexo_pichu_side_special_expression, Low)
    .expression_acmd("expression_specialairs", ssbexo_pichu_side_special_expression, Low)
    .game_acmd("game_specialsthrow", ssbexo_pichu_side_special_throw_acmd, Low)
    .game_acmd("game_specialairsthrow", ssbexo_pichu_side_special_throw_acmd, Low)
    .effect_acmd("effect_specialsthrow", ssbexo_pichu_side_special_throw_effect, Low)
    .effect_acmd("effect_specialairsthrow", ssbexo_pichu_side_special_throw_effect, Low)
    .sound_acmd("sound_specialsthrow", ssbexo_pichu_side_special_throw_sound, Low)
    .sound_acmd("sound_specialairsthrow", ssbexo_pichu_side_special_throw_sound, Low)
    .expression_acmd("expression_specialsthrow", ssbexo_pichu_side_special_throw_expression, Low)
    .expression_acmd("expression_specialairsthrow", ssbexo_pichu_side_special_throw_expression, Low)
    .game_acmd("game_specialairhiend", ssbexo_pichu_aerial_up_special_end_acmd, Low)
    .install()
    ;
}