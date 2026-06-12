use super::*;

//Standing Grab ACMD
unsafe extern "C" fn ssbexo_demon_grab_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 6.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        //CATCH(agent, 0, Hash40::new("top"), 3.4, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(10.5), *FIGHTER_STATUS_KIND_THROWN, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 3.3, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(10.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

//Gates of Hell ACMD
unsafe extern "C" fn ssbexo_demon_gates_of_hell_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.6);
    frame(lua_state, 3.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 6.6, 5.0, Some(0.0), Some(6.6), Some(11.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_CATCH_COMMAND_FLAG_CHANGE_THROW);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 1.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

//Dash Grab ACMD
unsafe extern "C" fn ssbexo_demon_dash_grab_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 8.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        //CATCH(agent, 0, Hash40::new("top"), 3.7, 0.0, 6.6, 5.0, Some(0.0), Some(6.6), Some(11.5), *FIGHTER_STATUS_KIND_THROWN, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 3.6, 0.0, 6.6, 5.0, Some(0.0), Some(6.6), Some(11.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

//Pivot Grab ACMD
unsafe extern "C" fn ssbexo_demon_pivot_grab_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 9.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        //CATCH(agent, 0, Hash40::new("top"), 3.4, 0.0, 6.6, -5.0, Some(0.0), Some(6.6), Some(-13.5), *FIGHTER_STATUS_KIND_THROWN, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 3.3, 0.0, 6.6, -5.0, Some(0.0), Some(6.6), Some(-13.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

//Skull Smash ACMD
unsafe extern "C" fn ssbexo_demon_skull_smash_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 270, 40, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 80, 100, 0, 30, 8.0, 0.0, 8.0, 8.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 11.0, 0.0);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_THROW);
    }
}

//Skull Smash Effect
unsafe extern "C" fn ssbexo_demon_skull_smash_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("demon_s_fujinken_elec"), Hash40::new("waist"), 2, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("demon_combo_elec"), Hash40::new("armr"), 0.5, 0, -0.5, 0, 0, 0, 1, true);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 8, 8, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 360, true, 0.2);
        FOOT_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("demon_s_fujinken_elec"), false, false);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("demon_combo_elec"), false, false);
    }
}

//Skull Smash Sound
unsafe extern "C" fn ssbexo_demon_skull_smash_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 30.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_demon_rnd_attack_s_03"));
        PLAY_SE(agent, Hash40::new("se_demon_attackstep2l_01"));
    }
}

//Skull Smash Expression
unsafe extern "C" fn ssbexo_demon_skull_smash_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_80_attack_special_t"), 0);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_80_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_catch", ssbexo_demon_grab_acmd, Low)
    .acmd("game_catchcommand", ssbexo_demon_gates_of_hell_acmd, Low)
    .acmd("game_catchdash", ssbexo_demon_dash_grab_acmd, Low)
    .acmd("game_catchturn", ssbexo_demon_pivot_grab_acmd, Low)
    .acmd("game_throwsmash", ssbexo_demon_skull_smash_acmd, Low)
    .acmd("effect_throwsmash", ssbexo_demon_skull_smash_effect, Low)
    .acmd("sound_throwsmash", ssbexo_demon_skull_smash_sound, Low)
    .acmd("expression_throwsmash", ssbexo_demon_skull_smash_expression, Low)
    .install()
    ;
}