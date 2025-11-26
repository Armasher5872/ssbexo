use super::*;

//Aerial Down Special End ACMD
unsafe extern "C" fn ssbexo_wario_aerial_down_special_end_acmd(_agent: &mut L2CAgentBase) {}

//Aerial Down Special End Effect
unsafe extern "C" fn ssbexo_wario_aerial_down_special_end_effect(_agent: &mut L2CAgentBase) {}

//Aerial Down Special End Sound
unsafe extern "C" fn ssbexo_wario_aerial_down_special_end_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_swing_s"));
    }
}

//Aerial Down Special End Expression
unsafe extern "C" fn ssbexo_wario_aerial_down_special_end_expression(_agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialairlwend", ssbexo_wario_aerial_down_special_end_acmd, Low)
    .effect_acmd("effect_specialairlwend", ssbexo_wario_aerial_down_special_end_effect, Low)
    .sound_acmd("sound_specialairlwend", ssbexo_wario_aerial_down_special_end_sound, Low)
    .expression_acmd("expression_specialairlwend", ssbexo_wario_aerial_down_special_end_expression, Low)
    .install()
    ;
}