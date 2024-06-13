use super::*;

unsafe extern "C" fn ssbexo_littlemac_dash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

pub fn install() {
    Agent::new("littlemac")
    .game_acmd("game_dash", ssbexo_littlemac_dash_acmd, Priority::Low)
    .install()
    ;
}