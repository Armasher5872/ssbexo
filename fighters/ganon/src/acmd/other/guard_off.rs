use super::*;

//Guard Off ACMD
unsafe extern "C" fn ssbexo_ganon_guard_off_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        HitModule::set_status_all(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

//Guard Off Sound
unsafe extern "C" fn ssbexo_ganon_guard_off_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_guardoff"));
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        let swipe = SoundModule::play_se(boma, Hash40::new("se_ganon_special_n07"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(boma, swipe as i32, 2.0, 0);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_guardoff", ssbexo_ganon_guard_off_acmd, Low)
    .acmd("sound_guardoff", ssbexo_ganon_guard_off_sound, Low)
    .install()
    ;
}