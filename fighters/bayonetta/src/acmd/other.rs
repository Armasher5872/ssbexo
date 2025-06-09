use super::*;

//Landing Light Effect
unsafe extern "C" fn ssbexo_bayonetta_landing_light_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("bayonetta_butterfly_landing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Landing Heavy Effect
unsafe extern "C" fn ssbexo_bayonetta_landing_heavy_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("bayonetta_butterfly_landing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Landing Fall Special Effect
unsafe extern "C" fn ssbexo_bayonetta_landing_fall_special_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("bayonetta_butterfly_landing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    Agent::new("bayonetta")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .effect_acmd("effect_landinglight", ssbexo_bayonetta_landing_light_effect, Low)
    .effect_acmd("effect_landingheavy", ssbexo_bayonetta_landing_heavy_effect, Low)
    .effect_acmd("effect_landingfallspecial", ssbexo_bayonetta_landing_fall_special_effect, Low)
    .install()
    ;
}