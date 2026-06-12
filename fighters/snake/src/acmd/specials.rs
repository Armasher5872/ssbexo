use super::*;

//Neutral Special Start ACMD
unsafe extern "C" fn ssbexo_snake_neutral_special_start_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        CORRECT(agent, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        CORRECT(agent, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, false, -1);
        ArticleModule::generate_article(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, false, -1);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
            ArticleModule::change_motion(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, Hash40::new("special_n_start"), false, -1.0);
        }
        else {
            ArticleModule::change_motion(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, Hash40::new("special_air_n_start"), false, -1.0);
        }
    }
    if ArticleModule::is_exist(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE) {
        ArticleModule::remove_exist(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        let itemmanager = smash2::app::ItemManager::instance().unwrap();
        let grenade_count = smash2::app::ItemManager::get_num_of_ownered_item(itemmanager, agent.battle_object_id, smash2::app::ItemKind::Snakegrenade);
        if grenade_count < 2 {
            ItemModule::have_item(boma, ItemKind(*ITEM_KIND_SNAKEGRENADE), 0, 0, false, false);
        }
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE(agent, 0.5);
}

pub fn install() {
    Agent::new("snake")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialnstart", ssbexo_snake_neutral_special_start_acmd, Low)
    .acmd("game_specialairnstart", ssbexo_snake_neutral_special_start_acmd, Low)
    .install()
    ;
}