use super::*;

//Aerial Jump Forward ACMD
unsafe extern "C" fn ssbexo_yoshi_aerial_jump_forward_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    MotionModule::set_rate(agent.module_accessor, 0.75);
    frame(agent.lua_state_agent, 26.0);
    MotionModule::set_rate(agent.module_accessor, 1.0);
}

//Aerial Jump Backward ACMD
unsafe extern "C" fn ssbexo_yoshi_aerial_jump_backward_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    MotionModule::set_rate(agent.module_accessor, 0.75);
    frame(agent.lua_state_agent, 22.0);
    MotionModule::set_rate(agent.module_accessor, 1.0);
}

pub fn install() {
    Agent::new("yoshi")
    .game_acmd("game_jumpaerialfront", ssbexo_yoshi_aerial_jump_forward_acmd, Priority::Low)
    .game_acmd("game_jumpaerialback", ssbexo_yoshi_aerial_jump_backward_acmd, Priority::Low)
    .install()
    ;
}