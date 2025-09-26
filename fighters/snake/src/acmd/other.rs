use super::*;

unsafe extern "C" fn ssbexo_snake_entry_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x1f20a9d549), false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x24772eddef), false);
        if StatusModule::status_kind(agent.module_accessor) == *FIGHTER_STATUS_KIND_ENTRY {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_SNAKE_STATUS_ENTRY_FLAG_SPYCLOAK);
        }
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        if StatusModule::status_kind(agent.module_accessor) == *FIGHTER_STATUS_KIND_ENTRY {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_SNAKE_STATUS_ENTRY_FLAG_SPYCLOAK);
        }
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        if StatusModule::status_kind(agent.module_accessor) == *FIGHTER_STATUS_KIND_ENTRY {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_SNAKE_STATUS_ENTRY_FLAG_SPYCLOAK);
        }
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        if StatusModule::status_kind(agent.module_accessor) == *FIGHTER_STATUS_KIND_ENTRY {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_SNAKE_STATUS_ENTRY_FLAG_SPYCLOAK);
        }
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x1f20a9d549), true);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x24772eddef), true);
    }
    frame(agent.lua_state_agent, 38.0);
    if is_excute(agent) {
        if StatusModule::status_kind(agent.module_accessor) == *FIGHTER_STATUS_KIND_ENTRY {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_SNAKE_STATUS_ENTRY_FLAG_SPYCLOAK);
        }
    }
    frame(agent.lua_state_agent, 44.0);
    if is_excute(agent) {
        if StatusModule::status_kind(agent.module_accessor) == *FIGHTER_STATUS_KIND_ENTRY {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_SNAKE_STATUS_ENTRY_FLAG_SPYCLOAK);
        }
    }
    frame(agent.lua_state_agent, 56.0);
    if is_excute(agent) {
        if StatusModule::status_kind(agent.module_accessor) == *FIGHTER_STATUS_KIND_ENTRY {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_SNAKE_STATUS_ENTRY_FLAG_SPYCLOAK);
        }
    }
    frame(agent.lua_state_agent, 60.0);
    if is_excute(agent) {
        if StatusModule::status_kind(agent.module_accessor) == *FIGHTER_STATUS_KIND_ENTRY {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_SNAKE_STATUS_ENTRY_FLAG_SPYCLOAK);
        }
    }
}

pub fn install() {
    Agent::new("snake")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .expression_acmd("expression_entryr", ssbexo_snake_entry_expression, Low)
    .expression_acmd("expression_entryl", ssbexo_snake_entry_expression, Low)
    .install()
    ;
}