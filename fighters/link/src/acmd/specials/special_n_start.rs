use super::*;

//Neutral Special Start Sound
unsafe extern "C" fn ssbexo_link_special_n_start_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 13.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_link_special_n01"));
    }
    frame(lua_state, 50.0);
    if ArticleModule::is_exist(boma, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW) {
        let bow_arrow_boma = get_article_boma(boma, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW);
        let arrow_type = WorkModule::get_int(bow_arrow_boma, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
        if arrow_type == *WN_LINK_BOWARROW_LIGHT_ARROW {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("se_link_special_n09"));
            }
        }
    }
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .sound_acmd("sound_specialnstart", ssbexo_link_special_n_start_sound, Low)
    .sound_acmd("sound_specialairnstart", ssbexo_link_special_n_start_sound, Low)
    .install()
    ;
}