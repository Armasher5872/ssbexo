use super::*;

//Mega Lucario Turn ACMD
unsafe extern "C" fn ssbexo_lucariom_turn_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        PostureModule::reverse_lr(agent.module_accessor);
		PostureModule::update_rot_y_lr(agent.module_accessor);
    }
}

//Mega Lucario Turn Dash ACMD
unsafe extern "C" fn ssbexo_lucariom_turn_dash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        PostureModule::reverse_lr(agent.module_accessor);
		PostureModule::update_rot_y_lr(agent.module_accessor);
    }
}

//Mega Lucario Turn Run ACMD
unsafe extern "C" fn ssbexo_lucariom_turn_run_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        PostureModule::reverse_lr(agent.module_accessor);
		PostureModule::update_rot_y_lr(agent.module_accessor);
    }
}

//Mega Lucario Forward Roll ACMD
unsafe extern "C" fn ssbexo_lucariom_forward_roll_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        PostureModule::reverse_lr(agent.module_accessor);
		PostureModule::update_rot_y_lr(agent.module_accessor);
    }
}

//Mega Lucario Back Roll ACMD
unsafe extern "C" fn ssbexo_lucariom_back_roll_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        PostureModule::reverse_lr(agent.module_accessor);
		PostureModule::update_rot_y_lr(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("lucario_lucariom")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_turn", ssbexo_lucariom_turn_acmd, Low)
    .game_acmd("game_turndash", ssbexo_lucariom_turn_dash_acmd, Low)
    .game_acmd("game_turnrun", ssbexo_lucariom_turn_run_acmd, Low)
    .game_acmd("game_escapef", ssbexo_lucariom_forward_roll_acmd, Low)
    .game_acmd("game_escapeb", ssbexo_lucariom_back_roll_acmd, Low)
    .install()
    ;
}