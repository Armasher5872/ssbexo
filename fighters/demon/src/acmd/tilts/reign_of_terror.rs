use super::*;

//Reign of Terror ACMD
unsafe extern "C" fn ssbexo_demon_reign_of_terror_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 180, 5, 0, 20, 3.5, 0.0, 6.5, 10.5, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 180, 5, 0, 20, 3.0, 0.0, 9.0, 5.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(boma, 5, 9.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 6, 9.0, false);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 6.0, 180, 0, 0, 35, 3.5, 0.0, 6.5, 10.5, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 1, Hash40::new("top"), 6.0, 180, 0, 0, 35, 3.0, 0.0, 9.0, 5.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 12.0, false);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//Reign of Terror Effect
unsafe extern "C" fn ssbexo_demon_reign_of_terror_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 10.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 3, 0, 1.7, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("toer"), 0, 0, 0, 0, 0, 0, 0.55, 0, 0, 0, 0, 0, 360, true, 0.2);
        LAST_EFFECT_SET_RATE(agent, 1.8);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 0.55, 0, 0, 0, 0, 0, 360, true, 0.2);
        LAST_EFFECT_SET_RATE(agent, 1.8);
    }
}

//Reign of Terror Sound
unsafe extern "C" fn ssbexo_demon_reign_of_terror_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_demon_appeal_s03"));
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_demon_appeal_s04"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_demon_rnd_attack_s_01"));
    }
    frame(lua_state, 65.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_demon_step_right_l"));
    }
}

//Reign of Terror Expression
unsafe extern "C" fn ssbexo_demon_reign_of_terror_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_R, 11);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 16);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 15, false, *BATTLE_OBJECT_ID_INVALID as u32);
        RUMBLE_HIT(agent, Hash40::new("rbkind_80_attackm"), 9);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 1);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_R, 37);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        RUMBLE_HIT(agent, Hash40::new("rbkind_80_attackm"), 0);
    }
    frame(lua_state, 65.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_attackstand52", ssbexo_demon_reign_of_terror_acmd, Low)
    .acmd("effect_attackstand52", ssbexo_demon_reign_of_terror_effect, Low)
    .acmd("sound_attackstand52", ssbexo_demon_reign_of_terror_sound, Low)
    .acmd("expression_attackstand52", ssbexo_demon_reign_of_terror_expression, Low)
    .install()
    ;
}