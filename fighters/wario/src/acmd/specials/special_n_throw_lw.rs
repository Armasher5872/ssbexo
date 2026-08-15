use super::*;

//Grounded Neutral Special Down Throw ACMD
unsafe extern "C" fn ssbexo_wario_grounded_neutral_special_throw_lw_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 80, 60, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 4, 4);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_N_THROW);
    }
}

//Aerial Neutral Special Down Throw ACMD
unsafe extern "C" fn ssbexo_wario_aerial_neutral_special_throw_lw_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 21.0);
    if is_excute(agent) {
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(set_speed, agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.7);
        sv_kinetic_energy!(set_accel, agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.01);
    }
}

//Grounded Neutral Special Down Throw Effect
unsafe extern "C" fn ssbexo_wario_grounded_neutral_special_throw_lw_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 6.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("wario_kamitsuki_open"), Hash40::new("top"), 0, 10, 0, 90, 0, 0, 1, false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("wario_kamitsuki_bite"), Hash40::new("top"), 0, 13, 0, 90, 0, 0, 1, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Neutral Special Down Throw Effect
unsafe extern "C" fn ssbexo_wario_aerial_neutral_special_throw_lw_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("wario_attack_air_n"), Hash40::new("wario_attack_air_n"), Hash40::new("rot"), 0, 3, 0, 0, -90, 0, 1, true, *EF_FLIP_YZ, 1);
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("wario_attack_air_n"), Hash40::new("wario_attack_air_n"), Hash40::new("rot"), 0, 2, 0, 0, -90, 0, 1.8, true, *EF_FLIP_YZ, 0.6);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("wario_attack_air_n"), Hash40::new("wario_attack_air_n"), Hash40::new("rot"), 0, 4, 0, 0, -90, 0, 1, true, *EF_FLIP_YZ, 1);
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("wario_attack_air_n"), Hash40::new("wario_attack_air_n"), Hash40::new("rot"), 0, 5, 0, 0, -90, 0, 1.8, true, *EF_FLIP_YZ, 0.6);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("wario_attack_air_n"), Hash40::new("wario_attack_air_n"), Hash40::new("rot"), 0, 3, 0, 0, -90, 0, 1, true, *EF_FLIP_YZ, 1);
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("wario_attack_air_n"), Hash40::new("wario_attack_air_n"), Hash40::new("rot"), 0, 4, 0, 0, -90, 0, 1.8, true, *EF_FLIP_YZ, 0.6);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("wario_attack_air_n"), Hash40::new("wario_attack_air_n"), Hash40::new("rot"), 0, 3, 0, 0, -90, 0, 1, true, *EF_FLIP_YZ, 1);
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("wario_attack_air_n"), Hash40::new("wario_attack_air_n"), Hash40::new("rot"), 0, 2, 0, 0, -90, 0, 1.8, true, *EF_FLIP_YZ, 0.6);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("wario_attack_air_n"), Hash40::new("wario_attack_air_n"), Hash40::new("rot"), 0, 4, 0, 0, -90, 0, 1, true, *EF_FLIP_YZ, 1);
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("wario_attack_air_n"), Hash40::new("wario_attack_air_n"), Hash40::new("rot"), 0, 5, 0, 0, -90, 0, 1.8, true, *EF_FLIP_YZ, 0.6);
    }
    frame(lua_state, 47.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 52.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("wario_attack_air_n"), Hash40::new("wario_attack_air_n"), Hash40::new("rot"), 0, 3, 0, 0, -90, 0, 1, true, *EF_FLIP_YZ, 1);
        EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("wario_attack_air_n"), Hash40::new("wario_attack_air_n"), Hash40::new("rot"), 0, 4, 0, 0, -90, 0, 1.8, true, *EF_FLIP_YZ, 0.6);
    }
    frame(lua_state, 69.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("rot"), 0, 20, 0, 90, 0, 0, 1.2, true);
    }
}

//Grounded Neutral Special Down Throw Sound
unsafe extern "C" fn ssbexo_wario_grounded_neutral_special_throw_lw_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_wario_001"));
        PLAY_SE_REMAIN(agent, Hash40::new("se_wario_special_n02"));
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_special_n05"));
        PLAY_SE(agent, Hash40::new("vc_wario_002"));
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_swing_l"));
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_03"));
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_landing01"));
    }
}

//Aerial Neutral Special Down Throw Sound
unsafe extern "C" fn ssbexo_wario_aerial_neutral_special_throw_lw_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_special_l03"));
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_jump02"));
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        let hehaho = SoundModule::play_se(boma, Hash40::new("vc_wario_final01"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(boma, hehaho as i32, 0.75, 0);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_03"));
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_03"));
    }
    frame(lua_state, 51.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_03"));
    }
}

//Grounded Neutral Special Down Throw Expression
unsafe extern "C" fn ssbexo_wario_grounded_neutral_special_throw_lw_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("head"), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_landl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Aerial Neutral Special Down Throw Expression
unsafe extern "C" fn ssbexo_wario_aerial_neutral_special_throw_lw_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialnthrowlw", ssbexo_wario_grounded_neutral_special_throw_lw_acmd, Low)
    .acmd("effect_specialnthrowlw", ssbexo_wario_grounded_neutral_special_throw_lw_effect, Low)
    .acmd("sound_specialnthrowlw", ssbexo_wario_grounded_neutral_special_throw_lw_sound, Low)
    .acmd("expression_specialnthrowlw", ssbexo_wario_grounded_neutral_special_throw_lw_expression, Low)
    .acmd("game_specialairnthrowlw", ssbexo_wario_aerial_neutral_special_throw_lw_acmd, Low)
    .acmd("effect_specialairnthrowlw", ssbexo_wario_aerial_neutral_special_throw_lw_effect, Low)
    .acmd("sound_specialairnthrowlw", ssbexo_wario_aerial_neutral_special_throw_lw_sound, Low)
    .acmd("expression_specialairnthrowlw", ssbexo_wario_aerial_neutral_special_throw_lw_expression, Low)
    .install()
    ;
}