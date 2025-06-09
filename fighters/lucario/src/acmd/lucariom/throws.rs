use super::*;

//Mega Lucario Pivot Grab ACMD
unsafe extern "C" fn ssbexo_lucariom_pivot_grab_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        PostureModule::reverse_lr(agent.module_accessor);
		PostureModule::update_rot_y_lr(agent.module_accessor);
    }
}

//Mega Lucario Back Throw ACMD
unsafe extern "C" fn ssbexo_lucariom_back_throw_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        PostureModule::reverse_lr(agent.module_accessor);
		PostureModule::update_rot_y_lr(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("lucario_lucariom")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_catchturn", ssbexo_lucariom_pivot_grab_acmd, Low)
    .game_acmd("game_throwb", ssbexo_lucariom_back_throw_acmd, Low)
    .install()
    ;
}