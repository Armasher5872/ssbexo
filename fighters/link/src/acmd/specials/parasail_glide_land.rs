use super::*;

//Glider Glide Land Effect
unsafe extern "C" fn ssbexo_link_parasail_glide_land_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("link_entry"), Hash40::new("trans"), 9.5, 3.6, 1.3, 0, 0, 0, 1, false);
        EFFECT_FOLLOW(agent, Hash40::new("link_entry"), Hash40::new("trans"), -9.5, 3.6, 1.3, 0, 0, 0, 1, false);
    }
}

pub fn install() {
    Agent::new("link_parasail")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .effect_acmd("effect_glideland", ssbexo_link_parasail_glide_land_effect, Low)
    .install()
    ;
}