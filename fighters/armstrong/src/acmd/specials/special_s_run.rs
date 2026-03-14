use super::*;

//Grounded Side Special Run ACMD
unsafe extern "C" fn ssbexo_armstrong_grounded_side_special_run_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 6.0, 7.5, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 5.0, 0.0, 11.0, 7.5, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON, *COLLISION_SITUATION_MASK_GA);
    }
}

//Grounded Side Special Run Effect
unsafe extern "C" fn ssbexo_armstrong_grounded_side_special_run_effect(agent: &mut L2CAgentBase) {
    for _ in 0..3 {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 2, 0, 4, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 4.0);
    }
}

//Grounded Side Special Run Sound
unsafe extern "C" fn ssbexo_armstrong_grounded_side_special_run_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_step_right_m"));
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_step_left_m"));
    }
}

//Grounded Side Special Run Expression
unsafe extern "C" fn ssbexo_armstrong_grounded_side_special_run_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

//Aerial Side Special Run ACMD
unsafe extern "C" fn ssbexo_armstrong_aerial_side_special_run_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 6.0, 7.5, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 5.0, 0.0, 11.0, 7.5, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON, *COLLISION_SITUATION_MASK_GA);
    }
}

//Aerial Side Special Run Effect
unsafe extern "C" fn ssbexo_armstrong_aerial_side_special_run_effect(agent: &mut L2CAgentBase) {
    for _ in 0..3 {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 2, 0, 4, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 4.0);
    }
}

//Aerial Side Special Run Sound
unsafe extern "C" fn ssbexo_armstrong_aerial_side_special_run_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_step_right_m"));
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_step_left_m"));
    }
}

//Aerial Side Special Run Expression
unsafe extern "C" fn ssbexo_armstrong_aerial_side_special_run_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([8, 9, 10, 11, 12, 13, 14, 15].to_vec())
    .game_acmd("game_specialsrun", ssbexo_armstrong_grounded_side_special_run_acmd, Low)
    .effect_acmd("effect_specialsrun", ssbexo_armstrong_grounded_side_special_run_effect, Low)
    .sound_acmd("sound_specialsrun", ssbexo_armstrong_grounded_side_special_run_sound, Low)
    .expression_acmd("expression_specialsrun", ssbexo_armstrong_grounded_side_special_run_expression, Low)
    .game_acmd("game_specialairsrun", ssbexo_armstrong_aerial_side_special_run_acmd, Low)
    .effect_acmd("effect_specialairsrun", ssbexo_armstrong_aerial_side_special_run_effect, Low)
    .sound_acmd("sound_specialairsrun", ssbexo_armstrong_aerial_side_special_run_sound, Low)
    .expression_acmd("expression_specialairsrun", ssbexo_armstrong_aerial_side_special_run_expression, Low)
    .install()
    ;
}