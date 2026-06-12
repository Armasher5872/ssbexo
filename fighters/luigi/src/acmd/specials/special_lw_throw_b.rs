use super::*;

//Down Special Back Throw ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_throw_b_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        let get_sum_speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 220, 5, 0, 90, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, get_sum_speed_x);
        sv_kinetic_energy!(set_brake, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.05);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        CHECK_FINISH_CAMERA(agent, 21, 3);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.0);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 10.0, y: -1.0, z: 0.0});
        FT_CATCH_STOP(agent, 5, 1);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_THROW);
        SA_SET(agent, *SITUATION_KIND_AIR);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        ADD_SPEED_NO_LIMIT(agent, 0.5, 1.5);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
    }
}

//Down Special Back Throw Effect
unsafe extern "C" fn ssbexo_luigi_down_special_throw_b_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 29.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("luigi_final_shot"), Hash40::new("throw"), 0, 0, 0, 0, 0, -90, 1, 0, 0, 0, 0, 0, 0, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE);
        smash::app::sv_animcmd::EFFECT_ATTR(lua_state);
    }
}

//Down Special Back Throw Sound
unsafe extern "C" fn ssbexo_luigi_down_special_throw_b_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_luigi_final04"));
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_luigi_rnd_attack"));
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_luigi_final05"));
    }
}

//Down Special Back Throw Expression
unsafe extern "C" fn ssbexo_luigi_down_special_throw_b_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_beamm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_speciallwthrowb", ssbexo_luigi_down_special_throw_b_acmd, Low)
    .acmd("game_specialairlwthrowb", ssbexo_luigi_down_special_throw_b_acmd, Low)
    .acmd("effect_speciallwthrowb", ssbexo_luigi_down_special_throw_b_effect, Low)
    .acmd("effect_specialairlwthrowb", ssbexo_luigi_down_special_throw_b_effect, Low)
    .acmd("sound_speciallwthrowb", ssbexo_luigi_down_special_throw_b_sound, Low)
    .acmd("sound_specialairlwthrowb", ssbexo_luigi_down_special_throw_b_sound, Low)
    .acmd("expression_speciallwthrowb", ssbexo_luigi_down_special_throw_b_expression, Low)
    .acmd("expression_specialairlwthrowb", ssbexo_luigi_down_special_throw_b_expression, Low)
    .install()
    ;
}