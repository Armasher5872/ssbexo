use super::*;

unsafe extern "C" fn ssbexo_captain_neutral_special_charged_acmd(_agent: &mut L2CAgentBase) {}

unsafe extern "C" fn ssbexo_captain_grounded_neutral_special_charged_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fp_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        let fist = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_damage_fire"), Hash40::new("haver"), &Vector3f::zero(), &Vector3f::zero(), 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
        WorkModule::set_int(agent.module_accessor, fist as i32, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_SPECIAL_N_EFFECT_HANDLE);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn ssbexo_captain_aerial_neutral_special_charged_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fp_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        let fist = EffectModule::req_follow(agent.module_accessor, Hash40::new("captain_fn_flash"), Hash40::new("haver"), &Vector3f::zero(), &Vector3f::zero(), 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
        WorkModule::set_int(agent.module_accessor, fist as i32, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_SPECIAL_N_EFFECT_HANDLE);
    }
}

unsafe extern "C" fn ssbexo_captain_grounded_neutral_special_charged_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_special_h03"));
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_captain_005"));
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        PLAY_LANDING_SE(agent, Hash40::new("se_captain_landing01"));
    }
}

unsafe extern "C" fn ssbexo_captain_aerial_neutral_special_charged_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_captain_final07"));
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_captain_005"));
    }
}

unsafe extern "C" fn ssbexo_captain_grounded_neutral_special_charged_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 53.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(agent.lua_state_agent, 69.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

unsafe extern "C" fn ssbexo_captain_aerial_neutral_special_charged_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("captain")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialncharged", ssbexo_captain_neutral_special_charged_acmd, Low)
    .game_acmd("game_specialairncharged", ssbexo_captain_neutral_special_charged_acmd, Low)
    .effect_acmd("effect_specialncharged", ssbexo_captain_grounded_neutral_special_charged_effect, Low)
    .effect_acmd("effect_specialairncharged", ssbexo_captain_aerial_neutral_special_charged_effect, Low)
    .sound_acmd("sound_specialncharged", ssbexo_captain_grounded_neutral_special_charged_sound, Low)
    .sound_acmd("sound_specialairncharged", ssbexo_captain_aerial_neutral_special_charged_sound, Low)
    .expression_acmd("expression_specialncharged", ssbexo_captain_grounded_neutral_special_charged_expression, Low)
    .expression_acmd("expression_specialairncharged", ssbexo_captain_aerial_neutral_special_charged_expression, Low)
    .install()
    ;
}