use super::*;

//Megaflare ACMD
unsafe extern "C" fn ssbexo_edge_megaflare_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.4);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_EDGE_GENERATE_ARTICLE_FIRE, false, -1);
    }
    frame(lua_state, 33.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED);
    }
    frame(lua_state, 35.0);
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 60.0);
    FT_MOTION_RATE(agent, 1.0);
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialn2", ssbexo_edge_megaflare_acmd, Low)
    .acmd("game_specialairn2", ssbexo_edge_megaflare_acmd, Low)
    .install()
    ;
}