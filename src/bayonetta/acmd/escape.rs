use super::*;

//Spotdodge ACMD
unsafe extern "C" fn ssbexo_bayonetta_spotdodge_acmd(_agent: &mut L2CAgentBase) {}

//Forward Roll ACMD
unsafe extern "C" fn ssbexo_bayonetta_forward_roll_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::REVERSE_LR(agent);
    }
}

//Backward Roll ACMD
unsafe extern "C" fn ssbexo_bayonetta_backward_roll_acmd(_agent: &mut L2CAgentBase) {}

//Neutral Airdodge ACMD
unsafe extern "C" fn ssbexo_bayonetta_neutral_airdodge_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

//Directional Airdodge ACMD
unsafe extern "C" fn ssbexo_bayonetta_directional_airdodge_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install() {
    Agent::new("bayonetta")
    .game_acmd("game_escapen", ssbexo_bayonetta_spotdodge_acmd)
    .game_acmd("game_escapef", ssbexo_bayonetta_forward_roll_acmd)
    .game_acmd("game_escapeb", ssbexo_bayonetta_backward_roll_acmd)
    .game_acmd("game_escapeair", ssbexo_bayonetta_neutral_airdodge_acmd)
    .game_acmd("game_escapeairslide", ssbexo_bayonetta_directional_airdodge_acmd)
    .install()
    ;
}