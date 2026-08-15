use super::*;

//Side Taunt Effect
unsafe extern "C" fn ssbexo_ganon_side_taunt_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_final_hand_triforce"), Hash40::new("haver"), -1.1, -0.3, -0.2, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 0.3);
    }
    frame(lua_state, 100.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Side Taunt Sound
unsafe extern "C" fn ssbexo_ganon_side_taunt_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 7.0);
    if is_excute(agent) {
        let glow = SoundModule::play_se(boma, Hash40::new("se_ganon_appeal_s03"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(boma, glow as i32, 12.0, 0);
    }
    frame(lua_state, 95.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_appeal_s01"));
    }
}

//Side Taunt Expression
unsafe extern "C" fn ssbexo_ganon_side_taunt_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 100.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattack"), 18, true, *BATTLE_OBJECT_ID_INVALID as u32);
        AREA_WIND_2ND_arg10(agent, 0, 3, 110, 2, 0.6, 0, 15, 30, 30, 40);
    }
    frame(lua_state, 140.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("effect_appealsr", ssbexo_ganon_side_taunt_effect, Low)
    .acmd("sound_appealsr", ssbexo_ganon_side_taunt_sound, Low)
    .acmd("expression_appealsr", ssbexo_ganon_side_taunt_expression, Low)
    .acmd("effect_appealsl", ssbexo_ganon_side_taunt_effect, Low)
    .acmd("sound_appealsl", ssbexo_ganon_side_taunt_sound, Low)
    .acmd("expression_appealsl", ssbexo_ganon_side_taunt_expression, Low)
    .install()
    ;
}