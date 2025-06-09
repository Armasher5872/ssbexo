use super::*;

//Aerial Jump Effect
unsafe extern "C" fn ssbexo_toonlink_aerial_jump_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_chicken_feather"), Hash40::new("top"), 0.0, 5.0, -2.0, 0.0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.5, 0.5, 3.0);
        LAST_EFFECT_SET_RATE(agent, 1.0);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_chicken_feather"), Hash40::new("top"), 0.0, 4.0, 1.5, 0.0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 0.7, 0.7, 3.0);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
}

pub fn install() {
    Agent::new("toonlink")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .effect_acmd("effect_jumpaerialfront", ssbexo_toonlink_aerial_jump_effect, Low)
    .effect_acmd("effect_jumpaerialback", ssbexo_toonlink_aerial_jump_effect, Low)
    .install()
    ;
}