use super::*;

//Final Smash Start
unsafe extern "C" fn ssbexo_armstrong_final_smash_start_acmd(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_CHARGE) {
        if is_excute(agent) {
            CHECK_VALID_FINAL_START_CAMERA(agent, 0, 40, 15, 0, 0, 0);
            REQ_FINAL_START_CAMERA_arg3(agent, Hash40::new("d04finalstart.nuanmb"), false, false);
            FT_SET_FINAL_FEAR_FACE(agent, 60);
            FT_START_CUTIN(agent);
        }
        frame(agent.lua_state_agent, 27.0);
        if is_excute(agent) {
            CAM_ZOOM_OUT(agent);
            ATTACK(agent, 0, 0, Hash40::new("top"), 40.0, 361, 40, 0, 40, 10.0, 0.0, 9.0, 6.0, None, None, None, 3.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze_ghost"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
            AttackModule::set_paralyze_frame(agent.module_accessor, 0, 120, false);
        }
        frame(agent.lua_state_agent, 37.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    else {
        if is_excute(agent) {
            WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
            SLOW_OPPONENT(agent, 8.0, 70.0);
            SlowModule::set_whole(agent.module_accessor, 2, 20);
            CHECK_VALID_FINAL_START_CAMERA(agent, 0, 40, 15, 0, 0, 0);
            REQ_FINAL_START_CAMERA_arg3(agent, Hash40::new("d04finalstart.nuanmb"), false, false);
            FT_SET_FINAL_FEAR_FACE(agent, 60);
            FT_START_CUTIN(agent);
        }
        frame(agent.lua_state_agent, 27.0);
        if is_excute(agent) {
            CAM_ZOOM_OUT(agent);
            ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 0.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        frame(agent.lua_state_agent, 28.0);
        if is_excute(agent) {
            CATCH(agent, 0, Hash40::new("top"), 14.0, 0.0, 9.0, 6.0, None, None, None, *FIGHTER_STATUS_KIND_THROWN, *COLLISION_SITUATION_MASK_GA);
        }
        frame(agent.lua_state_agent, 37.0);
        if is_excute(agent) {
            grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        }
    }
}

//Grounded Final Smash Start Effect
unsafe extern "C" fn ssbexo_armstrong_grounded_final_smash_start_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EffectModule::req_screen(agent.module_accessor, Hash40::new("bg_ganon_final"), false, false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_final_aura"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_hold"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 27.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("handr"), 0.5, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("kneel"), 1, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("head"), 1, 0, 0, 0, 0, 0, 1.2, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("handl"), 0.5, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("kneer"), 1, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 44.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Final Smash Start Effect
unsafe extern "C" fn ssbexo_armstrong_aerial_final_smash_start_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EffectModule::req_screen(agent.module_accessor, Hash40::new("bg_ganon_final"), false, false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("sys_final_aura"), false, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_majinken_hold"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 27.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("handr"), 0.5, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("kneel"), 1, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("head"), 1, 0, 0, 0, 0, 0, 1.2, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("handl"), 0.5, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("kneer"), 1, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura2"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 30.0);
    for _ in 0..=4 {
        if is_excute(agent) {
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, 2, 0, 180, 0, 1.5, true);
        }
        wait(agent.lua_state_agent, 3.0);
    }
}

//Final Smash Start Sound
unsafe extern "C" fn ssbexo_armstrong_final_smash_start_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_attack05"));
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_swing_10"));
    }
    wait(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_swing_10"));
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_attackhard_03"));
    }
}

//Grounded Final Smash Start Expression
unsafe extern "C" fn ssbexo_armstrong_grounded_final_smash_start_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        START_INFO_FLASH_EYE(agent.lua_state_agent);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        AREA_WIND_2ND_arg10(agent, 0.0, 3.0, 200.0, 8.0, 1.0, -4.0, 8.0, 30.0, 20.0, 50.0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_rush"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 44.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

//Aerial Final Smash Start Expression
unsafe extern "C" fn ssbexo_armstrong_aerial_final_smash_start_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        START_INFO_FLASH_EYE(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        AREA_WIND_2ND_arg10(agent, 0.0, 3.0, 200.0, 8.0, 1.0, -4.0, 8.0, 30.0, 20.0, 50.0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_rush"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([8, 9, 10, 11, 12, 13, 14, 15].to_vec())
    .game_acmd("game_finalstart", ssbexo_armstrong_final_smash_start_acmd, Low)
    .game_acmd("game_finalairstart", ssbexo_armstrong_final_smash_start_acmd, Low)
    .effect_acmd("effect_finalstart", ssbexo_armstrong_grounded_final_smash_start_effect, Low)
    .effect_acmd("effect_finalairstart", ssbexo_armstrong_aerial_final_smash_start_effect, Low)
    .sound_acmd("sound_finalstart", ssbexo_armstrong_final_smash_start_sound, Low)
    .sound_acmd("sound_finalairstart", ssbexo_armstrong_final_smash_start_sound, Low)
    .expression_acmd("expression_finalstart", ssbexo_armstrong_grounded_final_smash_start_expression, Low)
    .expression_acmd("expression_finalairstart", ssbexo_armstrong_aerial_final_smash_start_expression, Low)
    .install()
    ;
}