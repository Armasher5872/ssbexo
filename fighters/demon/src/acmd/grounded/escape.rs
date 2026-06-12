use super::*;

//Escape ACMD
unsafe extern "C" fn ssbexo_demon_escape_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_MIST_STEP_ACTIVE);
    }
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_escape", ssbexo_demon_escape_acmd, Low)
    .install()
    ;
}