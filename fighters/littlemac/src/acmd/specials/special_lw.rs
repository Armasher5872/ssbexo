use super::*;

//Slip Counter ACMD
unsafe extern "C" fn ssbexo_littlemac_slip_counter_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_SHIELD);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_SHIELD);
    }
}

pub fn install() {
    Agent::new("littlemac")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_speciallw", ssbexo_littlemac_slip_counter_acmd, Low)
    .acmd("game_specialairlw", ssbexo_littlemac_slip_counter_acmd, Low)
    .install()
    ;
}