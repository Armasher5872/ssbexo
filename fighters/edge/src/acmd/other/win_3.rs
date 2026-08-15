use super::*;

//Win 3 ACMD
unsafe extern "C" fn ssbexo_edge_win_3_acmd(_agent: &mut L2CAgentBase) {}

//Win 3 Effect
unsafe extern "C" fn ssbexo_edge_win_3_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("edge_win_fire"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("edge_win_sprks"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("edge_win_sprks_b"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("edge_win_sprks2"), true, true);
        EFFECT_OFF_KIND(agent, Hash40::new("edge_win_sprks3"), true, true);
        EFFECT(agent, Hash40::new("edge_win_fire"), Hash40::new("top"), 0, 1.5, -35, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("edge_win_sprks_b"), Hash40::new("top"), 0, 1.5, -50, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 123.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("edge_win_burst2"), Hash40::new("top"), 0, 1.5, -35, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 132.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("edge_win_sprks3"), Hash40::new("top"), 0, 5, -50, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//Win 3 Sound
unsafe extern "C" fn ssbexo_edge_win_3_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_edge_win01_02"));
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_attackair_b02"));
    }
    frame(lua_state, 47.0);
    if is_excute(agent) {
        let swing = SoundModule::play_se(boma, Hash40::new("se_edge_attackair_f02"), false, false, false, false, enSEType(0));
        SoundModule::set_se_vol(boma, swing as i32, 1.5, 0);
    }
    frame(lua_state, 62.0);
    if is_excute(agent) {
        let swing = SoundModule::play_se(boma, Hash40::new("se_edge_smash_l02"), false, false, false, false, enSEType(0));
        SoundModule::set_se_vol(boma, swing as i32, 4.0, 0);
    }
    frame(lua_state, 119.0);
    if is_excute(agent) {
        let swing = SoundModule::play_se(boma, Hash40::new("se_edge_win03_03"), false, false, false, false, enSEType(0));
        SoundModule::set_se_vol(boma, swing as i32, 2.0, 0);
    }
    frame(lua_state, 123.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_win03_01"));
    }
}

//Win 3 Sub Sound
unsafe extern "C" fn ssbexo_edge_win_3_sub_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 25.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_edge_win03"));
    }
}

//Win 3 Expression
unsafe extern "C" fn ssbexo_edge_win_3_expression(_agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_win3", ssbexo_edge_win_3_acmd, Low)
    .acmd("effect_win3", ssbexo_edge_win_3_effect, Low)
    .acmd("sound_win3", ssbexo_edge_win_3_sound, Low)
    .acmd("sound_win3a", ssbexo_edge_win_3_sub_sound, Low)
    .acmd("sound_win3b", ssbexo_edge_win_3_sub_sound, Low)
    .acmd("expression_win3", ssbexo_edge_win_3_expression, Low)
    .install()
    ;
}