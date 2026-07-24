use super::*;

//Neutral Special Start ACMD
unsafe extern "C" fn ssbexo_edge_neutral_special_start_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 12.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, *FIGHTER_EDGE_SPECIAL_N_S, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, *FIGHTER_EDGE_SPECIAL_N_M, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(lua_state, 79.0);
    FT_MOTION_RATE(agent, 1.2);
    frame(lua_state, 99.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 100.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, *FIGHTER_EDGE_SPECIAL_N_L, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(lua_state, 105.0);
    FT_MOTION_RATE(agent, 1.6);
    frame(lua_state, 115.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 120.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_EDGE_GENERATE_ARTICLE_FIRE, false, -1);
    }
    frame(lua_state, 140.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED);
    }
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialnstart", ssbexo_edge_neutral_special_start_acmd, Low)
    .acmd("game_specialairnstart", ssbexo_edge_neutral_special_start_acmd, Low)
    .install()
    ;
}