use super::*;

//Side Taunt ACMD
unsafe extern "C" fn ssbexo_edge_side_taunt_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 54.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_APPEAL_ENABLE_LOOP);
        WorkModule::set_float(boma, 68.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_APPEAL_RESET_FRAME);
    }
    frame(lua_state, 68.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_APPEAL_ENABLE_LOOP);
    }
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_appealsl", ssbexo_edge_side_taunt_acmd, Low)
    .acmd("game_appealsr", ssbexo_edge_side_taunt_acmd, Low)
    .install()
    ;
}