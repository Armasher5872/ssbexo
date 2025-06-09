use super::*;

//Dash ACMD
unsafe extern "C" fn ssbexo_gekkouga_dash_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
    }
}

//Backward Double Jump ACMD
unsafe extern "C" fn ssbexo_gekkouga_jump_aerial_b_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_BOUND);
    }
}

pub fn install() {
    Agent::new("gekkouga")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_dash", ssbexo_gekkouga_dash_acmd, Low)
    .game_acmd("game_jumpaerialb", ssbexo_gekkouga_jump_aerial_b_acmd, Low)
    .install()
    ;
}