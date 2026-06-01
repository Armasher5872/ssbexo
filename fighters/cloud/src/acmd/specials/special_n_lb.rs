use super::*;

//Limit Break Blade Beam ACMD
unsafe extern "C" fn ssbexo_cloud_limit_break_blade_beam_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        display_final_window(true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_CLOUD_GENERATE_ARTICLE_WAVE, false, -1);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x25813802b6));
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        display_final_window(false);
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialn_lb", ssbexo_cloud_limit_break_blade_beam_acmd, Low)
    .game_acmd("game_specialairn_lb", ssbexo_cloud_limit_break_blade_beam_acmd, Low)
    .install()
    ;
}