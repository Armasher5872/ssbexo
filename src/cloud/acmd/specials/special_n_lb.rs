use super::*;

//Limit Break Blade Beam ACMD
unsafe extern "C" fn ssbexo_cloud_limit_break_blade_beam_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        display_final_window(true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_CLOUD_GENERATE_ARTICLE_WAVE, false, -1);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x25813802b6));
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        display_final_window(false);
    }
}

pub fn install() {
    Agent::new("cloud")
    .game_acmd("game_specialn_lb", ssbexo_cloud_limit_break_blade_beam_acmd, Priority::Low)
    .game_acmd("game_specialairn_lb", ssbexo_cloud_limit_break_blade_beam_acmd, Priority::Low)
    .install()
    ;
}