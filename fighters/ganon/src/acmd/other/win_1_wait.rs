use super::*;

//Win 1 Wait Effect
unsafe extern "C" fn ssbexo_ganon_win_1_wait_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_final_hand_triforce"), Hash40::new("haver"), -1.1, -0.3, -0.2, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 0.2);
    }
}

//Win 1 Wait Sound
unsafe extern "C" fn ssbexo_ganon_win_1_wait_sound(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_ganon_appeal_s03"));
        let glow = SoundModule::play_se(boma, Hash40::new("se_ganon_appeal_s03"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(boma, glow as i32, 12.0, 0);
        LAST_EFFECT_SET_RATE(agent, 0.2);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("effect_win1wait", ssbexo_ganon_win_1_wait_effect, Low)
    .acmd("sound_win1wait", ssbexo_ganon_win_1_wait_sound, Low)
    .install()
    ;
}