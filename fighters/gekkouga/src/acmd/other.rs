use super::*;

//Dash ACMD
unsafe extern "C" fn ssbexo_gekkouga_dash_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

//Jump Aerial Back ACMD
unsafe extern "C" fn ssbexo_gekkouga_jump_aerial_back_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 30.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_BOUND);
    }
}

//Airdodge ACMD
unsafe extern "C" fn ssbexo_gekkouga_airdodge_acmd(_agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("gekkouga")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_dash", ssbexo_gekkouga_dash_acmd, Low)
    .acmd("game_jumpaerialback", ssbexo_gekkouga_jump_aerial_back_acmd, Low)
    .acmd("game_escapeair", ssbexo_gekkouga_airdodge_acmd, Low)
    .acmd("game_escapeairslide", ssbexo_gekkouga_airdodge_acmd, Low)
    .install()
    ;
}