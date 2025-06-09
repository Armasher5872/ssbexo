use super::*;

//Neutral Special Start ACMD
unsafe extern "C" fn ssbexo_snake_neutral_special_start_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        CORRECT(agent, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
    }
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        CORRECT(agent, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, false, -1);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, false, -1);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, Hash40::new("special_n_start"), false, -1.0);
        }
        else {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, Hash40::new("special_air_n_start"), false, -1.0);
        }
    }
    if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        let itemmanager = smash2::app::ItemManager::instance().unwrap();
        let grenade_count = smash2::app::ItemManager::get_num_of_ownered_item(itemmanager, agent.battle_object_id, smash2::app::ItemKind::Snakegrenade);
        if grenade_count < 2 {
            ItemModule::have_item(agent.module_accessor, ItemKind(*ITEM_KIND_SNAKEGRENADE), 0, 0, false, false);
        }
    }
    frame(agent.lua_state_agent, 16.0);
    FT_MOTION_RATE(agent, 0.5);
}

pub fn install() {
    Agent::new("snake")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialnstart", ssbexo_snake_neutral_special_start_acmd, Low)
    .game_acmd("game_specialairnstart", ssbexo_snake_neutral_special_start_acmd, Low)
    .install()
    ;
}