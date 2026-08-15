use super::*;

//Win 1 Effect
unsafe extern "C" fn ssbexo_ganon_win_1_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 110.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_final_hand_triforce"), Hash40::new("haver"), -1.1, -0.3, -0.2, 0, 0, 0, 1, true);
    }
}

//Win 1 Sound
unsafe extern "C" fn ssbexo_ganon_win_1_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 35.0);
    if is_excute(agent) {
        PLAY_SE_NO_3D(agent, Hash40::new("vc_ganon_win01"));
    }
    frame(lua_state, 51.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_swing_l"));
    }
    frame(lua_state, 110.0);
    if is_excute(agent) {
        let glow = SoundModule::play_se(boma, Hash40::new("se_ganon_appeal_s03"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(boma, glow as i32, 12.0, 0);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("effect_win1", ssbexo_ganon_win_1_effect, Low)
    .acmd("sound_win1", ssbexo_ganon_win_1_sound, Low)
    .install()
    ;
}