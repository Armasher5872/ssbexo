use super::*;

//Grounded Down Special Start ACMD
unsafe extern "C" fn ssbexo_wario_grounded_down_special_start_acmd(_agent: &mut L2CAgentBase) {}

//Aerial Down Special Start ACMD
unsafe extern "C" fn ssbexo_wario_aerial_down_special_start_acmd(agent: &mut L2CAgentBase) {
    FT_MOTION_RATE(agent, 24.0/17.0);
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        ATTACK(agent, 0, 0, Hash40::new("hip"), 14.2, 51, 65, 0, 44, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
}

//Grounded Down Special Start Effect
unsafe extern "C" fn ssbexo_wario_grounded_down_special_start_effect(_agent: &mut L2CAgentBase) {}

//Aerial Down Special Start Effect
unsafe extern "C" fn ssbexo_wario_aerial_down_special_start_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("wario_attack_dash"), Hash40::new("top"), 0, 0, 0, 90, 0, 0, 0.6, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 0.0);
    }
}

//Grounded Down Special Start Sound
unsafe extern "C" fn ssbexo_wario_grounded_down_special_start_sound(_agent: &mut L2CAgentBase) {}

//Aerial Down Special Start Sound
unsafe extern "C" fn ssbexo_wario_aerial_down_special_start_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_special_l03"));
        let rand = sv_math::randf(hash40("fighter"), 100.0);
        if rand > 75.0 {
            PLAY_SE(agent, Hash40::new("vc_wario_attack01"));
        }
        else if rand <= 75.0 && rand > 50.0 {
            PLAY_SE(agent, Hash40::new("vc_wario_attack02"));
        }
        else if rand <= 50.0 && rand > 25.0 {
            PLAY_SE(agent, Hash40::new("vc_wario_007"));
        }
    }
}

//Grounded Down Special Start Expression
unsafe extern "C" fn ssbexo_wario_grounded_down_special_start_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

//Aerial Down Special Start Expression
unsafe extern "C" fn ssbexo_wario_aerial_down_special_start_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallwstart", ssbexo_wario_grounded_down_special_start_acmd, Low)
    .effect_acmd("effect_speciallwstart", ssbexo_wario_grounded_down_special_start_effect, Low)
    .sound_acmd("sound_speciallwstart", ssbexo_wario_grounded_down_special_start_sound, Low)
    .expression_acmd("expression_speciallwstart", ssbexo_wario_grounded_down_special_start_expression, Low)
    .game_acmd("game_specialairlwstart", ssbexo_wario_aerial_down_special_start_acmd, Low)
    .effect_acmd("effect_specialairlwstart", ssbexo_wario_aerial_down_special_start_effect, Low)
    .sound_acmd("sound_specialairlwstart", ssbexo_wario_aerial_down_special_start_sound, Low)
    .expression_acmd("expression_specialairlwstart", ssbexo_wario_aerial_down_special_start_expression, Low)
    .install()
    ;
}