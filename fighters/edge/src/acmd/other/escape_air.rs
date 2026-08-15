use super::*;

//Airdodge ACMD
unsafe extern "C" fn ssbexo_edge_escape_air_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ESCAPE_AIR_ENABLE_WING);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ESCAPE_AIR_ENABLE_WING);
    }
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_escapeair", ssbexo_edge_escape_air_acmd, Low)
    .acmd("game_escapeairslide", ssbexo_edge_escape_air_acmd, Low)
    .install()
    ;
}