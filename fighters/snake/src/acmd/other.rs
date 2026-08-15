use super::*;

unsafe extern "C" fn ssbexo_snake_entry_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x1f20a9d549), false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x24772eddef), false);
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ENTRY {
            WorkModule::on_flag(boma, *FIGHTER_SNAKE_STATUS_ENTRY_FLAG_SPYCLOAK);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ENTRY {
            WorkModule::on_flag(boma, *FIGHTER_SNAKE_STATUS_ENTRY_FLAG_SPYCLOAK);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ENTRY {
            WorkModule::on_flag(boma, *FIGHTER_SNAKE_STATUS_ENTRY_FLAG_SPYCLOAK);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ENTRY {
            WorkModule::on_flag(boma, *FIGHTER_SNAKE_STATUS_ENTRY_FLAG_SPYCLOAK);
        }
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x1f20a9d549), true);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x24772eddef), true);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ENTRY {
            WorkModule::on_flag(boma, *FIGHTER_SNAKE_STATUS_ENTRY_FLAG_SPYCLOAK);
        }
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ENTRY {
            WorkModule::on_flag(boma, *FIGHTER_SNAKE_STATUS_ENTRY_FLAG_SPYCLOAK);
        }
    }
    frame(lua_state, 56.0);
    if is_excute(agent) {
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ENTRY {
            WorkModule::on_flag(boma, *FIGHTER_SNAKE_STATUS_ENTRY_FLAG_SPYCLOAK);
        }
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ENTRY {
            WorkModule::on_flag(boma, *FIGHTER_SNAKE_STATUS_ENTRY_FLAG_SPYCLOAK);
        }
    }
}

//Airdodge ACMD
unsafe extern "C" fn ssbexo_snake_airdodge_acmd(_agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("snake")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("expression_entryr", ssbexo_snake_entry_expression, Low)
    .acmd("expression_entryl", ssbexo_snake_entry_expression, Low)
    .acmd("game_escapeair", ssbexo_snake_airdodge_acmd, Low)
    .acmd("game_escapeairslide", ssbexo_snake_airdodge_acmd, Low)
    .install()
    ;
}