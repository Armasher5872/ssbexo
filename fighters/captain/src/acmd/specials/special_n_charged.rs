use super::*;

unsafe extern "C" fn ssbexo_captain_neutral_special_charged_acmd(_agent: &mut L2CAgentBase) {}

unsafe extern "C" fn ssbexo_captain_neutral_special_charged_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("captain_fp_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        let fist = EffectModule::req_follow(boma, Hash40::new("sys_damage_fire"), Hash40::new("haver"), &Vector3f::zero(), &Vector3f::zero(), 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
        WorkModule::set_int(boma, fist as i32, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_SPECIAL_N_EFFECT_HANDLE);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn ssbexo_captain_neutral_special_charged_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_captain_boost_charge"));
        PLAY_SE(agent, Hash40::new("se_captain_special_n04"));
        PLAY_SE(agent, Hash40::new("vc_captain_boost"));
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        PLAY_LANDING_SE(agent, Hash40::new("se_captain_landing01"));
    }
}

unsafe extern "C" fn ssbexo_captain_neutral_special_charged_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 53.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(lua_state, 69.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("captain")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialncharged", ssbexo_captain_neutral_special_charged_acmd, Low)
    .acmd("effect_specialncharged", ssbexo_captain_neutral_special_charged_effect, Low)
    .acmd("sound_specialncharged", ssbexo_captain_neutral_special_charged_sound, Low)
    .acmd("expression_specialncharged", ssbexo_captain_neutral_special_charged_expression, Low)
    .install()
    ;
}