use super::*;

//Down Special Forward Throw ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_throw_f_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.0, 25, 45, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 18, 4);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.0);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 12.0, y: -1.0, z: 0.0});
        FT_CATCH_STOP(agent, 5, 1);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_THROW);
    }
    frame(agent.lua_state_agent, 45.0);
    if is_excute(agent) {
        let object_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_OBAKYUMU_OBJECT_ID);
        ArticleModule::remove_exist_object_id(agent.module_accessor, object_id as u32);
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Down Special Forward Throw Effect
unsafe extern "C" fn ssbexo_luigi_down_special_throw_f_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("luigi_final_shot"), Hash40::new("throw"), 0, 0, 0, 0, 0, -90, 1, 0, 0, 0, 0, 0, 0, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE);
        smash::app::sv_animcmd::EFFECT_ATTR(agent.lua_state_agent);
    }
}

//Down Special Forward Throw Sound
unsafe extern "C" fn ssbexo_luigi_down_special_throw_f_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_luigi_final04"));
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_luigi_rnd_attack"));
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_luigi_final05"));
    }
}

//Down Special Forward Throw Expression
unsafe extern "C" fn ssbexo_luigi_down_special_throw_f_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_beamm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallwthrowf", ssbexo_luigi_down_special_throw_f_acmd, Low)
    .game_acmd("game_specialairlwthrowf", ssbexo_luigi_down_special_throw_f_acmd, Low)
    .effect_acmd("effect_speciallwthrowf", ssbexo_luigi_down_special_throw_f_effect, Low)
    .effect_acmd("effect_specialairlwthrowf", ssbexo_luigi_down_special_throw_f_effect, Low)
    .sound_acmd("sound_speciallwthrowf", ssbexo_luigi_down_special_throw_f_sound, Low)
    .sound_acmd("sound_specialairlwthrowf", ssbexo_luigi_down_special_throw_f_sound, Low)
    .expression_acmd("expression_speciallwthrowf", ssbexo_luigi_down_special_throw_f_expression, Low)
    .expression_acmd("expression_specialairlwthrowf", ssbexo_luigi_down_special_throw_f_expression, Low)
    .install()
    ;
}