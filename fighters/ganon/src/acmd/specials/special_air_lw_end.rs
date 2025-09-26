use super::*;

//Aerial Down Special Landing Effect
unsafe extern "C" fn ssbexo_ganon_aerial_down_special_landing_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("ganon_ground_crack"), Hash40::new("top"), -2, 0, 4, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 0.0);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .effect_acmd("effect_specialairlwend", ssbexo_ganon_aerial_down_special_landing_effect, Low)
    .install()
    ;
}