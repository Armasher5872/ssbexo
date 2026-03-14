use super::*;

//Slip Counter ACMD
unsafe extern "C" fn ssbexo_littlemac_slip_counter_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_SHIELD);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_SHIELD);
    }
}

pub fn install() {
    Agent::new("littlemac")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallw", ssbexo_littlemac_slip_counter_acmd, Low)
    .game_acmd("game_specialairlw", ssbexo_littlemac_slip_counter_acmd, Low)
    .install()
    ;
}