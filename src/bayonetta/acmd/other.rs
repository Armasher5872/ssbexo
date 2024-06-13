use super::*;

//Landing Light Effect
unsafe extern "C" fn ssbexo_bayonetta_landing_light_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("bayonetta_butterfly_landing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Landing Heavy Effect
unsafe extern "C" fn ssbexo_bayonetta_landing_heavy_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("bayonetta_butterfly_landing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Landing Fall Special Effect
unsafe extern "C" fn ssbexo_bayonetta_landing_fall_special_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("bayonetta_butterfly_landing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    Agent::new("bayonetta")
    .effect_acmd("effect_landinglight", ssbexo_bayonetta_landing_light_effect, Priority::Low)
    .effect_acmd("effect_landingheavy", ssbexo_bayonetta_landing_heavy_effect, Priority::Low)
    .effect_acmd("effect_landingfallspecial", ssbexo_bayonetta_landing_fall_special_effect, Priority::Low)
    .install()
    ;
}